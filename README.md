# TheHive API Client for Rust

TheHive is an open-source, scalable, and collaborative Security Incident Response Platform (SIRP). It is designed to help SOCs, CSIRTs, CERTs, and any security practitioner to investigate and respond to security incidents quickly and efficiently. TheHive allows analysts to create cases for investigations, add observables (like IPs, URLs, domains, files), create tasks, log their progress, and collaborate with team members.

The TheHive API provides a programmatic interface to its comprehensive set of features, offering several advantages:

*   **Automation:** Automate the creation of alerts and cases from external systems (SIEMs, EDRs, email alerts), and manage their lifecycle programmatically.
*   **Integration:** Seamlessly integrate TheHive's incident management capabilities into your existing security ecosystem (SOAR platforms, threat intelligence feeds, custom scripts).
*   **Scalability:** Handle a large volume of security events and manage numerous cases and observables programmatically.
*   **Customization:** Build custom scripts and tools that leverage TheHive's backend for tailored security operations and reporting.
*   **Efficiency:** Speed up incident response by automating repetitive tasks, enriching cases with threat intelligence, and orchestrating response actions.

## Installation

To use this client in your Rust project, add it as a dependency in your `Cargo.toml`.

```toml
[dependencies]
thehive-client = "0.1"
```


## Rust Usage Examples

Below are conceptual examples of how to use the generated Rust client to interact with TheHive API. Ensure you have configured your client with the correct TheHive URL and API key, via environment variables `THEHIVE_API_ENDPOINT` and `THEHIVE_API_KEY`.

### 1. Create an Alert with an IP Observable

This example demonstrates how to create a new alert in TheHive and attach an IP address as an observable.

```rust
use std::env;
use thehive_client::apis::configuration::Configuration;
use thehive_client::apis::alert_api; // Module for alert operations
use thehive_client::models::{InputAlert, InputObservable};

async fn create_alert_with_ip(
    thehive_url: &str,
    api_key: &str,
    alert_title: &str,
    ip_address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Configuration::new();
    config.base_path = thehive_url.to_string();
    config.bearer_access_token = Some(api_key.to_string());

    let observable = InputObservable {
        data_type: "ip".to_string(),
        data: Some(ip_address.to_string()),
        message: Some("Suspicious IP address from Rust example".to_string()),
        ioc: Some(true),
        tlp: Some(2), // TLP:AMBER (0:White, 1:Green, 2:Amber, 3:Red)
        pap: Some(2), // PAP:AMBER (0:White, 1:Green, 2:Amber, 3:Red)
        tags: Some(vec!["suspicious_ip".to_string()]),
        ..Default::default() // Fills other Option fields with None
    };

    let alert_data = InputAlert {
        r#type: "external_event".to_string(), // A common type for alerts
        source: "rust_client_example_source".to_string(),
        source_ref: format!("rust-example-alert-{}", chrono::Utc::now().timestamp_micros()), // Example unique ref
        title: alert_title.to_string(),
        description: format!("This alert was automatically generated by the Rust client regarding IP: {}.", ip_address),
        severity: Some(2), // Severity: 1 (Low), 2 (Medium), 3 (High), 4 (Critical)
        tags: Some(vec!["rust_example".to_string(), "auto_generated_alert".to_string()]),
        observables: Some(vec![observable]),
        ..Default::default()
    };

    match alert_api::create_alert(&config, alert_data, None).await {
        Ok(created_alert) => {
            println!(
                "Alert '{}' created successfully! ID: {}",
                created_alert.title,
                created_alert._id
            );
            // You can now use created_alert._id for further operations like promoting to case.
        }
        Err(e) => {
            eprintln!("Error creating alert: {:?}", e);
        }
    }
    Ok(())
}
```
### 2. Create a New Case

This example shows how to create a new case from scratch.

