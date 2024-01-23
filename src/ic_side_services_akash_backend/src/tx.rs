use std::str::FromStr;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use cosmrs::{
    crypto::PublicKey,
    proto::cosmos::tx::v1beta1::TxRaw,
    tendermint::chain::Id,
    tx::{self, Fee, Msg, SignDoc, SignerInfo},
    AccountId, Any, Coin, Denom, ErrorReport, Tx,
};
use ic_cdk::{
    api::management_canister::ecdsa::{sign_with_ecdsa, SignWithEcdsaArgument},
    print,
};

use crate::{
    address::get_account_id_from_public_key,
    proto::{
        self,
        deployment::{deployment::DeploymentID, deploymentmsg::MsgCreateDeployment},
    },
    sdl::SdlV3,
    sha256, EcdsaKeyIds,
};

/// from https://docs.rs/cosmrs/latest/cosmrs/tx/index.html#usage
///
/// Transaction data created in the Keplr wallet on Sandbox network
/// (500000 uakt = 0.5 AKT)
/// ```
/// {
///   "txBody": {
///     "messages": [
///       {
///         "typeUrl": "/cosmos.bank.v1beta1.MsgSend",
///         "value": {
///           "fromAddress": "akash13gtrvjrzx8tst260ucszcflt4wny68shwdmrxs",
///           "toAddress": "akash1c5fnkfqq5yn7femz960m70w0ea4j2urayddhal",
///           "amount": [
///             {
///               "denom": "uakt",
///               "amount": "500000"
///             }
///           ]
///         }
///       }
///     ],
///     "memo": "",
///     "timeoutHeight": "0",
///     "extensionOptions": [],
///     "nonCriticalExtensionOptions": []
///   },
///   "authInfo": {
///     "signerInfos": [
///       {
///         "publicKey": {
///           "typeUrl": "/cosmos.crypto.secp256k1.PubKey",
///           "value": "CiEDwfW3Ts+1EB9KFNMDCRhG/J+OHlkFwkWq/JaIYEtu3o8="
///         },
///         "modeInfo": {
///           "single": {
///             "mode": "SIGN_MODE_DIRECT"
///           }
///         },
///         "sequence": "18"
///       }
///     ],
///     "fee": {
///       "amount": [
///         {
///           "denom": "uakt",
///           "amount": "2111"
///         }
///       ],
///       "gasLimit": "84413",
///       "payer": "",
///       "granter": ""
///     }
///   },
///   "chainId": "sandbox-01",
///   "accountNumber": "259"
/// }
/// ```
pub async fn create_tx(
    sender_public_key: &PublicKey,
    msg: Any,
    fee: Fee,
    sequence_number: u64,
) -> Result<String, String> {
    // Transaction metadata: chain, account, sequence, gas, fee, timeout, and memo.
    // from:
    // - sandbox: https://raw.githubusercontent.com/akash-network/net/main/sandbox/chain-id.txt
    // - mainnet: https://raw.githubusercontent.com/akash-network/net/main/mainnet/chain-id.txt
    //
    // more config params from: https://github.com/akash-network/net/blob/main/sandbox/meta.json
    // see also: https://docs.akash.network/guides/sandbox/detailed-steps/part-4.-configure-your-network
    let chain_id = Id::from_str("sandbox-01").map_err(|e| e.to_string())?;
    let account_number = 263; // TODO: figure out how to obtain this
    let timeout_height = 0u16;
    let memo = "created from canister";

    // Create transaction body from the MsgSend, memo, and timeout height.
    let tx_body = tx::Body::new(vec![msg], memo, timeout_height);

    // print(format!("tx_body: {:?}", tx_body));

    // Create signer info from public key and sequence number.
    // This uses a standard "direct" signature from a single signer.
    let signer_info = SignerInfo::single_direct(Some(*sender_public_key), sequence_number);

    // Compute auth info from signer info by associating a fee.
    let auth_info = signer_info.auth_info(fee);

    //////////////////////////
    // Signing transactions //
    //////////////////////////

    // The "sign doc" contains a message to be signed.
    let sign_doc =
        SignDoc::new(&tx_body, &auth_info, &chain_id, account_number).map_err(|e| e.to_string())?;

    // Sign the "sign doc" with the sender's private key, producing a signed raw transaction.
    let tx_signed = sign_tx(sign_doc).await?;

    // Serialize the raw transaction as bytes (i.e. `Vec<u8>`).
    let tx_bytes = tx_signed.to_bytes().map_err(|e| e.to_string())?;

    //////////////////////////
    // Parsing transactions //
    //////////////////////////

    // Parse the serialized bytes from above into a `cosmrs::Tx`
    let tx_parsed = Tx::from_bytes(&tx_bytes).map_err(|e| e.to_string())?;
    assert_eq!(tx_parsed.body, tx_body);
    assert_eq!(tx_parsed.auth_info, auth_info);

    // print(format!("tx_parsed: {:?}", tx_parsed));

    Ok(hex::encode(&tx_bytes))
}

/// adapted form https://docs.rs/cosmrs/latest/cosmrs/tx/struct.SignDoc.html#method.sign
async fn sign_tx(sign_doc: SignDoc) -> Result<tx::Raw, String> {
    let sign_doc_bytes = sign_doc.clone().into_bytes().map_err(|e| e.to_string())?;
    let hash = sha256(&sign_doc_bytes);

    let request = SignWithEcdsaArgument {
        message_hash: hash.to_vec(),
        derivation_path: vec![],
        key_id: EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id(),
    };

    let (response,) = sign_with_ecdsa(request)
        .await
        .map_err(|e| format!("sign_with_ecdsa failed {}", e.1))?;

    Ok(TxRaw {
        body_bytes: sign_doc.body_bytes,
        auth_info_bytes: sign_doc.auth_info_bytes,
        signatures: vec![response.signature],
    }
    .into())
}

