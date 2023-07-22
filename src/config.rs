#![allow(clippy::module_name_repetitions)]

/// Config for the extractor  
///
///     use actix_bincode::config::BincodeConfig;
///     use actix_web::App;
///
///     let config = BincodeConfig::default();  
///     
///     let app = App::new().app_data(config);  
///
#[derive(Clone, Debug)]
pub struct BincodeConfig {
    /// The maximum size in bytes of a request payload that can be deserialized.
    /// By default set to [DEFAULT_LIMIT_BYTES]
    pub limit: usize,
}

/// The default limit in bytes used when deserializing a request payload.
/// Set to 256 KiB.
pub const DEFAULT_LIMIT_BYTES: usize = 262_144; // 256 KiB

#[allow(dead_code)]
impl BincodeConfig {
    #[must_use]
    /// Create new config with given limit
    pub fn new(limit: usize) -> Self {
        BincodeConfig { limit }
    }
}

impl Default for BincodeConfig {
    /// A default config with limit of 256 KiB.
    fn default() -> Self {
        BincodeConfig {
            limit: DEFAULT_LIMIT_BYTES,
        }
    }
}
