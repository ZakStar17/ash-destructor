use ash_destructor::DeviceDestroyable;

struct ImplDeviceDestroyable {}

impl DeviceDestroyable for ImplDeviceDestroyable {
    unsafe fn destroy_self(&self, _device: &ash::Device) {}
}

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
