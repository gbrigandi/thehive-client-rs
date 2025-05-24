# \AlertApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_alert_attachments**](AlertApi.md#add_alert_attachments) | **POST** /v1/alert/{alert_id}/attachments | Add attachments to an alert
[**bulk_delete_alerts**](AlertApi.md#bulk_delete_alerts) | **POST** /v1/alert/delete/_bulk | Bulk delete alerts
[**bulk_merge_alerts_into_case**](AlertApi.md#bulk_merge_alerts_into_case) | **POST** /v1/alert/merge/_bulk | Bulk merge alerts into a case
[**bulk_update_alerts**](AlertApi.md#bulk_update_alerts) | **PATCH** /v1/alert/_bulk | Bulk update alerts
[**create_alert**](AlertApi.md#create_alert) | **POST** /v1/alert | Create an alert
[**create_alert_observable**](AlertApi.md#create_alert_observable) | **POST** /v1/alert/{alert_id}/observable | Create an observable in an alert
[**create_alert_procedure**](AlertApi.md#create_alert_procedure) | **POST** /v1/alert/{alert_id}/procedure | Create a procedure in an alert
[**delete_alert**](AlertApi.md#delete_alert) | **DELETE** /v1/alert/{alert_id} | Delete an alert
[**delete_alert_attachment**](AlertApi.md#delete_alert_attachment) | **DELETE** /v1/alert/{alert_id}/attachment/{attachment_id} | Delete an alert attachment
[**download_alert_attachment**](AlertApi.md#download_alert_attachment) | **GET** /v1/alert/{alert_id}/attachment/{attachment_id}/download | Download an alert attachment
[**follow_alert**](AlertApi.md#follow_alert) | **POST** /v1/alert/{alert_id}/follow | Follow an alert
[**get_alert_by_id**](AlertApi.md#get_alert_by_id) | **GET** /v1/alert/{alert_id} | Get an alert by ID
[**merge_alert_into_case**](AlertApi.md#merge_alert_into_case) | **POST** /v1/alert/{alert_id}/merge/{case_id} | Merge an alert into an existing case
[**promote_alert_to_case**](AlertApi.md#promote_alert_to_case) | **POST** /v1/alert/{alert_id}/case | Promote an alert to a case
[**unfollow_alert**](AlertApi.md#unfollow_alert) | **POST** /v1/alert/{alert_id}/unfollow | Unfollow an alert
[**update_alert**](AlertApi.md#update_alert) | **PATCH** /v1/alert/{alert_id} | Update an alert



## add_alert_attachments

> models::AddAlertAttachments201Response add_alert_attachments(alert_id, x_organisation, attachments)
Add attachments to an alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the alert. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |
**attachments** | Option<[**Vec<std::path::PathBuf>**](std::path::PathBuf.md)> |  |  |

### Return type

[**models::AddAlertAttachments201Response**](addAlertAttachments_201_response.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_delete_alerts

> bulk_delete_alerts(bulk_delete_alerts_request, x_organisation)
Bulk delete alerts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_delete_alerts_request** | [**BulkDeleteAlertsRequest**](BulkDeleteAlertsRequest.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_merge_alerts_into_case

> models::OutputCase bulk_merge_alerts_into_case(bulk_merge_alerts_into_case_request, x_organisation)
Bulk merge alerts into a case

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_merge_alerts_into_case_request** | [**BulkMergeAlertsIntoCaseRequest**](BulkMergeAlertsIntoCaseRequest.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::OutputCase**](OutputCase.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_update_alerts

> bulk_update_alerts(input_bulk_update_alert, x_organisation)
Bulk update alerts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**input_bulk_update_alert** | [**InputBulkUpdateAlert**](InputBulkUpdateAlert.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_alert

> models::OutputAlert create_alert(input_alert, x_organisation)
Create an alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**input_alert** | [**InputAlert**](InputAlert.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::OutputAlert**](OutputAlert.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_alert_observable

> Vec<models::OutputObservable> create_alert_observable(alert_id, input_observable, x_organisation)
Create an observable in an alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the alert. | [required] |
**input_observable** | [**InputObservable**](InputObservable.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**Vec<models::OutputObservable>**](OutputObservable.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_alert_procedure

> models::OutputProcedure create_alert_procedure(alert_id, input_procedure, x_organisation)
Create a procedure in an alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the alert. | [required] |
**input_procedure** | [**InputProcedure**](InputProcedure.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::OutputProcedure**](OutputProcedure.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alert

> delete_alert(alert_id, x_organisation)
Delete an alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the alert. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alert_attachment

> delete_alert_attachment(alert_id, attachment_id, x_organisation)
Delete an alert attachment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the alert. | [required] |
**attachment_id** | **String** | The ID of the attachment. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_alert_attachment

> download_alert_attachment(alert_id, attachment_id, x_organisation)
Download an alert attachment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the alert. | [required] |
**attachment_id** | **String** | The ID of the attachment. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## follow_alert

> follow_alert(alert_id, x_organisation)
Follow an alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the alert. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alert_by_id

> models::OutputAlert get_alert_by_id(alert_id, x_organisation)
Get an alert by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the alert. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::OutputAlert**](OutputAlert.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_alert_into_case

> models::OutputCase merge_alert_into_case(alert_id, case_id, x_organisation)
Merge an alert into an existing case

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the alert. | [required] |
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


## promote_alert_to_case

> models::OutputCase promote_alert_to_case(alert_id, x_organisation, input_promote_alert)
Promote an alert to a case

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the alert. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |
**input_promote_alert** | Option<[**InputPromoteAlert**](InputPromoteAlert.md)> |  |  |

### Return type

[**models::OutputCase**](OutputCase.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfollow_alert

> unfollow_alert(alert_id, x_organisation)
Unfollow an alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the alert. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_alert

> update_alert(alert_id, input_update_alert, x_organisation)
Update an alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | The ID of the alert. | [required] |
**input_update_alert** | [**InputUpdateAlert**](InputUpdateAlert.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

