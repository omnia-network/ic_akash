use std::str::FromStr;

use cosmrs::{
    auth::BaseAccount,
    crypto::PublicKey,
    tx::{Fee, Msg},
    AccountId, Coin, Denom, ErrorReport,
};

use crate::helpers::EcdsaKeyIds;

use super::{
    address::get_account_id_from_public_key,
    proto::{self},
    tx::create_tx,
};

/// MsgCreateCertificate defines an SDK message for creating certificate.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgCreateCertificate {
    pub owner: AccountId,
    pub cert: Vec<u8>,
    pub pubkey: Vec<u8>,
}

impl Msg for MsgCreateCertificate {
    type Proto = proto::cert::MsgCreateCertificate;
}

impl TryFrom<proto::cert::MsgCreateCertificate> for MsgCreateCertificate {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::cert::MsgCreateCertificate,
    ) -> Result<MsgCreateCertificate, Self::Error> {
        MsgCreateCertificate::try_from(&proto)
    }
}

impl TryFrom<&proto::cert::MsgCreateCertificate> for MsgCreateCertificate {
    type Error = ErrorReport;

    fn try_from(
        proto: &proto::cert::MsgCreateCertificate,
    ) -> Result<MsgCreateCertificate, Self::Error> {
        Ok(MsgCreateCertificate {
            owner: proto.owner.parse()?,
            cert: proto.cert.clone(),
            pubkey: proto.pubkey.clone(),
        })
    }
}

impl From<MsgCreateCertificate> for proto::cert::MsgCreateCertificate {
    fn from(msg: MsgCreateCertificate) -> proto::cert::MsgCreateCertificate {
        proto::cert::MsgCreateCertificate::from(&msg)
    }
}

impl From<&MsgCreateCertificate> for proto::cert::MsgCreateCertificate {
    fn from(msg: &MsgCreateCertificate) -> proto::cert::MsgCreateCertificate {
        proto::cert::MsgCreateCertificate {
            owner: msg.owner.to_string(),
            cert: msg.cert.clone(),
            pubkey: msg.pubkey.clone(),
        }
    }
}

pub async fn create_certificate_tx(
    sender_public_key: &PublicKey,
    cert_pem: Vec<u8>,
    pub_key_pem: Vec<u8>,
    account: &BaseAccount,
    ecdsa_key: &EcdsaKeyIds,
) -> Result<Vec<u8>, String> {
    let msg = MsgCreateCertificate {
        owner: get_account_id_from_public_key(sender_public_key)?,
        cert: cert_pem,
        pubkey: pub_key_pem,
    };

    let amount = Coin {
        amount: 3_000u128,
        denom: Denom::from_str("uakt").unwrap(),
    };

    let gas = 100_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);

    create_tx(
        sender_public_key,
        msg.to_any().unwrap(),
        fee,
        account.sequence,
        account.account_number,
        ecdsa_key,
    )
    .await
}
