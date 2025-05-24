# OutputCaseTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** |  | 
**_type** | **String** |  | 
**_created_by** | **String** |  | 
**_created_at** | **i64** |  | 
**_updated_by** | Option<**String**> |  | [optional]
**_updated_at** | Option<**i64**> |  | [optional]
**name** | **String** |  | 
**display_name** | Option<**String**> |  | [optional]
**title_prefix** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**severity** | Option<[**models::SeverityValue**](SeverityValue.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**flag** | Option<**bool**> |  | [optional]
**tlp** | Option<[**models::TlpValue**](TlpValue.md)> |  | [optional]
**pap** | Option<[**models::PapValue**](PapValue.md)> |  | [optional]
**summary** | Option<**String**> |  | [optional]
**tasks** | Option<[**Vec<models::OutputTask>**](OutputTask.md)> |  | [optional]
**page_template_ids** | Option<**Vec<String>**> |  | [optional]
**custom_fields** | Option<[**Vec<models::OutputCustomFieldValue>**](OutputCustomFieldValue.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


