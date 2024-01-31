use std::str::FromStr;

use cosmrs::{
    crypto::PublicKey,
    proto::cosmos::tx::v1beta1::TxRaw,
    tendermint::chain::Id,
    tx::{self, Fee, SignDoc, SignerInfo},
    Any, Tx,
};

use crate::{
    ecdsa::{self},
    hash::sha256,
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
    account_number: u64,
) -> Result<Vec<u8>, String> {
    // Transaction metadata: chain, account, sequence, gas, fee, timeout, and memo.
    // from:
    // - sandbox: https://raw.githubusercontent.com/akash-network/net/main/sandbox/chain-id.txt
    // - mainnet: https://raw.githubusercontent.com/akash-network/net/main/mainnet/chain-id.txt
    //
    // more config params from: https://github.com/akash-network/net/blob/main/sandbox/meta.json
    // see also: https://docs.akash.network/guides/sandbox/detailed-steps/part-4.-configure-your-network
    let chain_id = Id::from_str("sandbox-01").map_err(|e| e.to_string())?;
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

    Ok(tx_bytes)
}

/// adapted form https://docs.rs/cosmrs/latest/cosmrs/tx/struct.SignDoc.html#method.sign
async fn sign_tx(sign_doc: SignDoc) -> Result<tx::Raw, String> {
    let sign_doc_bytes = sign_doc.clone().into_bytes().map_err(|e| e.to_string())?;
    let hash = sha256(&sign_doc_bytes);

    let signature = ecdsa::sign(hash.to_vec()).await.unwrap();

    Ok(TxRaw {
        body_bytes: sign_doc.body_bytes,
        auth_info_bytes: sign_doc.auth_info_bytes,
        signatures: vec![signature],
    }
    .into())
}
