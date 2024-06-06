use std::{borrow::Cow, fmt::Display};

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};

use utils::get_time_nanos;

use super::TimestampNs;

#[derive(Debug, CandidType, Deserialize, Clone, Copy, Ord, PartialOrd, PartialEq, Eq)]
pub struct UserId(Principal);

impl UserId {
    pub fn principal(&self) -> Principal {
        self.0
    }

    pub fn new(principal: Principal) -> Self {
        Self(principal)
    }
}

impl From<UserId> for Principal {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl From<Principal> for UserId {
    fn from(principal: Principal) -> Self {
        Self(principal)
    }
}

impl Storable for UserId {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_text())
    }
}

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    Deployer,
}

impl Storable for UserRole {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct MTlsCertificateData {
    pub cert: String,
    pub pub_key: String,
    pub priv_key: String,
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct User {
    role: UserRole,
    created_at: TimestampNs,
    payments: Vec<u64>,
    akt_balance: f64,
    mtls_certificate: Option<MTlsCertificateData>,
}

impl User {
    pub fn new(role: UserRole) -> Self {
        Self {
            role,
            created_at: get_time_nanos(),
            payments: vec![],
            akt_balance: 0.0,
            mtls_certificate: None,
        }
    }

    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }

    pub fn set_role(&mut self, role: UserRole) {
        self.role = role
    }

    pub fn add_payment(&mut self, payment_block_height: u64) {
        self.payments.push(payment_block_height);
    }

    pub fn is_double_payment(&self, payment_block_height: u64) -> bool {
        self.payments.contains(&payment_block_height)
    }

    pub fn akt_balance(&self) -> f64 {
        self.akt_balance
    }

    pub fn add_to_akt_balance(&mut self, amount: f64) -> f64 {
        self.akt_balance += amount;
        self.akt_balance
    }

    pub fn subtract_from_akt_balance(&mut self, amount: f64) -> f64 {
        self.akt_balance -= amount;
        self.akt_balance
    }

    pub fn set_mtls_certificate(&mut self, certificate: MTlsCertificateData) {
        self.mtls_certificate = Some(certificate);
    }
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateUserInput {
    pub mtls_certificate: Option<MTlsCertificateData>,
}

impl UpdateUserInput {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(cert_data) = &self.mtls_certificate {
            if cert_data.cert.is_empty()
                || cert_data.pub_key.is_empty()
                || cert_data.priv_key.is_empty()
            {
                return Err("Certificate data cannot be empty".to_string());
            }
        }

        Ok(())
    }
}
