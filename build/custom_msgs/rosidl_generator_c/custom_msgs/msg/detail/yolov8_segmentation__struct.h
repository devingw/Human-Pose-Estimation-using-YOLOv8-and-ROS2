// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from custom_msgs:msg/Yolov8Segmentation.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_MSGS__MSG__DETAIL__YOLOV8_SEGMENTATION__STRUCT_H_
#define CUSTOM_MSGS__MSG__DETAIL__YOLOV8_SEGMENTATION__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__struct.h"
// Member 'yolov8_segmentation'
#include "custom_msgs/msg/detail/segmentation_result__struct.h"

/// Struct defined in msg/Yolov8Segmentation in the package custom_msgs.
typedef struct custom_msgs__msg__Yolov8Segmentation
{
  std_msgs__msg__Header header;
  custom_msgs__msg__SegmentationResult__Sequence yolov8_segmentation;
} custom_msgs__msg__Yolov8Segmentation;

// Struct for a sequence of custom_msgs__msg__Yolov8Segmentation.
typedef struct custom_msgs__msg__Yolov8Segmentation__Sequence
{
  custom_msgs__msg__Yolov8Segmentation * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} custom_msgs__msg__Yolov8Segmentation__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // CUSTOM_MSGS__MSG__DETAIL__YOLOV8_SEGMENTATION__STRUCT_H_
