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
pub struct InputUpdateCase {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i64>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i64>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "flag", skip_serializing_if = "Option::is_none")]
    pub flag: Option<bool>,
    #[serde(rename = "tlp", skip_serializing_if = "Option::is_none")]
    pub tlp: Option<i32>,
    #[serde(rename = "pap", skip_serializing_if = "Option::is_none")]
    pub pap: Option<i32>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::CaseStatusValue>,
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(rename = "impactStatus", skip_serializing_if = "Option::is_none")]
    pub impact_status: Option<models::ImpactStatusValue>,
    #[serde(rename = "customFields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<models::InputCustomFieldValue>>,
    #[serde(rename = "taskRule", skip_serializing_if = "Option::is_none")]
    pub task_rule: Option<String>,
    #[serde(rename = "observableRule", skip_serializing_if = "Option::is_none")]
    pub observable_rule: Option<String>,
    #[serde(rename = "addTags", skip_serializing_if = "Option::is_none")]
    pub add_tags: Option<Vec<String>>,
    #[serde(rename = "removeTags", skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<String>>,
}

impl InputUpdateCase {
    pub fn new() -> InputUpdateCase {
        InputUpdateCase {
            title: None,
            description: None,
            severity: None,
            start_date: None,
            end_date: None,
            tags: None,
            flag: None,
            tlp: None,
            pap: None,
            status: None,
            summary: None,
            assignee: None,
            impact_status: None,
            custom_fields: None,
            task_rule: None,
            observable_rule: None,
            add_tags: None,
            remove_tags: None,
        }
    }
}

