// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from custom_msgs:msg/DetectedSurfaces.idl
// generated code does not contain a copyright notice

#ifndef CUSTOM_MSGS__MSG__DETAIL__DETECTED_SURFACES__STRUCT_HPP_
#define CUSTOM_MSGS__MSG__DETAIL__DETECTED_SURFACES__STRUCT_HPP_

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
# define DEPRECATED__custom_msgs__msg__DetectedSurfaces __attribute__((deprecated))
#else
# define DEPRECATED__custom_msgs__msg__DetectedSurfaces __declspec(deprecated)
#endif

namespace custom_msgs
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct DetectedSurfaces_
{
  using Type = DetectedSurfaces_<ContainerAllocator>;

  explicit DetectedSurfaces_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : position(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->surface_id = 0ul;
      this->height = 0.0f;
      this->width = 0.0f;
    }
  }

  explicit DetectedSurfaces_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : position(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->surface_id = 0ul;
      this->height = 0.0f;
      this->width = 0.0f;
    }
  }

  // field types and members
  using _surface_id_type =
    uint32_t;
  _surface_id_type surface_id;
  using _position_type =
    geometry_msgs::msg::Point_<ContainerAllocator>;
  _position_type position;
  using _height_type =
    float;
  _height_type height;
  using _width_type =
    float;
  _width_type width;

  // setters for named parameter idiom
  Type & set__surface_id(
    const uint32_t & _arg)
  {
    this->surface_id = _arg;
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

  // constant declarations

  // pointer types
  using RawPtr =
    custom_msgs::msg::DetectedSurfaces_<ContainerAllocator> *;
  using ConstRawPtr =
    const custom_msgs::msg::DetectedSurfaces_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<custom_msgs::msg::DetectedSurfaces_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<custom_msgs::msg::DetectedSurfaces_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      custom_msgs::msg::DetectedSurfaces_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<custom_msgs::msg::DetectedSurfaces_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      custom_msgs::msg::DetectedSurfaces_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<custom_msgs::msg::DetectedSurfaces_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<custom_msgs::msg::DetectedSurfaces_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<custom_msgs::msg::DetectedSurfaces_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__custom_msgs__msg__DetectedSurfaces
    std::shared_ptr<custom_msgs::msg::DetectedSurfaces_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__custom_msgs__msg__DetectedSurfaces
    std::shared_ptr<custom_msgs::msg::DetectedSurfaces_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const DetectedSurfaces_ & other) const
  {
    if (this->surface_id != other.surface_id) {
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
    return true;
  }
  bool operator!=(const DetectedSurfaces_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct DetectedSurfaces_

// alias to use template instance with default allocator
using DetectedSurfaces =
  custom_msgs::msg::DetectedSurfaces_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace custom_msgs

#endif  // CUSTOM_MSGS__MSG__DETAIL__DETECTED_SURFACES__STRUCT_HPP_
