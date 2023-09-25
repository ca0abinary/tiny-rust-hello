#[cfg_attr(
    all(target_arch = "x86_64", target_os = "linux"),
    path = "linux_x86_64.rs"
)]
#[cfg_attr(
    all(target_arch = "aarch64", target_os = "linux"),
    path = "linux_aarch64.rs"
)]
mod minilib;
pub use minilib::*;

mod noarch;
pub use noarch::*;
