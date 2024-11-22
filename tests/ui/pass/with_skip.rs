use ash_destructor::DeviceDestroyable;

#[path = "../../utils/mod.rs"]
mod utils;

use utils::ImplDeviceDestroyable;

#[derive(Default, DeviceDestroyable)]
struct Named {
    #[skip]
    pub a: bool,
    pub b: ImplDeviceDestroyable,
    #[skip_remaining]
    pub c: ImplDeviceDestroyable,
    pub d: usize,
}

#[derive(Default, DeviceDestroyable)]
struct Unnamed(
    ImplDeviceDestroyable,
    #[skip] ImplDeviceDestroyable,
    #[skip_remaining] usize,
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
