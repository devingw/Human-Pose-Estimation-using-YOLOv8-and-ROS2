// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from custom_msgs:msg/DetectedObjects.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_MSGS__MSG__DETAIL__DETECTED_OBJECTS__STRUCT_H_
#define CUSTOM_MSGS__MSG__DETAIL__DETECTED_OBJECTS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'position'
#include "geometry_msgs/msg/detail/point__struct.h"

/// Struct defined in msg/DetectedObjects in the package custom_msgs.
typedef struct custom_msgs__msg__DetectedObjects
{
  uint32_t object_id;
  geometry_msgs__msg__Point position;
  float height;
  float width;
  float thickness;
} custom_msgs__msg__DetectedObjects;

// Struct for a sequence of custom_msgs__msg__DetectedObjects.
typedef struct custom_msgs__msg__DetectedObjects__Sequence
{
  custom_msgs__msg__DetectedObjects * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} custom_msgs__msg__DetectedObjects__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // CUSTOM_MSGS__MSG__DETAIL__DETECTED_OBJECTS__STRUCT_H_
