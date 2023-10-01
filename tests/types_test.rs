#[cfg(test)]
mod tests {

    use carbone_sdk_rust::errors::CarboneError;
    use carbone_sdk_rust::types::*;
    use std::matches;

    use anyhow::Result;

    #[test]
    fn test_api_token_as_str() -> Result<()> {
        let api_token_value = "test_32u1i3ui1212334395349dsaowe912384ads89de8e93hj123iowa21085dsaowe91843784p213894dsa912384ads89de8e93hj123iowa210309dhsudausdasda72q37q783hy3243829434gdgadghdsaowe912384ads89de8e93hj1owa21023113i12u32i1321io39534985dsaowe9123843784p213894309dhsudausdasda72q37q783h43784p213894309dhsuda4gdgadghdsaow2384ads89de8e93hj123iowa21023113i12u32i1321io39534985dsa";
        let api_token = ApiJsonToken::new(api_token_value.to_string())?;

        assert_eq!(api_token_value, api_token.as_str());

        Ok(())
    }

    #[test]
    fn test_api_token_short_token_given() -> Result<()> {
        let api_token_value = "test_";

        let result = ApiJsonToken::new(api_token_value.to_string());
        let is_err = result.is_err();
        let error = result.unwrap_err().to_string();

        let _expected_error = "api_token: Validation error: length [{\"min\": Number(357), \"value\": String(\"test_\")}]";

        assert!(is_err);
        assert!(matches!(error, _expected_error));

        Ok(())
    }

    #[test]
    fn test_id() -> Result<(), CarboneError> {
        let id_value = "0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114";
        let id = Id::new(id_value.to_string(), "id")?;

        assert_eq!(id.as_str(), id_value);

        Ok(())
    }

    #[test]
    fn test_id_empty_value_given() -> Result<(), CarboneError> {
        let id_value = "";
        let result = Id::new(id_value.to_string(), "id");

        let exepected_error = CarboneError::EmptyString("id".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), exepected_error.to_string());

        Ok(())
    }

    #[test]
    fn test_as_ref() -> Result<(), CarboneError> {
        let id_value = "1";
        let id = Id::new(id_value.to_string(), "id")?;

        assert_eq!(id.as_ref(), id_value);

        Ok(())
    }

    #[test]
    fn test_json_data_clone() -> Result<(), CarboneError> {
        let json_data_value = r#"
                                            "data" : {
                                                "firstname" : "John",
                                                "lastname" : "Wick"
                                        },
                                        "convertTo" : "odt"
                                        "#;
        let json_data = JsonData::new(json_data_value.to_string())?;

        let cloned = json_data.clone();

        assert_eq!(json_data, cloned);

        Ok(())
    }

    #[test]
    fn test_json_data() -> Result<(), CarboneError> {
        let json_data_value = r#"
            "data" : {
                "firstname" : "John",
                "lastname" : "Wick"
            },
            "convertTo" : "odt"
        "#;

        let json_data = JsonData::new(json_data_value.to_string())?;

        assert_eq!(json_data.as_str(), json_data_value);

        Ok(())
    }

    #[test]
    fn test_json_data_value_not_given() -> Result<(), CarboneError> {
        let json_data = "";
        let result = JsonData::new(json_data.to_string());

        let exepected_error = CarboneError::EmptyString("json_data".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), exepected_error.to_string());

        Ok(())
    }
}
