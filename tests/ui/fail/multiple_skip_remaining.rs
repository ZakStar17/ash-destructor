use ash_destructor::DeviceDestroyable;

#[path = "../../utils/mod.rs"]
mod utils;

use utils::ImplDeviceDestroyable;

#[derive(DeviceDestroyable)]
struct Named {
    #[skip]
    a: ImplDeviceDestroyable,
    #[skip_remaining]
    b: String,
    c: u32,
    #[skip_remaining]
    d: bool,
}

#[derive(DeviceDestroyable)]
struct Unnamed(
    ImplDeviceDestroyable,
    #[skip_remaining]
    #[skip_remaining]
    String,
);

fn main() {}
