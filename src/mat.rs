use libc::MEMORY_OBJECT_NULL;

use crate::mnn_bind::*;
use std::ffi::CString;
use std::os::raw::c_int;
pub enum DimensionType {
    TENSORFLOW,
    CAFFE,
    CAFFE_C4,
}

pub struct Tensor {
    // ptr: *mut MNN_Tensor,
}
impl Tensor {
    // 创建
    pub fn create(
        shape: Vec<i32>,
        data: *mut ::std::os::raw::c_void,
        dim_type: DimensionType,
    ) -> *mut MNN_Tensor {
        let halide_type = halide_type_t {
            code: 2, // 2代表float类型
            bits: 32,
            lanes: 1,
        };
        let dim_type = match dim_type {
            DimensionType::TENSORFLOW => MNN_Tensor_DimensionType_TENSORFLOW,
            DimensionType::CAFFE => MNN_Tensor_DimensionType_CAFFE,
            DimensionType::CAFFE_C4 => MNN_Tensor_DimensionType_CAFFE_C4,
        };
        unsafe {
            MNN_Tensor_create(
                shape.as_ptr() as *const u8,
                halide_type,
                std::ptr::null_mut(),
                dim_type,
            )
        }
    }
    // 从外部tensor复制

    // 复制到外部tensor
}

impl Drop for Tensor {
    fn drop(&mut self) {
        // unsafe {
        //     MNN_Tensor_destroy(self.ptr);
        // }
    }
}
