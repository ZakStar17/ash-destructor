mod device_impls;
mod generic_impls;
mod self_impls;

use ash::vk;
pub use ash_destructor_derive::DeviceDestroyable;

type Alloc<'a> = Option<&'a vk::AllocationCallbacks<'a>>;

// can destroy itself using a device
pub trait DeviceDestroyable {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Option<&vk::AllocationCallbacks>);

    unsafe fn destroy_self(&self, device: &ash::Device) {
        DeviceDestroyable::destroy_self_alloc(self, device, None);
    }
}

// can destroy itself without the need of a device
pub trait SelfDestroyable: DeviceDestroyable {
    unsafe fn destroy_self_alloc(&self, allocation_callbacks: Option<&vk::AllocationCallbacks>);

    unsafe fn destroy_self(&self) {
        SelfDestroyable::destroy_self_alloc(self, None);
    }
}
