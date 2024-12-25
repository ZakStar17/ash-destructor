use ash_destructor::DeviceDestroyable;

#[path = "../../utils/mod.rs"]
mod utils;

use utils::ImplDeviceDestroyable;

#[derive(DeviceDestroyable)]
struct Named {
    a: ImplDeviceDestroyable,
    #[destroy_ignore]
    #[destroy_ignore_remaining]
    b: String,
    c: u32,
    #[destroy_ignore]
    d: bool,
}

#[derive(DeviceDestroyable)]
struct Unnamed(#[destroy_ignore_remaining] ImplDeviceDestroyable, #[destroy_ignore] String);

fn main() {}
