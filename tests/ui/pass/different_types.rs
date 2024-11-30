use ash_destructor::DeviceDestroyable;

#[path = "../../utils/mod.rs"]
mod utils;

use utils::ImplDeviceDestroyable;

#[derive(DeviceDestroyable)]
struct Named<'a, 'b, T: DeviceDestroyable, const N: usize> {
    pub a: &'a T,
    pub b: [[Box<[ImplDeviceDestroyable; 5]>; N]; 3],
    pub c: Option<&'a &'b ImplDeviceDestroyable>,
    pub d: Option<[T; N]>,
}

#[derive(DeviceDestroyable)]
struct Unnamed<'a, T: DeviceDestroyable>(
    T,
    Box<[ImplDeviceDestroyable]>,
    &'a ImplDeviceDestroyable,
);

fn main() {}