/// MsgCreateCertificate defines an SDK message for creating certificate.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgCreateCertificate {
    pub owner: AccountId,
    pub cert: Vec<u8>,
    pub pubkey: Vec<u8>,
}

impl Msg for MsgCreateCertificate {
    type Proto = proto::cert::cert::MsgCreateCertificate;
}

impl TryFrom<proto::cert::cert::MsgCreateCertificate> for MsgCreateCertificate {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cert::cert::MsgCreateCertificate,
    ) -> Result<MsgCreateCertificate, Self::Error> {
        MsgCreateCertificate::try_from(&proto)
    }
}

impl TryFrom<&proto::cert::cert::MsgCreateCertificate> for MsgCreateCertificate {
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cert::cert::MsgCreateCertificate,
    ) -> Result<MsgCreateCertificate, Self::Error> {
        Ok(MsgCreateCertificate {
            owner: proto.owner.parse()?,
            cert: proto.cert.clone(),
            pubkey: proto.pubkey.clone(),
        })
    }
}

impl From<MsgCreateCertificate> for proto::cert::cert::MsgCreateCertificate {
    fn from(msg: MsgCreateCertificate) -> proto::cert::cert::MsgCreateCertificate {
        proto::cert::cert::MsgCreateCertificate::from(&msg)
    }
}

impl From<&MsgCreateCertificate> for proto::cert::cert::MsgCreateCertificate {
    fn from(msg: &MsgCreateCertificate) -> proto::cert::cert::MsgCreateCertificate {
        proto::cert::cert::MsgCreateCertificate {
            owner: msg.owner.to_string(),
            cert: msg.cert.clone(),
            pubkey: msg.pubkey.clone(),
        }
    }
}

pub async fn create_certificate_tx(
    sender_public_key: &PublicKey,
    cert_pem_base64: String,
    pub_key_pem_base64: String,
) -> Result<String, String> {
    let msg = MsgCreateCertificate {
        owner: get_account_id_from_public_key(sender_public_key).unwrap(),
        cert: STANDARD.decode(cert_pem_base64).unwrap(),
        pubkey: STANDARD.decode(pub_key_pem_base64).unwrap(),
    };

    let amount = Coin {
        amount: 3_000u128,
        denom: Denom::from_str("uakt").unwrap(),
    };

    let gas = 100_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);
    let sequence_number = 1;

    create_tx(
        &sender_public_key,
        msg.to_any().unwrap(),
        fee,
        sequence_number,
    )
    .await
}

pub async fn create_deployment_tx(sender_public_key: &PublicKey) -> Result<String, String> {
    // hash of this deployment (base64): TGNKUw/ffyyB/d0EaY9FWMEIhsBzcjY3PLBRHYDqszs=
    // see https://deploy.cloudmos.io/transactions/268DEE51F9FAB84B1BABCD916092D380784A483EA088345CF7B86657BBC8A4DA?network=sandbox
    let sdl_str = r#"
version: "3.0"
services:
  ic-websocket-gateway:
    image: omniadevs/ic-websocket-gateway
    expose:
      - port: 8080
        as: 80
        accept:
          - "akash-gateway.icws.io"
        to:
          - global: true
    command:
      - "/ic-ws-gateway/ic_websocket_gateway"
      - "--gateway-address"
      - "0.0.0.0:8080"
      - "--ic-network-url"
      - "https://icp-api.io"
      - "--polling-interval"
      - "400"
profiles:
  compute:
    ic-websocket-gateway:
      resources:
        cpu:
          units: 0.5
        memory:
          size: 512Mi
        storage:
          - size: 512Mi
        gpu:
          units: 0
  placement:
    dcloud:
      pricing:
        ic-websocket-gateway:
          denom: uakt
          amount: 1000
deployment:
  ic-websocket-gateway:
    dcloud:
      profile: ic-websocket-gateway
      count: 1
    "#;

    let sdl = SdlV3::try_from_str(sdl_str).unwrap();
    print(format!("sdl: {:?}", sdl));
    print(format!(
        "sdl manifest sorted: {}",
        sdl.manifest_sorted_json()
    ));
    print(format!(
        "sdl version (base64): {}",
        STANDARD.encode(sdl.manifest_version())
    ));

    // see https://github.com/akash-network/cloudmos/blob/8a8098b7e371e801dad3aad81ef92b8dfe387e4c/deploy-web/src/utils/deploymentData/v1beta3.ts#L230
    let msg = MsgCreateDeployment {
        id: Some(DeploymentID {
            owner: get_account_id_from_public_key(sender_public_key)
                .unwrap()
                .to_string(),
            // see https://github.com/akash-network/cloudmos/blob/8a8098b7e371e801dad3aad81ef92b8dfe387e4c/deploy-web/src/utils/deploymentData/v1beta3.ts#L248C27-L248C27
            dseq: 2893944, // obtained from /blocks/latest RPC call
        }),
        groups: sdl.groups(),
        version: sdl.manifest_version(),
        deposit: Some(
            Coin {
                amount: 5_000_000u128,
                denom: Denom::from_str("uakt").unwrap(),
            }
            .into(),
        ),
        depositor: get_account_id_from_public_key(sender_public_key)
            .unwrap()
            .to_string(),
    };

    let amount = Coin {
        amount: 3_000u128,
        denom: Denom::from_str("uakt").unwrap(),
    };

    let gas = 100_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);
    let sequence_number = 1;

    create_tx(
        &sender_public_key,
        Any::from_msg(&msg).unwrap(),
        fee,
        sequence_number,
    )
    .await
}
