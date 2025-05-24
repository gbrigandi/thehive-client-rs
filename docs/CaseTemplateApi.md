# \CaseTemplateApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_case_template**](CaseTemplateApi.md#create_case_template) | **POST** /v1/caseTemplate | Create a case template
[**delete_case_template**](CaseTemplateApi.md#delete_case_template) | **DELETE** /v1/caseTemplate/{case_template_id} | Delete a case template
[**get_case_template_by_id**](CaseTemplateApi.md#get_case_template_by_id) | **GET** /v1/caseTemplate/{case_template_id} | Get a case template by ID
[**update_case_template**](CaseTemplateApi.md#update_case_template) | **PATCH** /v1/caseTemplate/{case_template_id} | Update a case template



## create_case_template

> models::OutputCaseTemplate create_case_template(input_case_template, x_organisation)
Create a case template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**input_case_template** | [**InputCaseTemplate**](InputCaseTemplate.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::OutputCaseTemplate**](OutputCaseTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_case_template

> delete_case_template(case_template_id, x_organisation)
Delete a case template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**case_template_id** | **String** | The ID of the case template. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_case_template_by_id

> models::OutputCaseTemplate get_case_template_by_id(case_template_id, x_organisation)
Get a case template by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**case_template_id** | **String** | The ID of the case template. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::OutputCaseTemplate**](OutputCaseTemplate.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_case_template

> update_case_template(case_template_id, input_case_template, x_organisation)
Update a case template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**case_template_id** | **String** | The ID of the case template. | [required] |
**input_case_template** | [**InputCaseTemplate**](InputCaseTemplate.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

