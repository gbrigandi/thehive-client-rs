# OutputUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** |  | 
**_created_by** | **String** |  | 
**_created_at** | **i64** |  | 
**_updated_by** | Option<**String**> |  | [optional]
**_updated_at** | Option<**i64**> |  | [optional]
**login** | **String** |  | 
**name** | **String** |  | 
**email** | Option<**String**> |  | [optional]
**has_key** | **bool** |  | 
**has_password** | **bool** |  | 
**has_mfa** | **bool** |  | 
**locked** | **bool** |  | 
**profile** | **String** |  | 
**organisation** | **String** |  | 
**r#type** | **String** |  | 
**permissions** | Option<**Vec<String>**> |  | [optional]
**avatar** | Option<**String**> |  | [optional]
**organisations** | Option<[**Vec<models::OutputOrganisationProfile>**](OutputOrganisationProfile.md)> |  | [optional]
**default_organisation** | Option<**String**> |  | [optional]
**extra_data** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


