// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from custom_msgs:msg/PoseResult.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_MSGS__MSG__DETAIL__POSE_RESULT__BUILDER_HPP_
#define CUSTOM_MSGS__MSG__DETAIL__POSE_RESULT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "custom_msgs/msg/detail/pose_result__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace custom_msgs
{

namespace msg
{

namespace builder
{

class Init_PoseResult_keypoints
{
public:
  explicit Init_PoseResult_keypoints(::custom_msgs::msg::PoseResult & msg)
  : msg_(msg)
  {}
  ::custom_msgs::msg::PoseResult keypoints(::custom_msgs::msg::PoseResult::_keypoints_type arg)
  {
    msg_.keypoints = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_msgs::msg::PoseResult msg_;
};

class Init_PoseResult_header
{
public:
  Init_PoseResult_header()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PoseResult_keypoints header(::custom_msgs::msg::PoseResult::_header_type arg)
  {
    msg_.header = std::move(arg);
    return Init_PoseResult_keypoints(msg_);
  }

private:
  ::custom_msgs::msg::PoseResult msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_msgs::msg::PoseResult>()
{
  return custom_msgs::msg::builder::Init_PoseResult_header();
}

}  // namespace custom_msgs

#endif  // CUSTOM_MSGS__MSG__DETAIL__POSE_RESULT__BUILDER_HPP_
