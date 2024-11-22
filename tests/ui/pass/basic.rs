use ash_destructor::DeviceDestroyable;

#[path = "../../utils/mod.rs"]
mod utils;

use utils::ImplDeviceDestroyable;

#[derive(Default, DeviceDestroyable)]
struct Named {
    pub a: ImplDeviceDestroyable,
    pub b: ImplDeviceDestroyable,
}

#[derive(Default, DeviceDestroyable)]
struct Unnamed(ImplDeviceDestroyable, ImplDeviceDestroyable);

#[derive(DeviceDestroyable)]
struct Empty;

fn main() {
    let device = utils::create_dummy_device();

    let named = Named::default();
    let unnamed = Unnamed::default();
    let empty = Empty {};
    unsafe {
        named.destroy_self(&device);
        unnamed.destroy_self(&device);
        empty.destroy_self(&device);
    }

    named.a.assert_destroyed();
    named.b.assert_destroyed();
    unnamed.0.assert_destroyed();
    unnamed.1.assert_destroyed();
}
