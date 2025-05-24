# \UserApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UserApi.md#create_user) | **POST** /v1/user | Create a new user
[**delete_user**](UserApi.md#delete_user) | **DELETE** /v1/user/{user_id}/force | Delete a user (force)
[**get_current_user**](UserApi.md#get_current_user) | **GET** /v1/user/current | Get current authenticated user's details
[**get_user_api_key**](UserApi.md#get_user_api_key) | **GET** /v1/user/{user_id}/key | Get user's API key
[**get_user_by_id**](UserApi.md#get_user_by_id) | **GET** /v1/user/{user_id} | Get user details by ID
[**remove_user_api_key**](UserApi.md#remove_user_api_key) | **DELETE** /v1/user/{user_id}/key | Remove user's API key
[**renew_user_api_key**](UserApi.md#renew_user_api_key) | **POST** /v1/user/{user_id}/key/renew | Renew user's API key
[**set_user_organisations**](UserApi.md#set_user_organisations) | **PUT** /v1/user/{user_id}/organisations | Set user's organisations and profiles
[**set_user_password**](UserApi.md#set_user_password) | **POST** /v1/user/{user_id}/password/set | Set a user's password
[**update_user**](UserApi.md#update_user) | **PATCH** /v1/user/{user_id} | Update user details



## create_user

> models::OutputUser create_user(input_user, x_organisation)
Create a new user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**input_user** | [**InputUser**](InputUser.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::OutputUser**](OutputUser.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(user_id, organisation, x_organisation)
Delete a user (force)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**organisation** | Option<**String**> |  |  |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> models::OutputUser get_current_user(x_organisation)
Get current authenticated user's details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::OutputUser**](OutputUser.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_api_key

> String get_user_api_key(user_id, x_organisation)
Get user's API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_id

> models::OutputUser get_user_by_id(user_id, x_organisation)
Get user details by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::OutputUser**](OutputUser.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_api_key

> remove_user_api_key(user_id, x_organisation)
Remove user's API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## renew_user_api_key

> String renew_user_api_key(user_id, x_organisation)
Renew user's API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_organisations

> models::SetUserOrganisations200Response set_user_organisations(user_id, set_user_organisations_request, x_organisation)
Set user's organisations and profiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**set_user_organisations_request** | [**SetUserOrganisationsRequest**](SetUserOrganisationsRequest.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

[**models::SetUserOrganisations200Response**](setUserOrganisations_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_password

> set_user_password(user_id, set_user_password_request, x_organisation)
Set a user's password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**set_user_password_request** | [**SetUserPasswordRequest**](SetUserPasswordRequest.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> update_user(user_id, input_update_user, x_organisation)
Update user details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of the user. | [required] |
**input_update_user** | [**InputUpdateUser**](InputUpdateUser.md) |  | [required] |
**x_organisation** | Option<**String**> | Specifies the organisation context for the request. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

