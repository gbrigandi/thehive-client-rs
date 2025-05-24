# \QueryApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find_entities_by_query**](QueryApi.md#find_entities_by_query) | **POST** /v1/query | Find entities using a flexible query



## find_entities_by_query

> models::FindEntitiesByQuery200Response find_entities_by_query(input_query, x_organisation, name)
Find entities using a flexible query

Generic endpoint to query various TheHive entities. The structure of the response depends on the `name` query parameter and the query itself. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**input_query** | [**InputQuery**](InputQuery.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |
**name** | Option<**String**> | The type of entity or pre-defined query to run (e.g., \"alerts\", \"cases\", \"alert-observables\"). |  |

### Return type

[**models::FindEntitiesByQuery200Response**](findEntitiesByQuery_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

