use ash_destructor::DeviceDestroyable;

#[path = "../../utils/mod.rs"]
mod utils;

use utils::ImplDeviceDestroyable;

#[derive(DeviceDestroyable)]
struct Named<'a, 'b, T: DeviceDestroyable, const N: usize> {
    pub a: &'a T,
    pub b: [[Box<ImplDeviceDestroyable>; 2]; 3],
    pub c: Option<&'a &'b ImplDeviceDestroyable>,
    pub d: [T; N],
}

#[derive(DeviceDestroyable)]
struct Unnamed<'a, T: DeviceDestroyable>(
    T,
    Box<[ImplDeviceDestroyable]>,
    &'a ImplDeviceDestroyable,
);

fn main() {}
