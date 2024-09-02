use bindings::MNN;

pub struct MNN_Tensor {
    session: *mut MNN::Session,
}
impl MNN_Tensor {
    // 添加会话相关的方法
}

impl Drop for MNN_Tensor {
    fn drop(&mut self) {
        unsafe {
            // 注意：这里假设有一个销毁会话的方法，实际方法可能不同
            MNN::Interpreter_releaseSession(self.session);
        }
    }
}
