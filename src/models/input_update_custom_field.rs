/*
 * TheHive API
 *
 * Comprehensive OpenAPI specification inferred from the TheHive4py client library. This API allows interaction with TheHive platform for managing alerts, cases, observables, tasks, users, and other entities. 
 *
 * The version of the OpenAPI document: 2.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputUpdateCustomField {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "mandatory", skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
}

impl InputUpdateCustomField {
    pub fn new() -> InputUpdateCustomField {
        InputUpdateCustomField {
            display_name: None,
            group: None,
            description: None,
            r#type: None,
            options: None,
            mandatory: None,
        }
    }
}

