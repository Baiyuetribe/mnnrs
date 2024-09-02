use crate::mnn_bind::*;

pub struct Ex {
    ptr: *mut MNN_Session,
}
impl Ex {
    // 添加会话相关的方法
}

impl Drop for Ex {
    fn drop(&mut self) {
        unsafe {
            MNN_Interpreter_releaseSession(self.ptr);
        }
    }
}
