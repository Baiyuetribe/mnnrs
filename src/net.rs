use crate::mnn_bind::*;
use std::ffi::CString;
use std::os::raw::c_int;

pub struct Net {
    ptr: *mut MNN_Interpreter,
}

unsafe impl Send for Net {}
unsafe impl Sync for Net {}

impl Net {
    pub fn new() -> Net {
        Net {
            ptr: std::ptr::null_mut(),
        }
    }
    // 模型加载
    pub fn load_model(&mut self, model_path: &str) -> anyhow::Result<()> {
        let c_str = CString::new(model_path)?;
        unsafe {
            self.ptr = MNN_Interpreter_createFromFile(c_str.as_ptr());
        }
        Ok(())
    }
    // 创建session
    pub fn create_session(&self, session_cfg: &MNN_ScheduleConfig) -> anyhow::Result<Session> {
        let ptr = unsafe { MNN_Interpreter_createSession(self.ptr, session_cfg) };
        Ok(Session { ptr })
    }
    // 获取输入张量
    pub fn get_input_tensor(&self, session: &*mut MNN_Session, name: &str) -> *mut MNN_Tensor {
        let c_str = CString::new(name).unwrap();
        unsafe { MNN_Interpreter_getSessionInput(self.ptr, session.clone(), c_str.as_ptr()) }
    }
    // 获取输出张量
    pub fn get_output_tensor(&self, session: &*mut MNN_Session, name: &str) -> *mut MNN_Tensor {
        let c_str = CString::new(name).unwrap();
        unsafe { MNN_Interpreter_getSessionOutput(self.ptr, session.clone(), c_str.as_ptr()) }
    }

    // 推理
    pub fn run_session(&self, session: &*mut MNN_Session) {
        unsafe {
            MNN_Interpreter_runSession(self.ptr, session.clone());
        }
    }

    pub fn release_session(&self, session: &mut Session) {
        unsafe {
            MNN_Interpreter_releaseSession(self.ptr, session.ptr);
            session.ptr = std::ptr::null_mut(); // 避免重复释放
        }
    }
}

impl Drop for Net {
    fn drop(&mut self) {
        unsafe {
            MNN_Interpreter_destroy(self.ptr);
        }
    }
}

pub struct Session {
    pub ptr: *mut MNN_Session,
}
impl Session {
    // 添加会话相关的方法
}

// impl Drop for Session {
//     fn drop(&mut self) {
//         unsafe {
//             MNN_Interpreter_releaseSession(self.ptr);
//         }
//     }
// }
