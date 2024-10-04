pub mod rmw {
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[link(name = "custom_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__DetectedSurfaces() -> *const std::os::raw::c_void;
}

#[link(name = "custom_msgs__rosidl_generator_c")]
extern "C" {
    fn custom_msgs__msg__DetectedSurfaces__init(msg: *mut DetectedSurfaces) -> bool;
    fn custom_msgs__msg__DetectedSurfaces__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectedSurfaces>, size: usize) -> bool;
    fn custom_msgs__msg__DetectedSurfaces__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectedSurfaces>);
    fn custom_msgs__msg__DetectedSurfaces__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectedSurfaces>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectedSurfaces>) -> bool;
}

// Corresponds to custom_msgs__msg__DetectedSurfaces
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedSurfaces {
    pub surface_id: u32,
    pub position: geometry_msgs::msg::rmw::Point,
    pub height: f32,
    pub width: f32,
}



impl Default for DetectedSurfaces {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_msgs__msg__DetectedSurfaces__init(&mut msg as *mut _) {
        panic!("Call to custom_msgs__msg__DetectedSurfaces__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectedSurfaces {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__DetectedSurfaces__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__DetectedSurfaces__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__DetectedSurfaces__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectedSurfaces {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectedSurfaces where Self: Sized {
  const TYPE_NAME: &'static str = "custom_msgs/msg/DetectedSurfaces";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__DetectedSurfaces() }
  }
}


#[link(name = "custom_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__DetectedObjects() -> *const std::os::raw::c_void;
}

#[link(name = "custom_msgs__rosidl_generator_c")]
extern "C" {
    fn custom_msgs__msg__DetectedObjects__init(msg: *mut DetectedObjects) -> bool;
    fn custom_msgs__msg__DetectedObjects__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectedObjects>, size: usize) -> bool;
    fn custom_msgs__msg__DetectedObjects__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectedObjects>);
    fn custom_msgs__msg__DetectedObjects__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectedObjects>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectedObjects>) -> bool;
}

// Corresponds to custom_msgs__msg__DetectedObjects
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedObjects {
    pub object_id: u32,
    pub position: geometry_msgs::msg::rmw::Point,
    pub height: f32,
    pub width: f32,
    pub thickness: f32,
}



impl Default for DetectedObjects {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_msgs__msg__DetectedObjects__init(&mut msg as *mut _) {
        panic!("Call to custom_msgs__msg__DetectedObjects__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectedObjects {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__DetectedObjects__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__DetectedObjects__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__DetectedObjects__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectedObjects {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectedObjects where Self: Sized {
  const TYPE_NAME: &'static str = "custom_msgs/msg/DetectedObjects";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__DetectedObjects() }
  }
}


#[link(name = "custom_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__InferenceResult() -> *const std::os::raw::c_void;
}

#[link(name = "custom_msgs__rosidl_generator_c")]
extern "C" {
    fn custom_msgs__msg__InferenceResult__init(msg: *mut InferenceResult) -> bool;
    fn custom_msgs__msg__InferenceResult__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InferenceResult>, size: usize) -> bool;
    fn custom_msgs__msg__InferenceResult__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InferenceResult>);
    fn custom_msgs__msg__InferenceResult__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InferenceResult>, out_seq: *mut rosidl_runtime_rs::Sequence<InferenceResult>) -> bool;
}

// Corresponds to custom_msgs__msg__InferenceResult
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InferenceResult {
    pub class_name: rosidl_runtime_rs::String,
    pub top: i64,
    pub left: i64,
    pub bottom: i64,
    pub right: i64,
    pub box_width: i64,
    pub box_height: i64,
    pub x: f64,
    pub y: f64,
}



impl Default for InferenceResult {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_msgs__msg__InferenceResult__init(&mut msg as *mut _) {
        panic!("Call to custom_msgs__msg__InferenceResult__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InferenceResult {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__InferenceResult__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__InferenceResult__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__InferenceResult__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InferenceResult {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InferenceResult where Self: Sized {
  const TYPE_NAME: &'static str = "custom_msgs/msg/InferenceResult";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__InferenceResult() }
  }
}


#[link(name = "custom_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__Yolov8Inference() -> *const std::os::raw::c_void;
}

