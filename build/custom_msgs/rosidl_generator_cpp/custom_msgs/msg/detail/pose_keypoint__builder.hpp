// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from custom_msgs:msg/PoseKeypoint.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_MSGS__MSG__DETAIL__POSE_KEYPOINT__BUILDER_HPP_
#define CUSTOM_MSGS__MSG__DETAIL__POSE_KEYPOINT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "custom_msgs/msg/detail/pose_keypoint__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace custom_msgs
{

namespace msg
{

namespace builder
{

class Init_PoseKeypoint_confidence
{
public:
  explicit Init_PoseKeypoint_confidence(::custom_msgs::msg::PoseKeypoint & msg)
  : msg_(msg)
  {}
  ::custom_msgs::msg::PoseKeypoint confidence(::custom_msgs::msg::PoseKeypoint::_confidence_type arg)
  {
    msg_.confidence = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_msgs::msg::PoseKeypoint msg_;
};

class Init_PoseKeypoint_y
{
public:
  explicit Init_PoseKeypoint_y(::custom_msgs::msg::PoseKeypoint & msg)
  : msg_(msg)
  {}
  Init_PoseKeypoint_confidence y(::custom_msgs::msg::PoseKeypoint::_y_type arg)
  {
    msg_.y = std::move(arg);
    return Init_PoseKeypoint_confidence(msg_);
  }

private:
  ::custom_msgs::msg::PoseKeypoint msg_;
};

class Init_PoseKeypoint_x
{
public:
  explicit Init_PoseKeypoint_x(::custom_msgs::msg::PoseKeypoint & msg)
  : msg_(msg)
  {}
  Init_PoseKeypoint_y x(::custom_msgs::msg::PoseKeypoint::_x_type arg)
  {
    msg_.x = std::move(arg);
    return Init_PoseKeypoint_y(msg_);
  }

private:
  ::custom_msgs::msg::PoseKeypoint msg_;
};

class Init_PoseKeypoint_id
{
public:
  Init_PoseKeypoint_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PoseKeypoint_x id(::custom_msgs::msg::PoseKeypoint::_id_type arg)
  {
    msg_.id = std::move(arg);
    return Init_PoseKeypoint_x(msg_);
  }

private:
  ::custom_msgs::msg::PoseKeypoint msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_msgs::msg::PoseKeypoint>()
{
  return custom_msgs::msg::builder::Init_PoseKeypoint_id();
}

}  // namespace custom_msgs

#endif  // CUSTOM_MSGS__MSG__DETAIL__POSE_KEYPOINT__BUILDER_HPP_
