use crate::{Alloc, DeviceDestroyable, SelfDestroyable};

impl DeviceDestroyable for ash::Device {
    unsafe fn destroy_self_alloc(&self, _: &ash::Device, allocation_callbacks: Alloc) {
        self.destroy_device(allocation_callbacks);
    }
}

impl SelfDestroyable for ash::Device {
    unsafe fn destroy_self_alloc(&self, allocation_callbacks: Alloc) {
        self.destroy_device(allocation_callbacks);
    }
}

impl DeviceDestroyable for ash::Instance {
    unsafe fn destroy_self_alloc(&self, _: &ash::Device, allocation_callbacks: Alloc) {
        self.destroy_instance(allocation_callbacks);
    }
}

impl SelfDestroyable for ash::Instance {
    unsafe fn destroy_self_alloc(&self, allocation_callbacks: Alloc) {
        self.destroy_instance(allocation_callbacks);
    }
}
