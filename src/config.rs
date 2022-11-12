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
    /// Bytes
    pub limit: usize,
}

#[allow(dead_code)]
impl BincodeConfig {
    #[must_use]
    /// Create new config with given limit
    pub fn new(limit: usize) -> Self {
        BincodeConfig { limit }
    }
}

impl Default for BincodeConfig {
    /// Defaults to 256kb
    fn default() -> Self {
        BincodeConfig { limit: 262_144 }
    }
}
