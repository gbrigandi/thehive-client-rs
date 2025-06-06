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
pub struct OutputComment {
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "_type")]
    pub _type: String,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Option<i64>>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "isEdited")]
    pub is_edited: bool,
}

impl OutputComment {
    pub fn new(_id: String, _type: String, created_by: String, created_at: i64, message: String, is_edited: bool) -> OutputComment {
        OutputComment {
            _id,
            _type,
            created_by,
            created_at,
            updated_at: None,
            message,
            is_edited,
        }
    }
}

