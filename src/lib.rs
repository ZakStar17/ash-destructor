pub use ash_destructor_derive::DeviceDestroyable;

pub trait Destroyable {
    unsafe fn destroy_self(&self);
}
pub trait DeviceDestroyable {
    unsafe fn destroy_self(&self, device: &ash::Device);
}

impl<T: DeviceDestroyable + ?Sized> DeviceDestroyable for &T {
    unsafe fn destroy_self(&self, device: &ash::Device) {
        (*self).destroy_self(device);
    }
}

impl<T: DeviceDestroyable> DeviceDestroyable for [T] {
    unsafe fn destroy_self(&self, device: &ash::Device) {
        for item in self.iter().rev() {
            item.destroy_self(device);
        }
    }
}

impl<T: DeviceDestroyable, const S: usize> DeviceDestroyable for [T; S] {
    unsafe fn destroy_self(&self, device: &ash::Device) {
        for item in self.iter().rev() {
            item.destroy_self(device);
        }
    }
}

impl<T: DeviceDestroyable> DeviceDestroyable for Vec<T> {
    unsafe fn destroy_self(&self, device: &ash::Device) {
        self.as_slice().destroy_self(device);
    }
}

impl<T: DeviceDestroyable + ?Sized> DeviceDestroyable for Box<T> {
    unsafe fn destroy_self(&self, device: &ash::Device) {
        self.as_ref().destroy_self(device);
    }
}

impl<T: DeviceDestroyable> DeviceDestroyable for Option<T> {
    unsafe fn destroy_self(&self, device: &ash::Device) {
        if let Some(val) = self {
            val.destroy_self(device);
        }
    }
}

impl<T: DeviceDestroyable> DeviceDestroyable for std::cell::LazyCell<T> {
    unsafe fn destroy_self(&self, device: &ash::Device) {
        std::cell::LazyCell::force(self).destroy_self(device);
    }
}

impl<T: DeviceDestroyable> DeviceDestroyable for std::cell::OnceCell<T> {
    unsafe fn destroy_self(&self, device: &ash::Device) {
        self.get().destroy_self(device);
    }
}
