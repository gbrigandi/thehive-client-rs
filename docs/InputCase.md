# InputCase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** |  | 
**description** | **String** |  | 
**severity** | Option<**i32**> |  | [optional]
**start_date** | Option<**i64**> |  | [optional]
**end_date** | Option<**i64**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**flag** | Option<**bool**> |  | [optional][default to false]
**tlp** | Option<**i32**> |  | [optional]
**pap** | Option<**i32**> |  | [optional]
**status** | Option<[**models::CaseStatusValue**](CaseStatusValue.md)> |  | [optional]
**summary** | Option<**String**> |  | [optional]
**assignee** | Option<**String**> |  | [optional]
**custom_fields** | Option<[**Vec<models::InputCustomFieldValue>**](InputCustomFieldValue.md)> |  | [optional]
**case_template** | Option<**String**> |  | [optional]
**tasks** | Option<[**Vec<models::InputTask>**](InputTask.md)> |  | [optional]
**pages** | Option<[**Vec<models::InputCasePage>**](InputCasePage.md)> |  | [optional]
**sharing_parameters** | Option<[**Vec<models::InputShare>**](InputShare.md)> |  | [optional]
**task_rule** | Option<**String**> |  | [optional]
**observable_rule** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


