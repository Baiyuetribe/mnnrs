use crate::mnn_bind::*;
use std::ffi::CString;
use std::os::raw::c_int;

pub struct Session<'a> {
    pub net: &'a crate::Net,
    pub ptr: *mut MNN_Session,
}
impl<'a> Session<'a> {
    // 添加会话相关的方法
}

impl<'a> Drop for Session<'a> {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                MNN_Interpreter_releaseSession(self.net.ptr, self.ptr);
            }
        }
    }
}
