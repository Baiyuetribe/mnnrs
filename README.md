## MNN-RUST(WIP)

mnn 的 rust 绑定

### 完成度

- [✅] 加载模型
- [✅] 参数设置
- [✅] 推理
- [✅] 输入、输出
- [😭] tensor 创建

### 运行 demo

```rust

fn main() {
    let res = mnnrs::version();
    println!("mnnrs version: {}", res);
    let mut net = mnnrs::Net::new();
    net.load_model("xxx.mnn");
    println!("加载模型成功");
    let mut cfg = mnnrs::SessionConfig::default();
    cfg.forward_type = mnnrs::ForwardType::CPU; // CUDA, OpenCL, OpenGL, Vulkan...
    cfg.num_threads = 4;
    cfg.memory = mnnrs::Memory::Low;
    cfg.power = mnnrs::Power::Normal;
    cfg.precision = mnnrs::Precision::High;
    let mnn_cfg = unsafe { cfg.to_mnn_config() };
    let ex = match net.create_session(&mnn_cfg) {
        Ok(ex) => ex,
        Err(e) => {
            println!("create session failed: {:?}", e);
            return;
        }
    };
    let mut in0 = net.get_input_tensor(&ex.ptr, "input_image"); // 正确
    let mut out0 = net.get_output_tensor(&ex.ptr, "output_image"); // 正确
    let start = std::time::Instant::now();
    net.run_session(&ex.ptr); // 正式推理，正确
    println!(
        "耗时: {:?}",
        std::time::Instant::now().duration_since(start)
    );
    let data: Vec<f32> = vec![0.0; 1 * 255 * 255 * 3]; // 假设有一些数据

    let nhwc = mnnrs::Tensor::create(
        vec![1, 255, 255, 3],
        data.as_ptr() as *mut std::os::raw::c_void,
        mnnrs::DimensionType::TENSORFLOW,
    ); // 这一步异常，卡主不动

    println!("运行正常");
}

```

## 遇到的问题

由于 rust 对 c 绑定友好，c++绑定时遇到如下问题，暂时无解，欢迎 pr

经过`bindgen`后，生成如下 rust 代码

```rust
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
```

当前卡在 MNN_Tensor_create 函数调用后，程序卡主。
