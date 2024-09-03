## MNN-RUST

mnn 的 rust 绑定，成功实现 rust 调用。

### 已完成

- [✅] 加载模型
- [✅] 参数设置
- [✅] 推理
- [✅] 输入、输出
- [✅] 预处理及后处理

### 运行 demo

```rust
fn main() -> anyhow::Result<()> {
    let res = mnnrs::version();
    println!("mnnrs version: {}", res);
    let mut net = mnnrs::Net::new();
    net.load_model("xxx.mnn")?; // 加载模型
    let mut opt = mnnrs::SessionConfig::default();
    opt.forward_type = mnnrs::ForwardType::CPU; // CUDA, OpenCL, OpenGL, Vulkan...
    opt.num_threads = 4;
    opt.memory = mnnrs::Memory::Low;
    opt.power = mnnrs::Power::High;
    opt.precision = mnnrs::Precision::High;
    let mnn_cfg = opt.to_mnn_config();
    let ex = net.create_session(&mnn_cfg); // 创建session
                                           // 输入和输出创建
    let mut in0 = net.get_input_tensor(&ex, "image")?; // 输入
    let mut in1 = net.get_input_tensor(&ex, "mask")?; // 输入
    let mut out0 = net.get_output_tensor(&ex, "output")?; // 输出

    // 预处理
    let data = [1.460; 1 * 512 * 512 * 1]; // 假设有一些数据
    in1.set_data(&data);

    // 推理
    net.run_session(&ex); // 正式推理

    // 后处理
    let ptr_f32 = out0.get_data()?;
    let res = unsafe { std::slice::from_raw_parts(ptr_f32, data.len()) };
    println!("{:?}", res);

    println!("推理结束");
    Ok(())
}

```

### 使用方法

```bash
cargo add mnnrs
# 然后设置环境变量
export MNN_INCLUDE_DIR=/path/to/mnn/include
```

用法和我写的 ncnnrs 一样，风格也一样。MNN 的好处是几乎支持 100%的 onnx 模型转换，不像 ncnn 那样几乎 9 成新模型卡在无法转换上。

### 特性

- 使用 rust 开发 MNN
- 分离静态库，可满足跨端编译要求

### ref

- https://github.com/alibaba/MNN
- https://github.com/baiyuetribe/ncnnrs
