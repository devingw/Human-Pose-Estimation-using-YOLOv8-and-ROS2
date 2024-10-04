// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from custom_msgs:msg/DetectedSurfaces.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_MSGS__MSG__DETAIL__DETECTED_SURFACES__STRUCT_H_
#define CUSTOM_MSGS__MSG__DETAIL__DETECTED_SURFACES__STRUCT_H_

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

/// Struct defined in msg/DetectedSurfaces in the package custom_msgs.
typedef struct custom_msgs__msg__DetectedSurfaces
{
  uint32_t surface_id;
  geometry_msgs__msg__Point position;
  float height;
  float width;
} custom_msgs__msg__DetectedSurfaces;

// Struct for a sequence of custom_msgs__msg__DetectedSurfaces.
typedef struct custom_msgs__msg__DetectedSurfaces__Sequence
{
  custom_msgs__msg__DetectedSurfaces * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} custom_msgs__msg__DetectedSurfaces__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // CUSTOM_MSGS__MSG__DETAIL__DETECTED_SURFACES__STRUCT_H_