```rust
use std::env;
use thehive_client::apis::configuration::Configuration;
use thehive_client::apis::case_api; // Module for case operations
use thehive_client::models::{InputCase, CaseStatusValue};

async fn create_new_case(
    thehive_url: &str,
    api_key: &str,
    case_title: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Configuration::new();
    config.base_path = thehive_url.to_string();
    config.bearer_access_token = Some(api_key.to_string());

    let case_data = InputCase {
        title: case_title.to_string(),
        description: "This is a new case created via the Rust API client.".to_string(),
        severity: Some(2), // Medium
        tags: Some(vec!["rust_example".to_string(), "new_investigation".to_string()]),
        flag: Some(false),
        tlp: Some(2), // TLP:AMBER
        pap: Some(2), // PAP:AMBER
        status: Some(CaseStatusValue::New), 
        ..Default::default()
    };

    match case_api::create_case(&config, case_data, None).await {
        Ok(created_case) => {
            println!(
                "Case '{}' created successfully! ID: {}, Number: {}",
                created_case.title,
                created_case._id,
                created_case.number
            );
            // You can now use created_case._id or created_case.number for further operations.
        }
        Err(e) => {
            eprintln!("Error creating case: {:?}", e);
        }
    }
    Ok(())
}
```

### 3. Promote an Alert to a Case

This example demonstrates how to promote an existing alert to a new case. You would get the `alert_id_to_promote` from a previous operation (like creating an alert).

```rust
use std::env;
use thehive_client::apis::configuration::Configuration;
use thehive_client::apis::alert_api; 
use thehive_client::models::InputPromoteAlert; 

async fn promote_alert_to_new_case(
    thehive_url: &str,
    api_key: &str,
    alert_id_to_promote: &str,
    case_title_prefix: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Configuration::new();
    config.base_path = thehive_url.to_string();
    config.bearer_access_token = Some(api_key.to_string());

    let promotion_details = InputPromoteAlert {
        title: case_title_prefix, 
        ..Default::default()
    };

    match alert_api::promote_alert_to_case(&config, alert_id_to_promote, None, Some(promotion_details)).await {
        Ok(created_case) => {
            println!(
                "Alert {} promoted to Case '{}' successfully! Case ID: {}, Case Number: {}",
                alert_id_to_promote,
                created_case.title,
                created_case._id,
                created_case.number
            );
        }
        Err(e) => {
            eprintln!("Error promoting alert {} to case: {:?}", alert_id_to_promote, e);
        }
    }
    Ok(())
}
```
## Running the Examples

To run the provided examples, you'll first need to ensure you have a running TheHive instance and the necessary credentials. The examples are configured to use environment variables for the API endpoint and API key.

**Prerequisites:**
*   A running TheHive 5.x instance.
*   An API key for your TheHive user.
*   Rust and Cargo installed.
*   The client library cloned or added as a dependency to your project as described in the "Installation" section.

**Environment Variables:**
Before running any example, you need to set the following environment variables in your terminal session:

*   `THEHIVE_API_ENDPOINT`: The URL of your TheHive API. If not set, it defaults to `http://localhost:9000/api` in the examples.
    ```bash
    export THEHIVE_API_ENDPOINT="http://your-thehive-instance.com:9000/api"
    ```
*   `THEHIVE_API_KEY`: Your TheHive API key. This variable **must** be set.
    ```bash
    export THEHIVE_API_KEY="your_actual_api_key"
    ```

**Executing an Example:**
Once the environment variables are set, you can run an example using Cargo. The examples are located in the `examples/` directory and defined in the `Cargo.toml` file.

For instance, to run an example named `create_minimalistic_case`:
```bash
cargo run --example create_minimalistic_case
```

Replace `create_minimalistic_case` with the actual name of the example you wish to run. You can list available examples by checking the `examples/` directory and the `Cargo.toml` file.


## Documentation for API Endpoints

