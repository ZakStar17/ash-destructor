use ash::vk;
pub use ash_destructor_derive::DeviceDestroyable;

pub trait Destroyable {
    unsafe fn destroy_self(&self);
  }
pub trait DeviceDestroyable {
    unsafe fn destroy_self(&self, device: &ash::Device);
}

impl<T: Destroyable> DeviceDestroyable for T {
    unsafe fn destroy_self(&self, _device: &ash::Device) {
        self.destroy_self();
    }
}
