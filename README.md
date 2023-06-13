[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/pascal-chenevas/carbone_sdk_rs/actions/workflows/rust.yml/badge.svg)](https://github.com/pascal-chenevas/carbone_sdk_rs/actions/workflows/rust.yml)
[![unstable](http://badges.github.io/stability-badges/dist/unstable.svg)](http://github.com/badges/stability-badges)

# carbone_sdk_rs

carbone_sdk_rs is a Library which supply functionalites to communicate with the [Carbone API](https://carbone.io/api-reference.html).

# Process to render a new report

```mermaid
sequenceDiagram
    Client->>Carbone API: send a template file to /template
    Carbone API-->>Client: send a template_id 
    Client->>Carbone API: send json data to be rendered to /render/{template_id}
    Carbone API-->>Client: send a render_id
    Carbone API-->>Client: get the rendered report from /render/{render_id}
```

# Installation

TODO

# Render a new report

### Using an existing Template

```rust
use std::env;
 
use carbone_sdk_rs::config::Config;
use carbone_sdk_rs::render::*;
use carbone_sdk_rs::carbone::CarboneSDK;
use carbone_sdk_rs::types::ApiJsonToken;
use carbone_sdk_rs::template::TemplateId;
 
use carbone_sdk_rs::errors::CarboneSdkError;

fn main() -> Result<(), CarboneSdkError> {
    
     let token =  match env::var("CARBONE_TOKEN") {
             Ok(v) => v,
             Err(e) => panic!("{}", e.to_string())
     };
 
    let config = &Config::new("http://127.0.0.1".to_string(), 4, 2)?;
 
    let api_token = &ApiJsonToken::new(token)?;
 
    let template_id = TemplateId::new("0545253258577a632a99065f0572720225f5165cc43db9515e9cef0e17b40114".to_string())?;
    let carbone_sdk = CarboneSDK::new(&config, api_token)?;
 
    let render_options_value = String::from(r#"
         "data" : {
             "firstname" : "John",
             "lastname" : "Wick"
        },
        "convertTo" : "odt"
    "#);
 
    let render_options = RenderOptions::new(render_options_value)?;
    let report_content = carbone_sdk.generate_report_with_template_id(template_id, render_options)?;

    Ok(())
}
```

# References

[Carbone.io](https://carbone.io) a report generator.

## Useful links

- [How to build a template file](https://carbone.io/documentation.html#building-a-template)

- [Substitutions](https://carbone.io/documentation.html#substitutions)

- [Repetitions](https://carbone.io/documentation.html#repetitions)

- [Formatters](https://carbone.io/documentation.html#formatters)

- [Translations](https://carbone.io/documentation.html#translations)
