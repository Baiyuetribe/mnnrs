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
    pub memory: Memory,
    pub power: Power,
    pub precision: Precision,
    pub forward_type: ForwardType,
    pub num_threads: i32,
    pub backend_extra: BackendExtra,
}

impl Default for SessionConfig {
    fn default() -> Self {
        SessionConfig {
            memory: Memory::Low,
            power: Power::High,
            precision: Precision::High,
            forward_type: ForwardType::CPU,
            num_threads: 4,
            backend_extra: BackendExtra::Flags(0),
        }
    }
}

impl SessionConfig {
    // 转换为 C++ 需要的 MNN_ScheduleConfig 和 MNN_BackendConfig
    pub fn to_mnn_config(&self) -> MNN_ScheduleConfig {
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
            saveTensors: [0; 3],
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
                inputs: [0; 3],
                outputs: [0; 3],
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
