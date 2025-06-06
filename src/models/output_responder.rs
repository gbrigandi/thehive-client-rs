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
pub struct OutputResponder {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "dataTypeList", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data_type_list: Option<Option<Vec<String>>>,
    #[serde(rename = "cortexIds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cortex_ids: Option<Option<Vec<String>>>,
}

impl OutputResponder {
    pub fn new(id: String, name: String, version: String, description: String) -> OutputResponder {
        OutputResponder {
            id,
            name,
            version,
            description,
            data_type_list: None,
            cortex_ids: None,
        }
    }
}

