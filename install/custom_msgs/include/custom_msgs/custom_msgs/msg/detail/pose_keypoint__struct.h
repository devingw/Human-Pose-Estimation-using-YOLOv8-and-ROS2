// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from custom_msgs:msg/PoseKeypoint.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_MSGS__MSG__DETAIL__POSE_KEYPOINT__STRUCT_H_
#define CUSTOM_MSGS__MSG__DETAIL__POSE_KEYPOINT__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in msg/PoseKeypoint in the package custom_msgs.
typedef struct custom_msgs__msg__PoseKeypoint
{
  int64_t id;
  float x;
  float y;
  float confidence;
} custom_msgs__msg__PoseKeypoint;

// Struct for a sequence of custom_msgs__msg__PoseKeypoint.
typedef struct custom_msgs__msg__PoseKeypoint__Sequence
{
  custom_msgs__msg__PoseKeypoint * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} custom_msgs__msg__PoseKeypoint__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // CUSTOM_MSGS__MSG__DETAIL__POSE_KEYPOINT__STRUCT_H_
