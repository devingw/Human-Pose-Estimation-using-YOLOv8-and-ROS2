// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from custom_msgs:msg/PoseResult.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_MSGS__MSG__DETAIL__POSE_RESULT__STRUCT_H_
#define CUSTOM_MSGS__MSG__DETAIL__POSE_RESULT__STRUCT_H_

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
// Member 'keypoints'
#include "custom_msgs/msg/detail/pose_keypoint__struct.h"

/// Struct defined in msg/PoseResult in the package custom_msgs.
typedef struct custom_msgs__msg__PoseResult
{
  std_msgs__msg__Header header;
  custom_msgs__msg__PoseKeypoint__Sequence keypoints;
} custom_msgs__msg__PoseResult;

// Struct for a sequence of custom_msgs__msg__PoseResult.
typedef struct custom_msgs__msg__PoseResult__Sequence
{
  custom_msgs__msg__PoseResult * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} custom_msgs__msg__PoseResult__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // CUSTOM_MSGS__MSG__DETAIL__POSE_RESULT__STRUCT_H_
