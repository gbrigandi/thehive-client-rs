use std::env;
use std::error::Error as StdError;
use thehive_client::apis::alert_api;
use thehive_client::apis::configuration::Configuration;
use thehive_client::models::InputAlert;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_path = env::var("THEHIVE_API_ENDPOINT").unwrap_or_else(|_| {
        println!("THEHIVE_API_ENDPOINT not set, defaulting to http://localhost:9000/api");
        "http://localhost:9000/api".to_string()
    });
    let api_key = env::var("THEHIVE_API_KEY").unwrap_or_else(|_| {
        println!("THEHIVE_API_KEY not set, using placeholder. Please set it for actual use.");
        "YOUR_API_KEY".to_string()
    });

    if api_key == "YOUR_API_KEY" {
        eprintln!("Please set the THEHIVE_API_KEY environment variable or replace 'YOUR_API_KEY' in the code.");
        return Ok(());
    }

    let mut config = Configuration::new();
    config.base_path = base_path;
    config.bearer_access_token = Some(api_key);

    let alert_to_create = InputAlert {
        r#type: "test_alert_type".to_string(),
        source: "rust_minimal_example".to_string(),
        source_ref: format!("minimal_alert_{}", chrono::Utc::now().timestamp()),
        title: "Minimalistic Rust Alert".to_string(),
        description: "This alert was created from a minimalistic Rust example.".to_string(),
        ..Default::default() // Fill other optional fields with None or their default
    };

    println!("Creating minimalistic alert: '{}'", alert_to_create.title);

    match alert_api::create_alert(&config, alert_to_create, None).await {
        Ok(created_alert) => {
            println!("Successfully created alert:");
            println!("  ID: {}", created_alert._id);
            println!("  Title: {}", created_alert.title);
            println!("  Type: {}", created_alert.r#type);
            println!("  Source: {}", created_alert.source);
            println!("  SourceRef: {}", created_alert.source_ref);
            println!(
                "  Severity: {} ({})",
                created_alert.severity, created_alert.severity_label
            );
            println!("  Status: {}", created_alert.status);
        }
        Err(e) => {
            eprintln!("Error creating alert: {}", e);
            if let Some(source) = e.source() {
                // .source() is from StdError trait
                eprintln!("  Caused by: {}", source);
            }
            if let thehive_client::apis::Error::ResponseError(ref response_details) = e {
                eprintln!("  Response Status: {}", response_details.status);
                eprintln!("  Response Content: {}", response_details.content);
            }
            return Err(Box::new(e) as Box<dyn StdError>);
        }
    }

    Ok(())
}
