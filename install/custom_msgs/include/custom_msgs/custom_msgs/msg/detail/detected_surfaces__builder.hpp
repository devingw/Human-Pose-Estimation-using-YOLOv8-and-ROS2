// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from custom_msgs:msg/DetectedSurfaces.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_MSGS__MSG__DETAIL__DETECTED_SURFACES__BUILDER_HPP_
#define CUSTOM_MSGS__MSG__DETAIL__DETECTED_SURFACES__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "custom_msgs/msg/detail/detected_surfaces__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace custom_msgs
{

namespace msg
{

namespace builder
{

class Init_DetectedSurfaces_width
{
public:
  explicit Init_DetectedSurfaces_width(::custom_msgs::msg::DetectedSurfaces & msg)
  : msg_(msg)
  {}
  ::custom_msgs::msg::DetectedSurfaces width(::custom_msgs::msg::DetectedSurfaces::_width_type arg)
  {
    msg_.width = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_msgs::msg::DetectedSurfaces msg_;
};

class Init_DetectedSurfaces_height
{
public:
  explicit Init_DetectedSurfaces_height(::custom_msgs::msg::DetectedSurfaces & msg)
  : msg_(msg)
  {}
  Init_DetectedSurfaces_width height(::custom_msgs::msg::DetectedSurfaces::_height_type arg)
  {
    msg_.height = std::move(arg);
    return Init_DetectedSurfaces_width(msg_);
  }

private:
  ::custom_msgs::msg::DetectedSurfaces msg_;
};

class Init_DetectedSurfaces_position
{
public:
  explicit Init_DetectedSurfaces_position(::custom_msgs::msg::DetectedSurfaces & msg)
  : msg_(msg)
  {}
  Init_DetectedSurfaces_height position(::custom_msgs::msg::DetectedSurfaces::_position_type arg)
  {
    msg_.position = std::move(arg);
    return Init_DetectedSurfaces_height(msg_);
  }

private:
  ::custom_msgs::msg::DetectedSurfaces msg_;
};

class Init_DetectedSurfaces_surface_id
{
public:
  Init_DetectedSurfaces_surface_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DetectedSurfaces_position surface_id(::custom_msgs::msg::DetectedSurfaces::_surface_id_type arg)
  {
    msg_.surface_id = std::move(arg);
    return Init_DetectedSurfaces_position(msg_);
  }

private:
  ::custom_msgs::msg::DetectedSurfaces msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_msgs::msg::DetectedSurfaces>()
{
  return custom_msgs::msg::builder::Init_DetectedSurfaces_surface_id();
}

}  // namespace custom_msgs

#endif  // CUSTOM_MSGS__MSG__DETAIL__DETECTED_SURFACES__BUILDER_HPP_