#[link(name = "custom_msgs__rosidl_generator_c")]
extern "C" {
    fn custom_msgs__msg__Yolov8Inference__init(msg: *mut Yolov8Inference) -> bool;
    fn custom_msgs__msg__Yolov8Inference__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Yolov8Inference>, size: usize) -> bool;
    fn custom_msgs__msg__Yolov8Inference__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Yolov8Inference>);
    fn custom_msgs__msg__Yolov8Inference__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Yolov8Inference>, out_seq: *mut rosidl_runtime_rs::Sequence<Yolov8Inference>) -> bool;
}

// Corresponds to custom_msgs__msg__Yolov8Inference
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Yolov8Inference {
    pub header: std_msgs::msg::rmw::Header,
    pub yolov8_inference: rosidl_runtime_rs::Sequence<crate::msg::rmw::InferenceResult>,
}



impl Default for Yolov8Inference {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_msgs__msg__Yolov8Inference__init(&mut msg as *mut _) {
        panic!("Call to custom_msgs__msg__Yolov8Inference__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Yolov8Inference {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__Yolov8Inference__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__Yolov8Inference__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__Yolov8Inference__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Yolov8Inference {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Yolov8Inference where Self: Sized {
  const TYPE_NAME: &'static str = "custom_msgs/msg/Yolov8Inference";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__Yolov8Inference() }
  }
}


#[link(name = "custom_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__PoseKeypoint() -> *const std::os::raw::c_void;
}

#[link(name = "custom_msgs__rosidl_generator_c")]
extern "C" {
    fn custom_msgs__msg__PoseKeypoint__init(msg: *mut PoseKeypoint) -> bool;
    fn custom_msgs__msg__PoseKeypoint__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PoseKeypoint>, size: usize) -> bool;
    fn custom_msgs__msg__PoseKeypoint__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PoseKeypoint>);
    fn custom_msgs__msg__PoseKeypoint__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PoseKeypoint>, out_seq: *mut rosidl_runtime_rs::Sequence<PoseKeypoint>) -> bool;
}

// Corresponds to custom_msgs__msg__PoseKeypoint
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PoseKeypoint {
    pub id: i64,
    pub x: f32,
    pub y: f32,
    pub confidence: f32,
}



impl Default for PoseKeypoint {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_msgs__msg__PoseKeypoint__init(&mut msg as *mut _) {
        panic!("Call to custom_msgs__msg__PoseKeypoint__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PoseKeypoint {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__PoseKeypoint__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__PoseKeypoint__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__PoseKeypoint__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PoseKeypoint {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PoseKeypoint where Self: Sized {
  const TYPE_NAME: &'static str = "custom_msgs/msg/PoseKeypoint";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__PoseKeypoint() }
  }
}


#[link(name = "custom_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__PoseResult() -> *const std::os::raw::c_void;
}

#[link(name = "custom_msgs__rosidl_generator_c")]
extern "C" {
    fn custom_msgs__msg__PoseResult__init(msg: *mut PoseResult) -> bool;
    fn custom_msgs__msg__PoseResult__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PoseResult>, size: usize) -> bool;
    fn custom_msgs__msg__PoseResult__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PoseResult>);
    fn custom_msgs__msg__PoseResult__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PoseResult>, out_seq: *mut rosidl_runtime_rs::Sequence<PoseResult>) -> bool;
}

// Corresponds to custom_msgs__msg__PoseResult
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PoseResult {
    pub header: std_msgs::msg::rmw::Header,
    pub keypoints: rosidl_runtime_rs::Sequence<crate::msg::rmw::PoseKeypoint>,
}



impl Default for PoseResult {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_msgs__msg__PoseResult__init(&mut msg as *mut _) {
        panic!("Call to custom_msgs__msg__PoseResult__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PoseResult {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__PoseResult__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__PoseResult__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__PoseResult__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PoseResult {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PoseResult where Self: Sized {
  const TYPE_NAME: &'static str = "custom_msgs/msg/PoseResult";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__PoseResult() }
  }
}


#[link(name = "custom_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__SegmentationResult() -> *const std::os::raw::c_void;
}

#[link(name = "custom_msgs__rosidl_generator_c")]
extern "C" {
    fn custom_msgs__msg__SegmentationResult__init(msg: *mut SegmentationResult) -> bool;
    fn custom_msgs__msg__SegmentationResult__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SegmentationResult>, size: usize) -> bool;
    fn custom_msgs__msg__SegmentationResult__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SegmentationResult>);
    fn custom_msgs__msg__SegmentationResult__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SegmentationResult>, out_seq: *mut rosidl_runtime_rs::Sequence<SegmentationResult>) -> bool;
}

