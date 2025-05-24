use std::env;
use thehive_client::apis::alert_api;
use thehive_client::apis::configuration::Configuration;
use thehive_client::models::{self, InputAlert, InputUpdateAlert};

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

    let initial_alert_data = InputAlert {
        r#type: "update_test_type".to_string(),
        source: "rust_create_update_example".to_string(),
        source_ref: format!("create_update_alert_{}", chrono::Utc::now().timestamp()),
        title: "Original Alert Title (Rust)".to_string(),
        description: "This alert will be updated.".to_string(),
        severity: Some(Some(models::input_alert::Severity::Variant2)), // Medium
        tags: Some(Some(vec!["initial_tag".to_string()])),
        ..Default::default()
    };

    println!("Creating alert: '{}'", initial_alert_data.title);
    let created_alert = match alert_api::create_alert(&config, initial_alert_data, None).await {
        Ok(alert) => {
            println!("Successfully created alert with ID: {}", alert._id);
            alert
        }
        Err(e) => {
            eprintln!("Error creating alert: {}", e);
            return Err(e.into());
        }
    };

    let alert_update_data = InputUpdateAlert {
        title: Some("Updated Alert Title (Rust)".to_string()),
        description: Some("The description has been updated.".to_string()),
        severity: Some(3),
        tags: Some(vec!["updated_tag".to_string(), "rust_example".to_string()]),
        status: Some("InProgress".to_string()), // Example status update
        ..Default::default()
    };

    println!("Updating alert with ID: {}", created_alert._id);
    match alert_api::update_alert(&config, &created_alert._id, alert_update_data, None).await {
        Ok(_) => {
            println!(
                "Successfully submitted update for alert ID: {}",
                created_alert._id
            );
        }
        Err(e) => {
            eprintln!("Error updating alert: {}", e);
        }
    }

    println!("Fetching updated alert with ID: {}", created_alert._id);
    match alert_api::get_alert_by_id(&config, &created_alert._id, None).await {
        Ok(fetched_alert) => {
            println!("Successfully fetched alert after update attempt:");
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
            println!("  Status: {}", fetched_alert.status);
        }
        Err(e) => {
            eprintln!("Error fetching alert after update: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
