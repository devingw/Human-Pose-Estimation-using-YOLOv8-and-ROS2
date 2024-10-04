// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from custom_msgs:msg/DetectedObjects.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_MSGS__MSG__DETAIL__DETECTED_OBJECTS__BUILDER_HPP_
#define CUSTOM_MSGS__MSG__DETAIL__DETECTED_OBJECTS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "custom_msgs/msg/detail/detected_objects__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace custom_msgs
{

namespace msg
{

namespace builder
{

class Init_DetectedObjects_thickness
{
public:
  explicit Init_DetectedObjects_thickness(::custom_msgs::msg::DetectedObjects & msg)
  : msg_(msg)
  {}
  ::custom_msgs::msg::DetectedObjects thickness(::custom_msgs::msg::DetectedObjects::_thickness_type arg)
  {
    msg_.thickness = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_msgs::msg::DetectedObjects msg_;
};

class Init_DetectedObjects_width
{
public:
  explicit Init_DetectedObjects_width(::custom_msgs::msg::DetectedObjects & msg)
  : msg_(msg)
  {}
  Init_DetectedObjects_thickness width(::custom_msgs::msg::DetectedObjects::_width_type arg)
  {
    msg_.width = std::move(arg);
    return Init_DetectedObjects_thickness(msg_);
  }

private:
  ::custom_msgs::msg::DetectedObjects msg_;
};

class Init_DetectedObjects_height
{
public:
  explicit Init_DetectedObjects_height(::custom_msgs::msg::DetectedObjects & msg)
  : msg_(msg)
  {}
  Init_DetectedObjects_width height(::custom_msgs::msg::DetectedObjects::_height_type arg)
  {
    msg_.height = std::move(arg);
    return Init_DetectedObjects_width(msg_);
  }

private:
  ::custom_msgs::msg::DetectedObjects msg_;
};

class Init_DetectedObjects_position
{
public:
  explicit Init_DetectedObjects_position(::custom_msgs::msg::DetectedObjects & msg)
  : msg_(msg)
  {}
  Init_DetectedObjects_height position(::custom_msgs::msg::DetectedObjects::_position_type arg)
  {
    msg_.position = std::move(arg);
    return Init_DetectedObjects_height(msg_);
  }

private:
  ::custom_msgs::msg::DetectedObjects msg_;
};

class Init_DetectedObjects_object_id
{
public:
  Init_DetectedObjects_object_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DetectedObjects_position object_id(::custom_msgs::msg::DetectedObjects::_object_id_type arg)
  {
    msg_.object_id = std::move(arg);
    return Init_DetectedObjects_position(msg_);
  }

private:
  ::custom_msgs::msg::DetectedObjects msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_msgs::msg::DetectedObjects>()
{
  return custom_msgs::msg::builder::Init_DetectedObjects_object_id();
}

}  // namespace custom_msgs

#endif  // CUSTOM_MSGS__MSG__DETAIL__DETECTED_OBJECTS__BUILDER_HPP_
