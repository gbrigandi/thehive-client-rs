# OutputObservable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** |  | 
**_type** | **String** |  | 
**_created_by** | **String** |  | 
**_created_at** | **i64** |  | 
**_updated_by** | Option<**String**> |  | [optional]
**_updated_at** | Option<**i64**> |  | [optional]
**data_type** | **String** |  | 
**data** | Option<**String**> |  | [optional]
**attachment** | Option<[**models::OutputAttachment**](OutputAttachment.md)> |  | [optional]
**message** | Option<**String**> |  | [optional]
**start_date** | **i64** |  | 
**tlp** | **i32** |  | 
**tlp_label** | **String** |  | 
**pap** | **i32** |  | 
**pap_label** | **String** |  | 
**ioc** | **bool** |  | 
**sighted** | **bool** |  | 
**sighted_at** | Option<**i64**> |  | [optional]
**reports** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**extra_data** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 
**ignore_similarity** | **bool** |  | 
**tags** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


