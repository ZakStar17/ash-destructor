use ash_destructor::DeviceDestroyable;

#[path = "../../utils/mod.rs"]
mod utils;

use utils::ImplDeviceDestroyable;

#[derive(DeviceDestroyable)]
struct Named {
    a: ImplDeviceDestroyable,
    #[skip]
    #[skip_remaining]
    b: String,
    c: u32,
    #[skip]
    d: bool,
}

#[derive(DeviceDestroyable)]
struct Unnamed(#[skip_remaining] ImplDeviceDestroyable, #[skip] String);

fn main() {}
