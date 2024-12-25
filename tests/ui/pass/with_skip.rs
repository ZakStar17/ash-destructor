use ash_destructor::DeviceDestroyable;

#[path = "../../utils/mod.rs"]
mod utils;

use utils::ImplDeviceDestroyable;

#[derive(Default, DeviceDestroyable)]
struct Named {
    #[destroy_ignore]
    pub a: bool,
    pub b: ImplDeviceDestroyable,
    #[destroy_ignore_remaining]
    pub c: ImplDeviceDestroyable,
    pub d: usize,
}

#[derive(Default, DeviceDestroyable)]
struct Unnamed(
    ImplDeviceDestroyable,
    #[destroy_ignore] ImplDeviceDestroyable,
    #[destroy_ignore_remaining] usize,
);

fn main() {
    let device = utils::create_dummy_device();

    let named = Named::default();
    let unnamed = Unnamed::default();
    unsafe {
        named.destroy_self(&device);
        unnamed.destroy_self(&device);
    }

    named.b.assert_destroyed();
    named.c.assert_not_destroyed();
    unnamed.0.assert_destroyed();
    unnamed.1.assert_not_destroyed();
}
