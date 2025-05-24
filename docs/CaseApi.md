# \CaseApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_case**](CaseApi.md#create_case) | **POST** /v1/case | Create a case
[**delete_case**](CaseApi.md#delete_case) | **DELETE** /v1/case/{case_id} | Delete a case
[**get_case_by_id**](CaseApi.md#get_case_by_id) | **GET** /v1/case/{case_id} | Get a case by ID or number
[**update_case**](CaseApi.md#update_case) | **PATCH** /v1/case/{case_id} | Update a case



## create_case

> models::OutputCase create_case(input_case, x_organisation)
Create a case

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**input_case** | [**InputCase**](InputCase.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::OutputCase**](OutputCase.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_case

> delete_case(case_id, x_organisation)
Delete a case

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**case_id** | **String** | The ID or number of the case. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_case_by_id

> models::OutputCase get_case_by_id(case_id, x_organisation)
Get a case by ID or number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**case_id** | **String** | The ID or number of the case. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::OutputCase**](OutputCase.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_case

> update_case(case_id, input_update_case, x_organisation)
Update a case

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**case_id** | **String** | The ID or number of the case. | [required] |
**input_update_case** | [**InputUpdateCase**](InputUpdateCase.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

