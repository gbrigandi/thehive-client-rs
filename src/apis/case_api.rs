/*
 * TheHive API
 *
 * Comprehensive OpenAPI specification inferred from the TheHive4py client library. This API allows interaction with TheHive platform for managing alerts, cases, observables, tasks, users, and other entities. 
 *
 * The version of the OpenAPI document: 2.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`create_case`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCaseError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_case`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCaseError {
    Status401(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_case_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCaseByIdError {
    Status401(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_case`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCaseError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


pub async fn create_case(configuration: &configuration::Configuration, input_case: models::InputCase, x_organisation: Option<&str>) -> Result<models::OutputCase, Error<CreateCaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_input_case = input_case;
    let p_x_organisation = x_organisation;

    let uri_str = format!("{}/v1/case", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_x_organisation {
        req_builder = req_builder.header("X-Organisation", param_value.to_string());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_input_case);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::OutputCase`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::OutputCase`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateCaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_case(configuration: &configuration::Configuration, case_id: &str, x_organisation: Option<&str>) -> Result<(), Error<DeleteCaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_case_id = case_id;
    let p_x_organisation = x_organisation;

    let uri_str = format!("{}/v1/case/{case_id}", configuration.base_path, case_id=p_case_id.to_string());
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_x_organisation {
        req_builder = req_builder.header("X-Organisation", param_value.to_string());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteCaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_case_by_id(configuration: &configuration::Configuration, case_id: &str, x_organisation: Option<&str>) -> Result<models::OutputCase, Error<GetCaseByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_case_id = case_id;
    let p_x_organisation = x_organisation;

    let uri_str = format!("{}/v1/case/{case_id}", configuration.base_path, case_id=p_case_id.to_string());
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_x_organisation {
        req_builder = req_builder.header("X-Organisation", param_value.to_string());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::OutputCase`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::OutputCase`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCaseByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_case(configuration: &configuration::Configuration, case_id: &str, input_update_case: models::InputUpdateCase, x_organisation: Option<&str>) -> Result<(), Error<UpdateCaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_case_id = case_id;
    let p_input_update_case = input_update_case;
    let p_x_organisation = x_organisation;

    let uri_str = format!("{}/v1/case/{case_id}", configuration.base_path, case_id=p_case_id.to_string());
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_x_organisation {
        req_builder = req_builder.header("X-Organisation", param_value.to_string());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_input_update_case);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateCaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

