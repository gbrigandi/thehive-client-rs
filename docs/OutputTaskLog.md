# OutputTaskLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** |  | 
**_type** | **String** |  | 
**_created_by** | **String** |  | 
**_created_at** | **i64** |  | 
**_updated_by** | Option<**String**> |  | [optional]
**_updated_at** | Option<**i64**> |  | [optional]
**message** | **String** |  | 
**date** | **i64** |  | 
**owner** | **String** |  | 
**attachments** | Option<[**Vec<models::OutputAttachment>**](OutputAttachment.md)> |  | [optional]
**include_in_timeline** | Option<**bool**> |  | [optional]
**extra_data** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


