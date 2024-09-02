## MNN-RUST(WIP)

mnn 的 rust 绑定

### 运行 demo

```rust

fn main() {
    let res = mnnrs::version();
    println!("mnnrs version: {}", res);
    let mut net = mnnrs::Net::new();
    let res = net.load_model("xxx.mnn");
    if res.is_err() {
        println!("load model failed: {:?}", res.err());
        return;
    }
    println!("加载模型成功");
    let cfg = mnnrs::SessionConfig::default();
    let mnn_cfg = unsafe { cfg.to_mnn_config() };
    let ex = net.create_session(&mnn_cfg); // 目前这一步卡住
    if ex.is_err() {
        println!("create session failed: {:?}", ex.err());
        return;
    }
    println!("运行正常");
}

```

## 遇到的问题

由于 rust 对 c 绑定友好，c++绑定时遇到如下问题，暂时无解，欢迎 pr

```c++
struct ScheduleConfig {
    std::vector<std::string> saveTensors;
    MNNForwardType type = MNN_FORWARD_CPU;
    union {
        int numThread = 4;
        int mode;
    };

    struct Path {
        std::vector<std::string> inputs;
        std::vector<std::string> outputs;

        enum Mode {
            Op = 0,
            Tensor = 1
        };
        Mode mode = Op;
    };
    Path path;
    MNNForwardType backupType = MNN_FORWARD_CPU;
    BackendConfig* backendConfig = nullptr;
};
```

上述代码，经过`bindgen`后，生成如下 rust 代码

```rust
pub struct MNN_ScheduleConfig {
    pub saveTensors: std_vector,
    pub type_: MNNForwardType,
    pub __bindgen_anon_1: MNN_ScheduleConfig__bindgen_ty_1,
    pub path: MNN_ScheduleConfig_Path,
    pub backupType: MNNForwardType,
    pub backendConfig: *mut MNN_BackendConfig,
}
```

此时，原本 c++中的字符列表等，就不太好处理了【个人萌新，欢迎大佬指正】
