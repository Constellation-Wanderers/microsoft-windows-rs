#[cfg(feature = "Security_Authentication")]
pub mod Authentication;
#[cfg(feature = "Security_Authorization")]
pub mod Authorization;
#[cfg(feature = "Security_Credentials")]
pub mod Credentials;
#[cfg(feature = "Security_Cryptography")]
pub mod Cryptography;
#[cfg(feature = "Security_DataProtection")]
pub mod DataProtection;
#[cfg(feature = "Security_EnterpriseData")]
pub mod EnterpriseData;
#[cfg(feature = "Security_ExchangeActiveSyncProvisioning")]
pub mod ExchangeActiveSyncProvisioning;
#[cfg(feature = "Security_Isolation")]
pub mod Isolation;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
