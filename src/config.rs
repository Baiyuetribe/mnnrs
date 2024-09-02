use crate::mnn_bind::{self, *};
use std::os::raw::c_int;

// 定义枚举来表示不同的选项
pub enum Memory {
    Low,
    Normal,
    High,
}

pub enum Power {
    Low,
    Normal,
    High,
}

pub enum Precision {
    Low,
    Normal,
    High,
}

pub enum ForwardType {
    CPU,
    AUTO,
    Metal,
    CUDA,
    OpenCL,
    OpenGL,
    Vulkan,
}
// 新增枚举来表示匿名联合体的两种可能状态
pub enum BackendExtra {
    SharedContext(*mut std::os::raw::c_void),
    Flags(usize),
}
// 创建一个 Rust 友好的配置结构体
pub struct SessionConfig {
    memory: Memory,
    power: Power,
    precision: Precision,
    forward_type: ForwardType,
    num_threads: i32,
    backend_extra: BackendExtra,
}

impl Default for SessionConfig {
    fn default() -> Self {
        SessionConfig {
            memory: Memory::Normal,
            power: Power::Normal,
            precision: Precision::High,
            forward_type: ForwardType::CPU,
            num_threads: 4,
            backend_extra: BackendExtra::Flags(0),
        }
    }
}

use std::marker::PhantomData;

// #[repr(C)]
// pub struct StdVector<T> {
//     __begin_: *mut T,
//     __end_: *mut T,
//     __end_cap_: *mut T,
//     _phantom: PhantomData<T>,
// }

// impl<T> StdVector<T> {
//     pub fn new() -> Self {
//         // 这里我们创建一个空的 StdVector
//         // 注意：这只是一个示例，实际上可能需要通过 FFI 调用 C++ 的 vector 构造函数
//         StdVector {
//             __begin_: std::ptr::null_mut(),
//             __end_: std::ptr::null_mut(),
//             __end_cap_: std::ptr::null_mut(),
//             _phantom: PhantomData,
//         }
//     }

//     // 其他方法...
// }
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_traits {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___compressed_pair {
    pub _address: u8,
}

pub type std_vector___alloc_traits = std_allocator_traits;
pub type std_vector_pointer = std_vector___alloc_traits;

#[derive(Debug)]
struct std_vector {
    pub __begin_: std_vector_pointer,
    pub __end_: std_vector_pointer,
    pub __end_cap_: std___compressed_pair,
}
use std::mem::MaybeUninit;
impl Default for std_vector {
    fn default() -> Self {
        // 创建一个未初始化的 std_vector
        let mut vector: MaybeUninit<std_vector> = MaybeUninit::uninit();

        unsafe {
            // 调用 C++ 的 std::vector 默认构造函数
            extern "C" {
                fn std_vector_default_constructor(vector: *mut std_vector);
            }
            std_vector_default_constructor(vector.as_mut_ptr());

            // 假设构造函数成功，我们现在可以安全地假定 vector 已经被初始化
            vector.assume_init()
        }
    }
}
impl SessionConfig {
    // 转换为 C++ 需要的 MNN_ScheduleConfig 和 MNN_BackendConfig
    pub unsafe fn to_mnn_config(&self) -> MNN_ScheduleConfig {
        let mut backend_config = MNN_BackendConfig {
            memory: match self.memory {
                Memory::Low => MNN_BackendConfig_MemoryMode_Memory_Low,
                Memory::Normal => MNN_BackendConfig_MemoryMode_Memory_Normal,
                Memory::High => MNN_BackendConfig_MemoryMode_Memory_High,
            },
            power: match self.power {
                Power::Low => MNN_BackendConfig_PowerMode_Power_Low,
                Power::Normal => MNN_BackendConfig_PowerMode_Power_Normal,
                Power::High => MNN_BackendConfig_PowerMode_Power_High,
            },
            precision: match self.precision {
                Precision::Low => MNN_BackendConfig_PrecisionMode_Precision_Low,
                Precision::Normal => MNN_BackendConfig_PrecisionMode_Precision_Normal,
                Precision::High => MNN_BackendConfig_PrecisionMode_Precision_High,
            },
            __bindgen_anon_1: match self.backend_extra {
                BackendExtra::SharedContext(ctx) => {
                    MNN_BackendConfig__bindgen_ty_1 { sharedContext: ctx }
                }
                BackendExtra::Flags(flags) => MNN_BackendConfig__bindgen_ty_1 {
                    sharedContext: flags as *mut std::os::raw::c_void,
                },
            },
        };

        let schedule_config = MNN_ScheduleConfig {
            saveTensors: mnn_bind::std_vector {
                __begin_: mnn_bind::std_allocator_traits { _address: 0 },
                __end_: mnn_bind::std_allocator_traits { _address: 0 },
                __end_cap_: mnn_bind::std___compressed_pair { _address: 0 },
            },
            type_: match self.forward_type {
                ForwardType::CPU => MNNForwardType_MNN_FORWARD_CPU,
                ForwardType::AUTO => MNNForwardType_MNN_FORWARD_AUTO,
                ForwardType::Metal => MNNForwardType_MNN_FORWARD_METAL,
                ForwardType::CUDA => MNNForwardType_MNN_FORWARD_CUDA,
                ForwardType::OpenCL => MNNForwardType_MNN_FORWARD_OPENCL,
                ForwardType::OpenGL => MNNForwardType_MNN_FORWARD_OPENGL,
                ForwardType::Vulkan => MNNForwardType_MNN_FORWARD_VULKAN,
            },
            __bindgen_anon_1: MNN_ScheduleConfig__bindgen_ty_1 {
                numThread: self.num_threads,
            },
            path: MNN_ScheduleConfig_Path {
                inputs: mnn_bind::std_vector {
                    __begin_: mnn_bind::std_allocator_traits { _address: 0 },
                    __end_: mnn_bind::std_allocator_traits { _address: 0 },
                    __end_cap_: mnn_bind::std___compressed_pair { _address: 0 },
                },
                outputs: mnn_bind::std_vector {
                    __begin_: mnn_bind::std_allocator_traits { _address: 0 },
                    __end_: mnn_bind::std_allocator_traits { _address: 0 },
                    __end_cap_: mnn_bind::std___compressed_pair { _address: 0 },
                },
                mode: MNN_ScheduleConfig_Path_Mode_Op, // 默认
            },
            backupType: match self.forward_type {
                ForwardType::CPU => MNNForwardType_MNN_FORWARD_CPU,
                ForwardType::AUTO => MNNForwardType_MNN_FORWARD_AUTO,
                ForwardType::Metal => MNNForwardType_MNN_FORWARD_METAL,
                ForwardType::CUDA => MNNForwardType_MNN_FORWARD_CUDA,
                ForwardType::OpenCL => MNNForwardType_MNN_FORWARD_OPENCL,
                ForwardType::OpenGL => MNNForwardType_MNN_FORWARD_OPENGL,
                ForwardType::Vulkan => MNNForwardType_MNN_FORWARD_VULKAN,
            }, // 默认
            backendConfig: &mut backend_config as *mut MNN_BackendConfig,
        };

        schedule_config
    }
}
