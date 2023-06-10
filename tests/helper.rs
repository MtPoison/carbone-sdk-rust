use httpmock::prelude::*;
use anyhow::Result;

use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::types::ApiJsonToken;
use carbone_sdk_rs::errors::CarboneSdkError;

const TOKEN_TEST: &str = "test_32u1i3ui1212334395349dsaowe912384ads89de8e93hj123iowa21085dsaowe91843784p213894dsa912384ads89de8e93hj123iowa210309dhsudausdasda72q37q783hy3243829434gdgadghdsaowe912384ads89de8e93hj1owa21023113i12u32i1321io39534985dsaowe9123843784p213894309dhsudausdasda72q37q783h43784p213894309dhsuda4gdgadghdsaow2384ads89de8e93hj123iowa21023113i12u32i1321io39534985dsa";

pub struct Helper();

impl Helper {

    pub fn new() -> Self {
        Self{}
    }
    
    pub fn create_config_for_mock_server(&self, server: Option<&MockServer>) -> Result<Config, CarboneSdkError> {
    
        let port = match server {
            Some(s) => s.port(),
            None => 8080
        };
    
        let config = Config::new(
            format!("{}{}", "http://127.0.0.1:", port), // port changes each run when used with the MockServer
            4,
            2
        )?;
        Ok(config)
    }
    
    pub fn create_api_token(&self) -> Result<ApiJsonToken> {
        let api_token = ApiJsonToken::new(TOKEN_TEST.to_string())?;
        Ok(api_token)
    }
}