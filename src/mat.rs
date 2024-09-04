use libc::MEMORY_OBJECT_NULL;

use crate::mnn_bind::*;
use std::ffi::CString;
use std::os::raw::c_int;
// pub enum DimensionType {
//     TENSORFLOW,
//     CAFFE,
//     CAFFE_C4,
// }

pub struct Tensor {
    pub ptr: *mut MNN_Tensor,
}
impl Tensor {
    // 创建 -- 该函数异常，已废弃
    // pub fn create(
    //     shape: Vec<i32>,
    //     data: *mut ::std::os::raw::c_void,
    //     dim_type: DimensionType,
    // ) -> *mut MNN_Tensor {
    //     let halide_type = halide_type_t {
    //         code: halide_type_code_t_halide_type_float, // 2代表float类型
    //         bits: 32,
    //         lanes: 1,
    //     };
    //     println!("{:?}", halide_type);
    //     let dim_type = match dim_type {
    //         DimensionType::TENSORFLOW => MNN_Tensor_DimensionType_TENSORFLOW,
    //         DimensionType::CAFFE => MNN_Tensor_DimensionType_CAFFE,
    //         DimensionType::CAFFE_C4 => MNN_Tensor_DimensionType_CAFFE_C4,
    //     };
    //     unsafe {
    //         MNN_Tensor_create(
    //             shape.as_ptr() as *const u8,
    //             halide_type,
    //             std::ptr::null_mut(),
    //             dim_type,
    //         )
    //     }
    // }

    // tensor里添加数据
    pub fn set_data(&self, arr: &[f32]) -> anyhow::Result<()> {
        unsafe {
            if let Some(mat) = self.ptr.as_mut() {
                let host_ptr = mat.mBuffer.host as *mut f32;
                std::ptr::copy_nonoverlapping(arr.as_ptr(), host_ptr, arr.len());
                return Ok(());
            }
        }
        anyhow::bail!("set data faild")
    }
    // 获得数据指针
    pub fn get_data(&self) -> anyhow::Result<*mut f32> {
        unsafe {
            if let Some(mat) = self.ptr.as_mut() {
                let host_ptr = mat.mBuffer.host as *mut f32;
                return Ok(host_ptr);
            }
        }
        anyhow::bail!("get data faild")
    }

    // 打印所有数据
    pub fn print(&self) {
        unsafe { MNN_Tensor_print(self.ptr) }
    }

    // 打印形状
    pub fn print_shape(&self) {
        unsafe { MNN_Tensor_printShape(self.ptr) }
    }

    // 复制到外部tensor
}

// 函数内调用会触发segmentation fault，暂时不知道原因
// impl Drop for Tensor {
//     fn drop(&mut self) {
//         unsafe {
//             MNN_Tensor_destroy(self.ptr);
//         }
//     }
// }
