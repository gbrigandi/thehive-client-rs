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
pub struct OutputTimelineEvent {
    #[serde(rename = "date")]
    pub date: i64,
    #[serde(rename = "endDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<i64>>,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "entity")]
    pub entity: String,
    #[serde(rename = "entityId")]
    pub entity_id: String,
    #[serde(rename = "details")]
    pub details: std::collections::HashMap<String, serde_json::Value>,
}

impl OutputTimelineEvent {
    pub fn new(date: i64, kind: String, entity: String, entity_id: String, details: std::collections::HashMap<String, serde_json::Value>) -> OutputTimelineEvent {
        OutputTimelineEvent {
            date,
            end_date: None,
            kind,
            entity,
            entity_id,
            details,
        }
    }
}

