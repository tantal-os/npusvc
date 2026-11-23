use npusvc::{Device, Options, Runtime};

#[test]
fn accepts_only_supported_devices() {
    for (name, device) in [
        ("cpu", Device::Cpu),
        ("gpu", Device::Gpu),
        ("npu", Device::Npu),
    ] {
        assert_eq!(name.parse(), Ok(device));
    }
    assert!("tpu".parse::<Device>().is_err());
}

#[test]
fn reports_missing_litert() {
    let result = Runtime::new("/dev/null", "/dev/null", None, Options::default());
    assert!(matches!(result, Err(error) if error.contains("libLiteRt.so")));
}