All URIs are relative to *http://localhost:9000/api*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AlertApi* | [**add_alert_attachments**](docs/AlertApi.md#add_alert_attachments) | **POST** /v1/alert/{alert_id}/attachments | Add attachments to an alert
*AlertApi* | [**bulk_delete_alerts**](docs/AlertApi.md#bulk_delete_alerts) | **POST** /v1/alert/delete/_bulk | Bulk delete alerts
*AlertApi* | [**bulk_merge_alerts_into_case**](docs/AlertApi.md#bulk_merge_alerts_into_case) | **POST** /v1/alert/merge/_bulk | Bulk merge alerts into a case
*AlertApi* | [**bulk_update_alerts**](docs/AlertApi.md#bulk_update_alerts) | **PATCH** /v1/alert/_bulk | Bulk update alerts
*AlertApi* | [**create_alert**](docs/AlertApi.md#create_alert) | **POST** /v1/alert | Create an alert
*AlertApi* | [**create_alert_observable**](docs/AlertApi.md#create_alert_observable) | **POST** /v1/alert/{alert_id}/observable | Create an observable in an alert
*AlertApi* | [**create_alert_procedure**](docs/AlertApi.md#create_alert_procedure) | **POST** /v1/alert/{alert_id}/procedure | Create a procedure in an alert
*AlertApi* | [**delete_alert**](docs/AlertApi.md#delete_alert) | **DELETE** /v1/alert/{alert_id} | Delete an alert
*AlertApi* | [**delete_alert_attachment**](docs/AlertApi.md#delete_alert_attachment) | **DELETE** /v1/alert/{alert_id}/attachment/{attachment_id} | Delete an alert attachment
*AlertApi* | [**download_alert_attachment**](docs/AlertApi.md#download_alert_attachment) | **GET** /v1/alert/{alert_id}/attachment/{attachment_id}/download | Download an alert attachment
*AlertApi* | [**follow_alert**](docs/AlertApi.md#follow_alert) | **POST** /v1/alert/{alert_id}/follow | Follow an alert
*AlertApi* | [**get_alert_by_id**](docs/AlertApi.md#get_alert_by_id) | **GET** /v1/alert/{alert_id} | Get an alert by ID
*AlertApi* | [**merge_alert_into_case**](docs/AlertApi.md#merge_alert_into_case) | **POST** /v1/alert/{alert_id}/merge/{case_id} | Merge an alert into an existing case
*AlertApi* | [**promote_alert_to_case**](docs/AlertApi.md#promote_alert_to_case) | **POST** /v1/alert/{alert_id}/case | Promote an alert to a case
*AlertApi* | [**unfollow_alert**](docs/AlertApi.md#unfollow_alert) | **POST** /v1/alert/{alert_id}/unfollow | Unfollow an alert
*AlertApi* | [**update_alert**](docs/AlertApi.md#update_alert) | **PATCH** /v1/alert/{alert_id} | Update an alert
*CaseApi* | [**create_case**](docs/CaseApi.md#create_case) | **POST** /v1/case | Create a case
*CaseApi* | [**delete_case**](docs/CaseApi.md#delete_case) | **DELETE** /v1/case/{case_id} | Delete a case
*CaseApi* | [**get_case_by_id**](docs/CaseApi.md#get_case_by_id) | **GET** /v1/case/{case_id} | Get a case by ID or number
*CaseApi* | [**update_case**](docs/CaseApi.md#update_case) | **PATCH** /v1/case/{case_id} | Update a case
*CaseTemplateApi* | [**create_case_template**](docs/CaseTemplateApi.md#create_case_template) | **POST** /v1/caseTemplate | Create a case template
*CaseTemplateApi* | [**delete_case_template**](docs/CaseTemplateApi.md#delete_case_template) | **DELETE** /v1/caseTemplate/{case_template_id} | Delete a case template
*CaseTemplateApi* | [**get_case_template_by_id**](docs/CaseTemplateApi.md#get_case_template_by_id) | **GET** /v1/caseTemplate/{case_template_id} | Get a case template by ID
*CaseTemplateApi* | [**update_case_template**](docs/CaseTemplateApi.md#update_case_template) | **PATCH** /v1/caseTemplate/{case_template_id} | Update a case template
*CommentApi* | [**create_alert_comment**](docs/CommentApi.md#create_alert_comment) | **POST** /v1/alert/{alert_id}/comment | Create a comment in an alert
*CommentApi* | [**create_case_comment**](docs/CommentApi.md#create_case_comment) | **POST** /v1/case/{case_id}/comment | Create a comment in a case
*CommentApi* | [**delete_comment**](docs/CommentApi.md#delete_comment) | **DELETE** /v1/comment/{comment_id} | Delete a comment
*CommentApi* | [**update_comment**](docs/CommentApi.md#update_comment) | **PATCH** /v1/comment/{comment_id} | Update a comment
*QueryApi* | [**find_entities_by_query**](docs/QueryApi.md#find_entities_by_query) | **POST** /v1/query | Find entities using a flexible query
*UserApi* | [**create_user**](docs/UserApi.md#create_user) | **POST** /v1/user | Create a new user
*UserApi* | [**delete_user**](docs/UserApi.md#delete_user) | **DELETE** /v1/user/{user_id}/force | Delete a user (force)
*UserApi* | [**get_current_user**](docs/UserApi.md#get_current_user) | **GET** /v1/user/current | Get current authenticated user's details
*UserApi* | [**get_user_api_key**](docs/UserApi.md#get_user_api_key) | **GET** /v1/user/{user_id}/key | Get user's API key
*UserApi* | [**get_user_by_id**](docs/UserApi.md#get_user_by_id) | **GET** /v1/user/{user_id} | Get user details by ID
*UserApi* | [**remove_user_api_key**](docs/UserApi.md#remove_user_api_key) | **DELETE** /v1/user/{user_id}/key | Remove user's API key
*UserApi* | [**renew_user_api_key**](docs/UserApi.md#renew_user_api_key) | **POST** /v1/user/{user_id}/key/renew | Renew user's API key
*UserApi* | [**set_user_organisations**](docs/UserApi.md#set_user_organisations) | **PUT** /v1/user/{user_id}/organisations | Set user's organisations and profiles
*UserApi* | [**set_user_password**](docs/UserApi.md#set_user_password) | **POST** /v1/user/{user_id}/password/set | Set a user's password
*UserApi* | [**update_user**](docs/UserApi.md#update_user) | **PATCH** /v1/user/{user_id} | Update user details


## Documentation For Models

 - [AddAlertAttachments201Response](docs/AddAlertAttachments201Response.md)
 - [BulkDeleteAlertsRequest](docs/BulkDeleteAlertsRequest.md)
 - [BulkMergeAlertsIntoCaseRequest](docs/BulkMergeAlertsIntoCaseRequest.md)
 - [CaseStatusValue](docs/CaseStatusValue.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [FindEntitiesByQuery200Response](docs/FindEntitiesByQuery200Response.md)
 - [ImpactStatusValue](docs/ImpactStatusValue.md)
 - [InputAlert](docs/InputAlert.md)
 - [InputAnalyzerJob](docs/InputAnalyzerJob.md)
 - [InputApplyCaseTemplate](docs/InputApplyCaseTemplate.md)
 - [InputBulkOrganisationLink](docs/InputBulkOrganisationLink.md)
 - [InputBulkUpdateAlert](docs/InputBulkUpdateAlert.md)
 - [InputBulkUpdateCase](docs/InputBulkUpdateCase.md)
 - [InputBulkUpdateObservable](docs/InputBulkUpdateObservable.md)
 - [InputBulkUpdateTask](docs/InputBulkUpdateTask.md)
 - [InputCase](docs/InputCase.md)
 - [InputCasePage](docs/InputCasePage.md)
 - [InputCaseTemplate](docs/InputCaseTemplate.md)
 - [InputComment](docs/InputComment.md)
 - [InputCustomEvent](docs/InputCustomEvent.md)
 - [InputCustomField](docs/InputCustomField.md)
 - [InputCustomFieldValue](docs/InputCustomFieldValue.md)
 - [InputImportCase](docs/InputImportCase.md)
 - [InputObservable](docs/InputObservable.md)
 - [InputObservableType](docs/InputObservableType.md)
 - [InputOrganisation](docs/InputOrganisation.md)
 - [InputOrganisationLink](docs/InputOrganisationLink.md)
 - [InputProcedure](docs/InputProcedure.md)
 - [InputProfile](docs/InputProfile.md)
 - [InputPromoteAlert](docs/InputPromoteAlert.md)
 - [InputQuery](docs/InputQuery.md)
 - [InputResponderAction](docs/InputResponderAction.md)
 - [InputShare](docs/InputShare.md)
 - [InputTask](docs/InputTask.md)
 - [InputTaskLog](docs/InputTaskLog.md)
 - [InputUpdateAlert](docs/InputUpdateAlert.md)
 - [InputUpdateCase](docs/InputUpdateCase.md)
 - [InputUpdateCasePage](docs/InputUpdateCasePage.md)
 - [InputUpdateComment](docs/InputUpdateComment.md)
 - [InputUpdateCustomEvent](docs/InputUpdateCustomEvent.md)
 - [InputUpdateCustomField](docs/InputUpdateCustomField.md)
 - [InputUpdateObservable](docs/InputUpdateObservable.md)
 - [InputUpdateOrganisation](docs/InputUpdateOrganisation.md)
 - [InputUpdateProcedure](docs/InputUpdateProcedure.md)
 - [InputUpdateProfile](docs/InputUpdateProfile.md)
 - [InputUpdateTask](docs/InputUpdateTask.md)
 - [InputUpdateTaskLog](docs/InputUpdateTaskLog.md)
 - [InputUpdateUser](docs/InputUpdateUser.md)
 - [InputUser](docs/InputUser.md)
 - [InputUserOrganisation](docs/InputUserOrganisation.md)
 - [MergeAlertIntoCaseCaseIdParameter](docs/MergeAlertIntoCaseCaseIdParameter.md)
 - [OutputAlert](docs/OutputAlert.md)
 - [OutputAnalyzer](docs/OutputAnalyzer.md)
 - [OutputAnalyzerJob](docs/OutputAnalyzerJob.md)
 - [OutputAttachment](docs/OutputAttachment.md)
 - [OutputCase](docs/OutputCase.md)
 - [OutputCasePage](docs/OutputCasePage.md)
 - [OutputCaseTemplate](docs/OutputCaseTemplate.md)
 - [OutputComment](docs/OutputComment.md)
 - [OutputCustomEvent](docs/OutputCustomEvent.md)
 - [OutputCustomField](docs/OutputCustomField.md)
 - [OutputCustomFieldValue](docs/OutputCustomFieldValue.md)
 - [OutputObservable](docs/OutputObservable.md)
 - [OutputObservableType](docs/OutputObservableType.md)
 - [OutputOrganisation](docs/OutputOrganisation.md)
 - [OutputOrganisationProfile](docs/OutputOrganisationProfile.md)
 - [OutputProcedure](docs/OutputProcedure.md)
 - [OutputProfile](docs/OutputProfile.md)
 - [OutputResponder](docs/OutputResponder.md)
 - [OutputResponderAction](docs/OutputResponderAction.md)
 - [OutputShare](docs/OutputShare.md)
 - [OutputSharingProfile](docs/OutputSharingProfile.md)
 - [OutputTask](docs/OutputTask.md)
 - [OutputTaskLog](docs/OutputTaskLog.md)
 - [OutputTimeline](docs/OutputTimeline.md)
 - [OutputTimelineEvent](docs/OutputTimelineEvent.md)
 - [OutputUser](docs/OutputUser.md)
 - [OutputUserOrganisation](docs/OutputUserOrganisation.md)
 - [PapValue](docs/PapValue.md)
 - [QueryFilterLt](docs/QueryFilterLt.md)
 - [QueryFilterLtAllOfLt](docs/QueryFilterLtAllOfLt.md)
 - [QueryOperation](docs/QueryOperation.md)
 - [QueryPaginate](docs/QueryPaginate.md)
 - [QuerySortExpr](docs/QuerySortExpr.md)
 - [SetUserOrganisations200Response](docs/SetUserOrganisations200Response.md)
 - [SetUserOrganisationsRequest](docs/SetUserOrganisationsRequest.md)
 - [SetUserPasswordRequest](docs/SetUserPasswordRequest.md)
 - [SeverityValue](docs/SeverityValue.md)
 - [TlpValue](docs/TlpValue.md)

