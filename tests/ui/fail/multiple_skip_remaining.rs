use ash_destructor::DeviceDestroyable;

struct ImplDeviceDestroyable {}

impl DeviceDestroyable for ImplDeviceDestroyable {
    unsafe fn destroy_self(&self, _device: &ash::Device) {}
}

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
