# Annotation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**core_colon_comment** | Option<**String**> | A human-readable comment | [optional][default to ]
**core_colon_freq_lower_edge** | Option<**f32**> | The frequency (Hz) of the lower edge of the feature described by this annotation. | [optional]
**core_colon_freq_upper_edge** | Option<**f32**> | The frequency (Hz) of the upper edge of the feature described by this annotation. | [optional]
**core_colon_generator** | Option<**String**> | Human-readable name of the entity that created this annotation. | [optional]
**core_colon_label** | Option<**String**> | A short form human/machine-readable label for the annotation. CAN BE USED TO LABEL CLASSIFIER OUTPUT | [optional]
**core_colon_sample_count** | **i32** | The number of samples that this Segment applies to. | 
**core_colon_sample_start** | **i32** | The sample index at which this Segment takes effect | [default to 0]
**core_colon_uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | RFC-4122 unique identifier. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


