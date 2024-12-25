use ash_destructor::DeviceDestroyable;

#[path = "../../utils/mod.rs"]
mod utils;

use utils::ImplDeviceDestroyable;

#[derive(DeviceDestroyable)]
struct Named {
    #[destroy_ignore]
    a: ImplDeviceDestroyable,
    #[destroy_ignore_remaining]
    b: String,
    c: u32,
    #[destroy_ignore_remaining]
    d: bool,
}

#[derive(DeviceDestroyable)]
struct Unnamed(
    ImplDeviceDestroyable,
    #[destroy_ignore_remaining]
    #[destroy_ignore_remaining]
    String,
);

fn main() {}
