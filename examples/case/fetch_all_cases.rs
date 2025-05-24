use serde_json::json;
use std::collections::HashMap;
use std::env;
use thehive_client::{
    apis::{configuration::Configuration, query_api},
    models::{
        CaseStatusValue, FindEntitiesByQuery200Response, InputQuery, OutputCase, QueryOperation,
    },
};

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

    let list_case_op = QueryOperation {
        _name: Some("listCase".to_string()),
        additional_fields: HashMap::new(), // No additional fields for listCase
    };

    let mut page_op_fields = HashMap::new();
    page_op_fields.insert("from".to_string(), json!(0));
    page_op_fields.insert("to".to_string(), json!(20)); // Fetching first 20 cases
    page_op_fields.insert("extraData".to_string(), json!([]));

    let page_op = QueryOperation {
        _name: Some("page".to_string()),
        additional_fields: page_op_fields,
    };

    let input_query = InputQuery {
        query: Some(vec![list_case_op, page_op]),
        exclude_fields: None,
    };

    println!("Fetching all cases (first 20)...");

    match query_api::find_entities_by_query(&config, input_query, None, Some("cases")).await {
        Ok(response) => match response {
            FindEntitiesByQuery200Response::Array(cases_json) => {
                if cases_json.is_empty() {
                    println!("No cases found.");
                } else {
                    println!("Successfully fetched {} case(s):", cases_json.len());
                    for case_value in cases_json {
                        match serde_json::from_value::<OutputCase>(case_value.clone()) {
                            Ok(case_detail) => {
                                print_case_details(&case_detail);
                            }
                            Err(e) => {
                                eprintln!(
                                    "Error deserializing case: {}. Raw JSON: {}",
                                    e, case_value
                                );
                            }
                        }
                    }
                }
            }
            _ => {
                eprintln!("Unexpected response format when fetching cases.");
            }
        },
        Err(e) => {
            eprintln!("Error fetching cases: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

fn print_case_details(case: &OutputCase) {
    println!("----------------------------------------");
    println!("  ID: {}", case._id);
    println!("  Case Number: {}", case.number);
    println!("  Title: {}", case.title);
    println!("  Severity: {} ({})", case.severity, case.severity_label);
    println!("  Status: {}", case.status);
    println!(
        "  Start Date: {}",
        chrono::DateTime::from_timestamp_millis(case.start_date)
            .map_or_else(|| "Invalid date".to_string(), |dt| dt.to_rfc3339())
    );
    if let Some(Some(tags)) = &case.tags {
        if !tags.is_empty() {
            println!("  Tags: {}", tags.join(", "));
        }
    }
    println!("  Description: {}", case.description);
    println!("----------------------------------------");
}
