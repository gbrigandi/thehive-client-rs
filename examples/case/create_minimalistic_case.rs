use std::env;
use thehive_client::apis::case_api;
use thehive_client::apis::configuration::Configuration;
use thehive_client::models::InputCase;

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

    let case_to_create = InputCase {
        title: "A Minimalistic Rust Case".to_string(),
        description: "This case was created from a Rust example.".to_string(),
        severity: None, // Optional: Default is 2 (Medium) if not set by server/template
        start_date: None,
        end_date: None,
        tags: None,
        flag: None,
        tlp: None,
        pap: None,
        status: None, // Optional: Default is "New"
        summary: None,
        assignee: None,
        custom_fields: None,
        case_template: None,
        tasks: None,
        pages: None,
        sharing_parameters: None,
        task_rule: None,
        observable_rule: None,
    };

    println!("Creating minimalistic case: '{}'", case_to_create.title);

    match case_api::create_case(&config, case_to_create, None).await {
        Ok(created_case) => {
            println!("Successfully created case:");
            println!("  ID: {}", created_case._id);
            println!("  Title: {}", created_case.title);
            println!("  Description: {}", created_case.description);
            println!("  Status: {:?}", created_case.status);
            println!("  Severity: {}", created_case.severity);
        }
        Err(e) => {
            eprintln!("Error creating case: {}", e);
        }
    }

    Ok(())
}
