use std::env;
use thehive_client::apis::configuration::Configuration;
use thehive_client::apis::case_api;
use thehive_client::models::InputCase;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_path = env::var("THEHIVE_API_ENDPOINT")
        .unwrap_or_else(|_| {
            println!("THEHIVE_API_ENDPOINT not set, defaulting to http://localhost:9000/api");
            "http://localhost:9000/api".to_string()
        });
    let api_key = env::var("THEHIVE_API_KEY")
        .unwrap_or_else(|_| {
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

    let case_to_create = InputCase {
        title: "Case to Fetch (Rust)".to_string(),
        description: "This case will be created and then fetched.".to_string(),
        tags: Some(Some(vec!["rust_example".to_string(), "fetch_test".to_string()])),
        ..Default::default() // Fills other fields with their default values
    };

    println!("Creating case: '{}'", case_to_create.title);
    let created_case = match case_api::create_case(&config, case_to_create, None).await {
        Ok(case) => {
            println!("Successfully created case with ID: {}", case._id);
            case
        }
        Err(e) => {
            eprintln!("Error creating case: {}", e);
            return Err(e.into());
        }
    };

    println!("Fetching case with ID: {}", created_case._id);
    match case_api::get_case_by_id(&config, &created_case._id, None).await {
        Ok(fetched_case) => {
            println!("Successfully fetched case:");
            println!("  ID: {}", fetched_case._id);
            println!("  Title: {}", fetched_case.title);
            println!("  Description: {}", fetched_case.description);
            println!("  Tags: {:?}", fetched_case.tags.unwrap_or_default());
            println!("  Status: {:?}", fetched_case.status);
        }
        Err(e) => {
            eprintln!("Error fetching case: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}

