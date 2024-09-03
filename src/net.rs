use crate::mnn_bind::*;
use std::ffi::CString;
use std::os::raw::c_int;

use crate::{mat::Tensor, session::Session};
pub struct Net {
    pub ptr: *mut MNN_Interpreter,
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
    pub fn load_model(&mut self, path: &str) -> anyhow::Result<()> {
        let c_str = {
            #[cfg(target_os = "windows")]
            {
                let (gbk_bytes, _, _) = encoding_rs::GB18030.encode(path);
                CString::new(gbk_bytes)?
            }
            #[cfg(not(target_os = "windows"))]
            {
                CString::new(path)?
            }
        };
        unsafe {
            self.ptr = MNN_Interpreter_createFromFile(c_str.as_ptr());
        }
        Ok(())
    }
    // 创建session
    pub fn create_session(&self, session_cfg: &MNN_ScheduleConfig) -> Session {
        let ptr = unsafe { MNN_Interpreter_createSession(self.ptr, session_cfg) };
        Session { net: self, ptr }
    }
    // 获取输入张量
    pub fn get_input_tensor(&self, session: &Session, name: &str) -> anyhow::Result<Tensor> {
        let c_str = CString::new(name)?;
        let ptr = unsafe { MNN_Interpreter_getSessionInput(self.ptr, session.ptr, c_str.as_ptr()) };
        Ok(Tensor { ptr })
    }
    // 获取输出张量
    pub fn get_output_tensor(&self, session: &Session, name: &str) -> anyhow::Result<Tensor> {
        let c_str = CString::new(name)?;
        let ptr =
            unsafe { MNN_Interpreter_getSessionOutput(self.ptr, session.ptr, c_str.as_ptr()) };
        Ok(Tensor { ptr })
    }

    // 推理
    pub fn run_session(&self, session: &Session) {
        unsafe {
            MNN_Interpreter_runSession(self.ptr, session.ptr);
        }
    }
    // ignore: 手动释放？
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
