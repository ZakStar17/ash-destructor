use std::cell::Cell;

use ash_destructor::DeviceDestroyable;

pub struct ImplDeviceDestroyable {
    destroyed: Cell<bool>,
}

impl ImplDeviceDestroyable {
    pub fn new() -> Self {
        Self {
            destroyed: Cell::new(false),
        }
    }

    pub fn assert_destroyed(&self) {
        assert!(self.destroyed.get());
    }

    pub fn assert_not_destroyed(&self) {
        assert!(!self.destroyed.get());
    }
}

impl std::default::Default for ImplDeviceDestroyable {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceDestroyable for ImplDeviceDestroyable {
    unsafe fn destroy_self(&self, _device: &ash::Device) {
        if self.destroyed.get() {
            panic!("A double destruction has occurred");
        }
        self.destroyed.set(true);
    }
}
