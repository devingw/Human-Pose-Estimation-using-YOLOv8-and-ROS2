// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from custom_msgs:msg/DetectedObjects.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_MSGS__MSG__DETAIL__DETECTED_OBJECTS__STRUCT_HPP_
#define CUSTOM_MSGS__MSG__DETAIL__DETECTED_OBJECTS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'position'
#include "geometry_msgs/msg/detail/point__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__custom_msgs__msg__DetectedObjects __attribute__((deprecated))
#else
# define DEPRECATED__custom_msgs__msg__DetectedObjects __declspec(deprecated)
#endif

namespace custom_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct DetectedObjects_
{
  using Type = DetectedObjects_<ContainerAllocator>;

  explicit DetectedObjects_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : position(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->object_id = 0ul;
      this->height = 0.0f;
      this->width = 0.0f;
      this->thickness = 0.0f;
    }
  }

  explicit DetectedObjects_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : position(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->object_id = 0ul;
      this->height = 0.0f;
      this->width = 0.0f;
      this->thickness = 0.0f;
    }
  }

  // field types and members
  using _object_id_type =
    uint32_t;
  _object_id_type object_id;
  using _position_type =
    geometry_msgs::msg::Point_<ContainerAllocator>;
  _position_type position;
  using _height_type =
    float;
  _height_type height;
  using _width_type =
    float;
  _width_type width;
  using _thickness_type =
    float;
  _thickness_type thickness;

  // setters for named parameter idiom
  Type & set__object_id(
    const uint32_t & _arg)
  {
    this->object_id = _arg;
    return *this;
  }
  Type & set__position(
    const geometry_msgs::msg::Point_<ContainerAllocator> & _arg)
  {
    this->position = _arg;
    return *this;
  }
  Type & set__height(
    const float & _arg)
  {
    this->height = _arg;
    return *this;
  }
  Type & set__width(
    const float & _arg)
  {
    this->width = _arg;
    return *this;
  }
  Type & set__thickness(
    const float & _arg)
  {
    this->thickness = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    custom_msgs::msg::DetectedObjects_<ContainerAllocator> *;
  using ConstRawPtr =
    const custom_msgs::msg::DetectedObjects_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<custom_msgs::msg::DetectedObjects_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<custom_msgs::msg::DetectedObjects_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      custom_msgs::msg::DetectedObjects_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<custom_msgs::msg::DetectedObjects_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      custom_msgs::msg::DetectedObjects_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<custom_msgs::msg::DetectedObjects_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<custom_msgs::msg::DetectedObjects_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<custom_msgs::msg::DetectedObjects_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__custom_msgs__msg__DetectedObjects
    std::shared_ptr<custom_msgs::msg::DetectedObjects_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__custom_msgs__msg__DetectedObjects
    std::shared_ptr<custom_msgs::msg::DetectedObjects_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const DetectedObjects_ & other) const
  {
    if (this->object_id != other.object_id) {
      return false;
    }
    if (this->position != other.position) {
      return false;
    }
    if (this->height != other.height) {
      return false;
    }
    if (this->width != other.width) {
      return false;
    }
    if (this->thickness != other.thickness) {
      return false;
    }
    return true;
  }
  bool operator!=(const DetectedObjects_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct DetectedObjects_

// alias to use template instance with default allocator
using DetectedObjects =
  custom_msgs::msg::DetectedObjects_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace custom_msgs

#endif  // CUSTOM_MSGS__MSG__DETAIL__DETECTED_OBJECTS__STRUCT_HPP_
