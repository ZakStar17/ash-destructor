use ash_destructor::DeviceDestroyable;

#[path = "../../utils/mod.rs"]
mod utils;

use utils::ImplDeviceDestroyable;

#[derive(DeviceDestroyable)]
struct Named {
    a: ImplDeviceDestroyable,
    #[skip]
    #[skip]
    b: String,
}

#[derive(DeviceDestroyable)]
struct Unnamed(
    ImplDeviceDestroyable,
    #[skip]
    #[skip]
    String,
);

fn main() {}
