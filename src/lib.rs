// 包含生成的绑定
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod mnn_bind {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::ffi::CStr;
mod config;
mod mat;
mod net;
mod session; // 引入
pub use config::*;
pub use mat::*;
pub use net::*;
pub use session::*; // 对外开发 // 对外开发

pub fn version() -> &'static str {
    let c_buf = unsafe { mnn_bind::MNN_getVersion() };
    if c_buf.is_null() {
        return "unknown";
    }
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap_or("unknown");
    str_slice
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v = version();
        println!("MNN version: {}", v);
        assert!(!v.is_empty());
    }
}
