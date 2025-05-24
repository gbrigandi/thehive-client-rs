# OutputAlert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** |  | 
**_type** | **String** |  | 
**_created_by** | **String** |  | 
**_created_at** | **i64** |  | 
**_updated_by** | Option<**String**> |  | [optional]
**_updated_at** | Option<**i64**> |  | [optional]
**r#type** | **String** |  | 
**source** | **String** |  | 
**source_ref** | **String** |  | 
**external_link** | Option<**String**> |  | [optional]
**title** | **String** |  | 
**description** | **String** |  | 
**severity** | **i32** |  | 
**severity_label** | **String** |  | 
**date** | **i64** |  | 
**tags** | Option<**Vec<String>**> |  | [optional]
**tlp** | **i32** |  | 
**tlp_label** | **String** |  | 
**pap** | **i32** |  | 
**pap_label** | **String** |  | 
**follow** | **bool** |  | 
**custom_fields** | Option<[**Vec<models::OutputCustomFieldValue>**](OutputCustomFieldValue.md)> |  | [optional]
**status** | **String** |  | 
**summary** | Option<**String**> |  | [optional]
**assignee** | Option<**String**> |  | [optional]
**case_template** | Option<**String**> |  | [optional]
**case_id** | Option<**String**> |  | [optional]
**observable_count** | **i32** |  | 
**stage** | **String** |  | 
**extra_data** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**new_date** | **i64** |  | 
**in_progress_date** | Option<**i64**> |  | [optional]
**closed_date** | Option<**i64**> |  | [optional]
**imported_date** | Option<**i64**> |  | [optional]
**time_to_detect** | **i64** |  | 
**time_to_triage** | Option<**i64**> |  | [optional]
**time_to_qualify** | Option<**i64**> |  | [optional]
**time_to_acknowledge** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


