use ash_destructor::DeviceDestroyable;

#[path = "../../utils/mod.rs"]
mod utils;

use utils::ImplDeviceDestroyable;

#[derive(DeviceDestroyable)]
struct Named {
    a: ImplDeviceDestroyable,
    #[destroy_ignore]
    #[destroy_ignore]
    b: String,
}

#[derive(DeviceDestroyable)]
struct Unnamed(
    ImplDeviceDestroyable,
    #[destroy_ignore]
    #[destroy_ignore]
    String,
);

fn main() {}