// Corresponds to custom_msgs__msg__SegmentationResult
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SegmentationResult {
    pub class_id: i64,
    pub class_name: rosidl_runtime_rs::String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}



impl Default for SegmentationResult {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_msgs__msg__SegmentationResult__init(&mut msg as *mut _) {
        panic!("Call to custom_msgs__msg__SegmentationResult__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SegmentationResult {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__SegmentationResult__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__SegmentationResult__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__SegmentationResult__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SegmentationResult {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SegmentationResult where Self: Sized {
  const TYPE_NAME: &'static str = "custom_msgs/msg/SegmentationResult";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__SegmentationResult() }
  }
}


#[link(name = "custom_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__Yolov8Segmentation() -> *const std::os::raw::c_void;
}

#[link(name = "custom_msgs__rosidl_generator_c")]
extern "C" {
    fn custom_msgs__msg__Yolov8Segmentation__init(msg: *mut Yolov8Segmentation) -> bool;
    fn custom_msgs__msg__Yolov8Segmentation__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Yolov8Segmentation>, size: usize) -> bool;
    fn custom_msgs__msg__Yolov8Segmentation__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Yolov8Segmentation>);
    fn custom_msgs__msg__Yolov8Segmentation__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Yolov8Segmentation>, out_seq: *mut rosidl_runtime_rs::Sequence<Yolov8Segmentation>) -> bool;
}

// Corresponds to custom_msgs__msg__Yolov8Segmentation
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Yolov8Segmentation {
    pub header: std_msgs::msg::rmw::Header,
    pub yolov8_segmentation: rosidl_runtime_rs::Sequence<crate::msg::rmw::SegmentationResult>,
}



impl Default for Yolov8Segmentation {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !custom_msgs__msg__Yolov8Segmentation__init(&mut msg as *mut _) {
        panic!("Call to custom_msgs__msg__Yolov8Segmentation__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Yolov8Segmentation {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__Yolov8Segmentation__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__Yolov8Segmentation__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { custom_msgs__msg__Yolov8Segmentation__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Yolov8Segmentation {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Yolov8Segmentation where Self: Sized {
  const TYPE_NAME: &'static str = "custom_msgs/msg/Yolov8Segmentation";
  fn get_type_support() -> *const std::os::raw::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__custom_msgs__msg__Yolov8Segmentation() }
  }
}


}  // mod rmw


#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedSurfaces {
    pub surface_id: u32,
    pub position: geometry_msgs::msg::Point,
    pub height: f32,
    pub width: f32,
}



impl Default for DetectedSurfaces {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::DetectedSurfaces::default())
  }
}

impl rosidl_runtime_rs::Message for DetectedSurfaces {
  type RmwMsg = crate::msg::rmw::DetectedSurfaces;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        surface_id: msg.surface_id,
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.position)).into_owned(),
        height: msg.height,
        width: msg.width,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      surface_id: msg.surface_id,
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.position)).into_owned(),
      height: msg.height,
      width: msg.width,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      surface_id: msg.surface_id,
      position: geometry_msgs::msg::Point::from_rmw_message(msg.position),
      height: msg.height,
      width: msg.width,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedObjects {
    pub object_id: u32,
    pub position: geometry_msgs::msg::Point,
    pub height: f32,
    pub width: f32,
    pub thickness: f32,
}



impl Default for DetectedObjects {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::DetectedObjects::default())
  }
}

impl rosidl_runtime_rs::Message for DetectedObjects {
  type RmwMsg = crate::msg::rmw::DetectedObjects;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        object_id: msg.object_id,
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.position)).into_owned(),
        height: msg.height,
        width: msg.width,
        thickness: msg.thickness,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      object_id: msg.object_id,
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.position)).into_owned(),
      height: msg.height,
      width: msg.width,
      thickness: msg.thickness,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      object_id: msg.object_id,
      position: geometry_msgs::msg::Point::from_rmw_message(msg.position),
      height: msg.height,
      width: msg.width,
      thickness: msg.thickness,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InferenceResult {
    pub class_name: std::string::String,
    pub top: i64,
    pub left: i64,
    pub bottom: i64,
    pub right: i64,
    pub box_width: i64,
    pub box_height: i64,
    pub x: f64,
    pub y: f64,
}



impl Default for InferenceResult {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::InferenceResult::default())
  }
}

