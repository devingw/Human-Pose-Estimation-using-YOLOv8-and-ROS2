// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from custom_msgs:msg/PoseKeypoint.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_MSGS__MSG__DETAIL__POSE_KEYPOINT__STRUCT_HPP_
#define CUSTOM_MSGS__MSG__DETAIL__POSE_KEYPOINT__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__custom_msgs__msg__PoseKeypoint __attribute__((deprecated))
#else
# define DEPRECATED__custom_msgs__msg__PoseKeypoint __declspec(deprecated)
#endif

namespace custom_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct PoseKeypoint_
{
  using Type = PoseKeypoint_<ContainerAllocator>;

  explicit PoseKeypoint_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->id = 0ll;
      this->x = 0.0f;
      this->y = 0.0f;
      this->confidence = 0.0f;
    }
  }

  explicit PoseKeypoint_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->id = 0ll;
      this->x = 0.0f;
      this->y = 0.0f;
      this->confidence = 0.0f;
    }
  }

  // field types and members
  using _id_type =
    int64_t;
  _id_type id;
  using _x_type =
    float;
  _x_type x;
  using _y_type =
    float;
  _y_type y;
  using _confidence_type =
    float;
  _confidence_type confidence;

  // setters for named parameter idiom
  Type & set__id(
    const int64_t & _arg)
  {
    this->id = _arg;
    return *this;
  }
  Type & set__x(
    const float & _arg)
  {
    this->x = _arg;
    return *this;
  }
  Type & set__y(
    const float & _arg)
  {
    this->y = _arg;
    return *this;
  }
  Type & set__confidence(
    const float & _arg)
  {
    this->confidence = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    custom_msgs::msg::PoseKeypoint_<ContainerAllocator> *;
  using ConstRawPtr =
    const custom_msgs::msg::PoseKeypoint_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<custom_msgs::msg::PoseKeypoint_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<custom_msgs::msg::PoseKeypoint_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      custom_msgs::msg::PoseKeypoint_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<custom_msgs::msg::PoseKeypoint_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      custom_msgs::msg::PoseKeypoint_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<custom_msgs::msg::PoseKeypoint_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<custom_msgs::msg::PoseKeypoint_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<custom_msgs::msg::PoseKeypoint_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__custom_msgs__msg__PoseKeypoint
    std::shared_ptr<custom_msgs::msg::PoseKeypoint_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__custom_msgs__msg__PoseKeypoint
    std::shared_ptr<custom_msgs::msg::PoseKeypoint_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const PoseKeypoint_ & other) const
  {
    if (this->id != other.id) {
      return false;
    }
    if (this->x != other.x) {
      return false;
    }
    if (this->y != other.y) {
      return false;
    }
    if (this->confidence != other.confidence) {
      return false;
    }
    return true;
  }
  bool operator!=(const PoseKeypoint_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct PoseKeypoint_

// alias to use template instance with default allocator
using PoseKeypoint =
  custom_msgs::msg::PoseKeypoint_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace custom_msgs

#endif  // CUSTOM_MSGS__MSG__DETAIL__POSE_KEYPOINT__STRUCT_HPP_
