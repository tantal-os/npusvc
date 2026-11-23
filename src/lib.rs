use std::ops::Deref;
use std::str::FromStr;

use npusvc_sdk::lrt::{CPU, Compiled, GPU, Lrt, NPU};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Device {
    Cpu,
    Gpu,
    Npu,
}

impl FromStr for Device {
    type Err = String;

    fn from_str(device: &str) -> Result<Self, Self::Err> {
        match device {
            "cpu" => Ok(Self::Cpu),
            "gpu" => Ok(Self::Gpu),
            "npu" => Ok(Self::Npu),
            _ => Err(format!(
                "unknown device {device:?}; expected cpu, gpu, or npu"
            )),
        }
    }
}

#[derive(Default)]
pub struct Options {
    pub google_tensor: Option<String>,
    pub cpu: Option<String>,
}

pub struct Runtime(Lrt);

impl Runtime {
    pub fn new(
        libraries: &str,
        plugins: &str,
        cache: Option<&str>,
        options: Options,
    ) -> Result<Self, String> {
        let mut runtime = Lrt::new(libraries, plugins, cache)?;
        runtime.google_tensor = options.google_tensor;
        runtime.cpu = options.cpu;
        Ok(Self(runtime))
    }

    pub fn compile(&self, file: &str, device: Device) -> Result<Model, String> {
        let accelerator = match device {
            Device::Cpu => CPU,
            Device::Gpu => GPU,
            Device::Npu => NPU,
        };
        self.0.compile(file, accelerator).map(Model)
    }
}

pub struct Model(Compiled);

impl Model {
    pub fn run(&mut self) -> Result<(), String> {
        self.0.run()
    }
}

impl Deref for Model {
    type Target = Compiled;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
