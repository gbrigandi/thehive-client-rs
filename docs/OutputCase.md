# OutputCase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** |  | 
**_type** | **String** |  | 
**_created_by** | **String** |  | 
**_created_at** | **i64** |  | 
**_updated_by** | Option<**String**> |  | [optional]
**_updated_at** | Option<**i64**> |  | [optional]
**number** | **i32** |  | 
**title** | **String** |  | 
**description** | **String** |  | 
**severity** | **i32** |  | 
**severity_label** | **String** |  | 
**start_date** | **i64** |  | 
**end_date** | Option<**i64**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**flag** | **bool** |  | 
**tlp** | **i32** |  | 
**tlp_label** | **String** |  | 
**pap** | **i32** |  | 
**pap_label** | **String** |  | 
**status** | [**models::CaseStatusValue**](CaseStatusValue.md) |  | 
**stage** | **String** |  | 
**summary** | Option<**String**> |  | [optional]
**impact_status** | Option<[**models::ImpactStatusValue**](ImpactStatusValue.md)> |  | [optional]
**assignee** | Option<**String**> |  | [optional]
**custom_fields** | Option<[**Vec<models::OutputCustomFieldValue>**](OutputCustomFieldValue.md)> |  | [optional]
**user_permissions** | Option<**Vec<String>**> |  | [optional]
**extra_data** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**new_date** | **i64** |  | 
**in_progress_date** | Option<**i64**> |  | [optional]
**closed_date** | Option<**i64**> |  | [optional]
**alert_date** | Option<**i64**> |  | [optional]
**alert_new_date** | Option<**i64**> |  | [optional]
**alert_in_progress_date** | Option<**i64**> |  | [optional]
**alert_imported_date** | Option<**i64**> |  | [optional]
**time_to_detect** | **i64** |  | 
**time_to_triage** | Option<**i64**> |  | [optional]
**time_to_qualify** | Option<**i64**> |  | [optional]
**time_to_acknowledge** | Option<**i64**> |  | [optional]
**time_to_resolve** | Option<**i64**> |  | [optional]
**handling_duration** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


