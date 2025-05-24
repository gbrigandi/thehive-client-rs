# InputAlert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**source** | **String** |  | 
**source_ref** | **String** |  | 
**title** | **String** |  | 
**description** | **String** |  | 
**date** | Option<**i64**> |  | [optional]
**external_link** | Option<**String**> |  | [optional]
**severity** | Option<**i32**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**flag** | Option<**bool**> |  | [optional][default to false]
**tlp** | Option<**i32**> |  | [optional]
**pap** | Option<**i32**> |  | [optional]
**custom_fields** | Option<[**Vec<models::InputCustomFieldValue>**](InputCustomFieldValue.md)> |  | [optional]
**summary** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**assignee** | Option<**String**> |  | [optional]
**case_template** | Option<**String**> |  | [optional]
**observables** | Option<[**Vec<models::InputObservable>**](InputObservable.md)> |  | [optional]
**procedures** | Option<[**Vec<models::InputProcedure>**](InputProcedure.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


