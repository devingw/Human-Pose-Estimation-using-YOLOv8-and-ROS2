// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from custom_msgs:msg/Yolov8Segmentation.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_MSGS__MSG__DETAIL__YOLOV8_SEGMENTATION__BUILDER_HPP_
#define CUSTOM_MSGS__MSG__DETAIL__YOLOV8_SEGMENTATION__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "custom_msgs/msg/detail/yolov8_segmentation__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace custom_msgs
{

namespace msg
{

namespace builder
{

class Init_Yolov8Segmentation_yolov8_segmentation
{
public:
  explicit Init_Yolov8Segmentation_yolov8_segmentation(::custom_msgs::msg::Yolov8Segmentation & msg)
  : msg_(msg)
  {}
  ::custom_msgs::msg::Yolov8Segmentation yolov8_segmentation(::custom_msgs::msg::Yolov8Segmentation::_yolov8_segmentation_type arg)
  {
    msg_.yolov8_segmentation = std::move(arg);
    return std::move(msg_);
  }

private:
  ::custom_msgs::msg::Yolov8Segmentation msg_;
};

class Init_Yolov8Segmentation_header
{
public:
  Init_Yolov8Segmentation_header()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Yolov8Segmentation_yolov8_segmentation header(::custom_msgs::msg::Yolov8Segmentation::_header_type arg)
  {
    msg_.header = std::move(arg);
    return Init_Yolov8Segmentation_yolov8_segmentation(msg_);
  }

private:
  ::custom_msgs::msg::Yolov8Segmentation msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::custom_msgs::msg::Yolov8Segmentation>()
{
  return custom_msgs::msg::builder::Init_Yolov8Segmentation_header();
}

}  // namespace custom_msgs

#endif  // CUSTOM_MSGS__MSG__DETAIL__YOLOV8_SEGMENTATION__BUILDER_HPP_
