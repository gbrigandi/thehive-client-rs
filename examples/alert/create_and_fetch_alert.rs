use std::env;
use thehive_client::apis::alert_api;
use thehive_client::apis::configuration::Configuration;
use thehive_client::models::{self, InputAlert}; // Ensure models::input_alert is available for enums

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
        r#type: "fetch_test_type".to_string(),
        source: "rust_create_fetch_example".to_string(),
        source_ref: format!("create_fetch_alert_{}", chrono::Utc::now().timestamp()),
        title: "Alert to Create and Fetch (Rust)".to_string(),
        description: "This alert will be created and then fetched by ID.".to_string(),
        severity: Some(Some(models::input_alert::Severity::Variant1)), // Low
        tags: Some(Some(vec![
            "rust_example".to_string(),
            "fetch_test".to_string(),
        ])),
        tlp: Some(Some(models::input_alert::Tlp::Variant2)), // Amber
        pap: Some(Some(models::input_alert::Pap::Variant2)), // Amber
        ..Default::default()
    };

    println!("Creating alert: '{}'", alert_to_create.title);
    let created_alert = match alert_api::create_alert(&config, alert_to_create, None).await {
        Ok(alert) => {
            println!("Successfully created alert with ID: {}", alert._id);
            alert
        }
        Err(e) => {
            eprintln!("Error creating alert: {}", e);
            return Err(e.into());
        }
    };

    println!("Fetching alert with ID: {}", created_alert._id);
    match alert_api::get_alert_by_id(&config, &created_alert._id, None).await {
        Ok(fetched_alert) => {
            println!("Successfully fetched alert:");
            println!("  ID: {}", fetched_alert._id);
            println!("  Title: {}", fetched_alert.title);
            println!("  Description: {}", fetched_alert.description);
            println!(
                "  Severity: {} ({})",
                fetched_alert.severity, fetched_alert.severity_label
            );
            println!(
                "  Tags: {:?}",
                fetched_alert.tags.unwrap_or_default().unwrap_or_default()
            );
            println!("  TLP: {} ({})", fetched_alert.tlp, fetched_alert.tlp_label);
            println!("  PAP: {} ({})", fetched_alert.pap, fetched_alert.pap_label);
            println!("  Status: {}", fetched_alert.status);
        }
        Err(e) => {
            eprintln!("Error fetching alert: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
