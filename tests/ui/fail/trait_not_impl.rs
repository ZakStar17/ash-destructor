use ash_destructor::DeviceDestroyable;

#[path = "../../utils/mod.rs"]
mod utils;

use utils::ImplDeviceDestroyable;

#[derive(DeviceDestroyable)]
struct Named {
    a: ImplDeviceDestroyable,
    // doesn't implement trait
    b: String,
}

#[derive(DeviceDestroyable)]
struct Unnamed(ImplDeviceDestroyable, String);

fn main() {}
