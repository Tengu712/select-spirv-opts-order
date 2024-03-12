#[link(name = "vulkan-wrapper", kind = "static")]
extern "C" {
    fn ff_delete_vulkan_app(_: *mut std::ffi::c_void);
    fn ff_create_vulkan_app() -> *mut std::ffi::c_void;
    fn ff_render(_: *mut std::ffi::c_void, _: *mut u64) -> std::ffi::c_int;
}

pub fn measure() -> Result<u64, String> {
    // create a vulkan app
    let vapp = unsafe { ff_create_vulkan_app() };
    if vapp.is_null() {
        return Err(format!("[ warning ] failed to create a vulkan app."));
    }

    // increase the rendering frequency to 20 times
    // sum up the measurement values for the latter 10 iterations
    let mut total = 0;
    for i in 0..20 {
        let mut time = 0;
        if unsafe { ff_render(vapp, &mut time) } == 0 {
            return Err(format!("[ warning ] failed to render."));
        }
        if i >= 10 {
            total += time;
        }
    }

    // delete the vulkan app
    unsafe { ff_delete_vulkan_app(vapp) };

    // finish
    Ok((total as f64 / 10.0) as u64)
}

pub fn eval(flags: &Vec<String>) -> Result<u64, String> {
    // run spirv-opt with flags
    let output = std::process::Command::new("spirv-opt")
        .args(flags)
        .args(["shader.org.frag.spv", "-o", "shader.frag.spv"])
        .stdout(std::process::Stdio::inherit())
        .status();
    match output {
        Ok(n) if !n.success() => {
            return Err(format!(
                "[ warning ] failed to create an optimized shader: {}",
                n.code().unwrap()
            ));
        }
        Err(e) => {
            return Err(format!(
                "[ warning ] failed to run the 'spirv-opt' command: {}",
                e.to_string()
            ));
        }
        _ => (),
    }

    measure()
}
