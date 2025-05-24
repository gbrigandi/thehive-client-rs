use std::env;
use thehive_client::apis::case_api;
use thehive_client::apis::configuration::Configuration;
use thehive_client::models::{InputCase, InputUpdateCase};

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

    let initial_case_data = InputCase {
        title: "Original Case Title (Rust)".to_string(),
        description: "This case will be updated.".to_string(),
        tags: Some(Some(vec!["initial_tag".to_string()])),
        ..Default::default()
    };

    println!("Creating case: '{}'", initial_case_data.title);
    let created_case = match case_api::create_case(&config, initial_case_data, None).await {
        Ok(case) => {
            println!("Successfully created case with ID: {}", case._id);
            case
        }
        Err(e) => {
            eprintln!("Error creating case: {}", e);
            return Err(e.into());
        }
    };

    let case_update_data = InputUpdateCase {
        title: Some("Updated Case Title (Rust)".to_string()),
        tags: Some(vec!["updated_tag".to_string(), "rust_example".to_string()]),
        description: Some("The description has been updated.".to_string()),
        ..Default::default() 
    };

    println!("Updating case with ID: {}", created_case._id);
    match case_api::update_case(&config, &created_case._id, case_update_data, None).await {
        Ok(_) => {
            println!(
                "Successfully submitted update for case ID: {}",
                created_case._id
            );
        }
        Err(e) => {
            eprintln!("Error updating case: {}", e);
        }
    }

    println!("Fetching updated case with ID: {}", created_case._id);
    match case_api::get_case_by_id(&config, &created_case._id, None).await {
        Ok(fetched_case) => {
            println!("Successfully fetched case after update attempt:");
            println!("  ID: {}", fetched_case._id);
            println!("  Title: {}", fetched_case.title);
            println!("  Description: {}", fetched_case.description);
            println!("  Tags: {:?}", fetched_case.tags.unwrap_or_default());
        }
        Err(e) => {
            eprintln!("Error fetching case after update: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
