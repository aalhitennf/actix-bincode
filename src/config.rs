#![allow(clippy::module_name_repetitions)]

/// Config for the extractor  
///
///     use actix_bincode::BincodeConfig;
///     use actix_web::App;
/// 
///     let config = BincodeConfig::default();  
///     
///     let app = App::new().app_data(config);  
/// 
#[derive(Clone, Debug)]
pub struct BincodeConfig {
    pub limit: usize,
}

#[allow(dead_code)]
impl BincodeConfig {
    #[must_use]
    pub fn new(limit: usize) -> Self {
        BincodeConfig { limit }
    }
}

// Default 256kb
impl Default for BincodeConfig {
    fn default() -> Self {
        BincodeConfig { limit: 262_144 }
    }
}