impl rosidl_runtime_rs::Message for InferenceResult {
  type RmwMsg = crate::msg::rmw::InferenceResult;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        class_name: msg.class_name.as_str().into(),
        top: msg.top,
        left: msg.left,
        bottom: msg.bottom,
        right: msg.right,
        box_width: msg.box_width,
        box_height: msg.box_height,
        x: msg.x,
        y: msg.y,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        class_name: msg.class_name.as_str().into(),
      top: msg.top,
      left: msg.left,
      bottom: msg.bottom,
      right: msg.right,
      box_width: msg.box_width,
      box_height: msg.box_height,
      x: msg.x,
      y: msg.y,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      class_name: msg.class_name.to_string(),
      top: msg.top,
      left: msg.left,
      bottom: msg.bottom,
      right: msg.right,
      box_width: msg.box_width,
      box_height: msg.box_height,
      x: msg.x,
      y: msg.y,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Yolov8Inference {
    pub header: std_msgs::msg::Header,
    pub yolov8_inference: Vec<crate::msg::InferenceResult>,
}



impl Default for Yolov8Inference {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::Yolov8Inference::default())
  }
}

impl rosidl_runtime_rs::Message for Yolov8Inference {
  type RmwMsg = crate::msg::rmw::Yolov8Inference;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        yolov8_inference: msg.yolov8_inference
          .into_iter()
          .map(|elem| crate::msg::InferenceResult::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        yolov8_inference: msg.yolov8_inference
          .iter()
          .map(|elem| crate::msg::InferenceResult::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      yolov8_inference: msg.yolov8_inference
          .into_iter()
          .map(crate::msg::InferenceResult::from_rmw_message)
          .collect(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PoseKeypoint {
    pub id: i64,
    pub x: f32,
    pub y: f32,
    pub confidence: f32,
}



impl Default for PoseKeypoint {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::PoseKeypoint::default())
  }
}

impl rosidl_runtime_rs::Message for PoseKeypoint {
  type RmwMsg = crate::msg::rmw::PoseKeypoint;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        x: msg.x,
        y: msg.y,
        confidence: msg.confidence,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      x: msg.x,
      y: msg.y,
      confidence: msg.confidence,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      x: msg.x,
      y: msg.y,
      confidence: msg.confidence,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PoseResult {
    pub header: std_msgs::msg::Header,
    pub keypoints: Vec<crate::msg::PoseKeypoint>,
}



impl Default for PoseResult {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::PoseResult::default())
  }
}

impl rosidl_runtime_rs::Message for PoseResult {
  type RmwMsg = crate::msg::rmw::PoseResult;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        keypoints: msg.keypoints
          .into_iter()
          .map(|elem| crate::msg::PoseKeypoint::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        keypoints: msg.keypoints
          .iter()
          .map(|elem| crate::msg::PoseKeypoint::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      keypoints: msg.keypoints
          .into_iter()
          .map(crate::msg::PoseKeypoint::from_rmw_message)
          .collect(),
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SegmentationResult {
    pub class_id: i64,
    pub class_name: std::string::String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}



impl Default for SegmentationResult {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::SegmentationResult::default())
  }
}

impl rosidl_runtime_rs::Message for SegmentationResult {
  type RmwMsg = crate::msg::rmw::SegmentationResult;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        class_id: msg.class_id,
        class_name: msg.class_name.as_str().into(),
        x: msg.x,
        y: msg.y,
        width: msg.width,
        height: msg.height,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      class_id: msg.class_id,
        class_name: msg.class_name.as_str().into(),
      x: msg.x,
      y: msg.y,
      width: msg.width,
      height: msg.height,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      class_id: msg.class_id,
      class_name: msg.class_name.to_string(),
      x: msg.x,
      y: msg.y,
      width: msg.width,
      height: msg.height,
    }
  }
}


#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Yolov8Segmentation {
    pub header: std_msgs::msg::Header,
    pub yolov8_segmentation: Vec<crate::msg::SegmentationResult>,
}



impl Default for Yolov8Segmentation {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(crate::msg::rmw::Yolov8Segmentation::default())
  }
}

impl rosidl_runtime_rs::Message for Yolov8Segmentation {
  type RmwMsg = crate::msg::rmw::Yolov8Segmentation;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        yolov8_segmentation: msg.yolov8_segmentation
          .into_iter()
          .map(|elem| crate::msg::SegmentationResult::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        yolov8_segmentation: msg.yolov8_segmentation
          .iter()
          .map(|elem| crate::msg::SegmentationResult::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      yolov8_segmentation: msg.yolov8_segmentation
          .into_iter()
          .map(crate::msg::SegmentationResult::from_rmw_message)
          .collect(),
    }
  }
}


