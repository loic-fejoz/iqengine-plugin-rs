# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**function_name_get**](DefaultApi.md#function_name_get) | **GET** /{functionName} | Gets the RF Function's custom params in the form of a JSON schema that is only 1 layer deep.
[**function_name_post**](DefaultApi.md#function_name_post) | **POST** /{functionName} | Run the RF Function, using the provided IQ samples and params
[**root_get**](DefaultApi.md#root_get) | **GET** / | Returns the list of RF Functions available at this ip/host, as a list of strings, must match functionName below.



## function_name_get

> crate::models::FunctionNameGet200Response function_name_get(function_name)
Gets the RF Function's custom params in the form of a JSON schema that is only 1 layer deep.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_name** | **String** | Name of RF Function | [required] |

### Return type

[**crate::models::FunctionNameGet200Response**](__functionName__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## function_name_post

> crate::models::FunctionNamePost200Response function_name_post(function_name, function_name_post_request)
Run the RF Function, using the provided IQ samples and params

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**function_name** | **String** | Name of RF Function | [required] |
**function_name_post_request** | Option<[**FunctionNamePostRequest**](FunctionNamePostRequest.md)> | IQ samples and parameters needed to run the RF Function | [required] |

### Return type

[**crate::models::FunctionNamePost200Response**](__functionName__post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## root_get

> Vec<String> root_get()
Returns the list of RF Functions available at this ip/host, as a list of strings, must match functionName below.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

