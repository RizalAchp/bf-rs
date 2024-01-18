#[allow(unused)]

pub struct Jit {}



#[cfg_attr(all(target_arch = "x86_64", target_os = "linux"), path = "linux.rs")]
#[cfg_attr(all(target_arch = "x86_64", target_os = "windows"), path = "windows.rs")]
mod platform;
