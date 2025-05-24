# \CommentApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_alert_comment**](CommentApi.md#create_alert_comment) | **POST** /v1/alert/{alert_id}/comment | Create a comment in an alert
[**create_case_comment**](CommentApi.md#create_case_comment) | **POST** /v1/case/{case_id}/comment | Create a comment in a case
[**delete_comment**](CommentApi.md#delete_comment) | **DELETE** /v1/comment/{comment_id} | Delete a comment
[**update_comment**](CommentApi.md#update_comment) | **PATCH** /v1/comment/{comment_id} | Update a comment



## create_alert_comment

> models::OutputComment create_alert_comment(alert_id, input_comment, x_organisation)
Create a comment in an alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the alert. | [required] |
**input_comment** | [**InputComment**](InputComment.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::OutputComment**](OutputComment.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_case_comment

> models::OutputComment create_case_comment(case_id, input_comment, x_organisation)
Create a comment in a case

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**case_id** | **String** | The ID or number of the case. | [required] |
**input_comment** | [**InputComment**](InputComment.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::OutputComment**](OutputComment.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comment

> delete_comment(comment_id, x_organisation)
Delete a comment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** | The ID of the comment. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_comment

> update_comment(comment_id, input_update_comment, x_organisation)
Update a comment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** | The ID of the comment. | [required] |
**input_update_comment** | [**InputUpdateComment**](InputUpdateComment.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

