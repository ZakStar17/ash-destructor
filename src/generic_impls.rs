use crate::{Alloc, DeviceDestroyable, SelfDestroyable};

impl<T: DeviceDestroyable + ?Sized> DeviceDestroyable for &T {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        DeviceDestroyable::destroy_self_alloc(*self, device, allocation_callbacks);
    }
}

impl<T: SelfDestroyable + ?Sized> SelfDestroyable for &T {
    unsafe fn destroy_self_alloc(&self, allocation_callbacks: Alloc) {
        SelfDestroyable::destroy_self_alloc(*self, allocation_callbacks);
    }
}

impl<T: DeviceDestroyable> DeviceDestroyable for [T] {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        for item in self.iter().rev() {
            DeviceDestroyable::destroy_self_alloc(item, device, allocation_callbacks);
        }
    }
}

impl<T: SelfDestroyable> SelfDestroyable for [T] {
    unsafe fn destroy_self_alloc(&self, allocation_callbacks: Alloc) {
        for item in self.iter().rev() {
            SelfDestroyable::destroy_self_alloc(item, allocation_callbacks);
        }
    }
}

impl<T: DeviceDestroyable, const S: usize> DeviceDestroyable for [T; S] {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        for item in self.iter().rev() {
            DeviceDestroyable::destroy_self_alloc(item, device, allocation_callbacks);
        }
    }
}

impl<T: SelfDestroyable, const S: usize> SelfDestroyable for [T; S] {
    unsafe fn destroy_self_alloc(&self, allocation_callbacks: Alloc) {
        for item in self.iter().rev() {
            SelfDestroyable::destroy_self_alloc(item, allocation_callbacks);
        }
    }
}

impl<T: DeviceDestroyable> DeviceDestroyable for Vec<T> {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        DeviceDestroyable::destroy_self_alloc(self.as_slice(), device, allocation_callbacks);
    }
}

impl<T: SelfDestroyable> SelfDestroyable for Vec<T> {
    unsafe fn destroy_self_alloc(&self, allocation_callbacks: Alloc) {
        SelfDestroyable::destroy_self_alloc(self.as_slice(), allocation_callbacks);
    }
}

impl<T: DeviceDestroyable + ?Sized> DeviceDestroyable for Box<T> {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        DeviceDestroyable::destroy_self_alloc(self.as_ref(), device, allocation_callbacks);
    }
}

impl<T: SelfDestroyable + ?Sized> SelfDestroyable for Box<T> {
    unsafe fn destroy_self_alloc(&self, allocation_callbacks: Alloc) {
        SelfDestroyable::destroy_self_alloc(self.as_ref(), allocation_callbacks);
    }
}

impl<T: DeviceDestroyable> DeviceDestroyable for Option<T> {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        if let Some(val) = self {
            DeviceDestroyable::destroy_self_alloc(val, device, allocation_callbacks);
        }
    }
}

impl<T: SelfDestroyable> SelfDestroyable for Option<T> {
    unsafe fn destroy_self_alloc(&self, allocation_callbacks: Alloc) {
        if let Some(val) = self {
            SelfDestroyable::destroy_self_alloc(val, allocation_callbacks);
        }
    }
}

impl<T: DeviceDestroyable> DeviceDestroyable for std::cell::LazyCell<T> {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        DeviceDestroyable::destroy_self_alloc(
            std::cell::LazyCell::force(self),
            device,
            allocation_callbacks,
        );
    }
}

impl<T: SelfDestroyable> SelfDestroyable for std::cell::LazyCell<T> {
    unsafe fn destroy_self_alloc(&self, allocation_callbacks: Alloc) {
        SelfDestroyable::destroy_self_alloc(std::cell::LazyCell::force(self), allocation_callbacks);
    }
}

impl<T: DeviceDestroyable> DeviceDestroyable for std::cell::OnceCell<T> {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        DeviceDestroyable::destroy_self_alloc(&self.get(), device, allocation_callbacks);
    }
}

impl<T: SelfDestroyable> SelfDestroyable for std::cell::OnceCell<T> {
    unsafe fn destroy_self_alloc(&self, allocation_callbacks: Alloc) {
        SelfDestroyable::destroy_self_alloc(&self.get(), allocation_callbacks);
    }
}
