use std::cell::Cell;

use ash_destructor::DeviceDestroyable;

struct ImplDeviceDestroyable {
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
}

impl DeviceDestroyable for ImplDeviceDestroyable {
    unsafe fn destroy_self(&self, _device: &ash::Device) {
        if self.destroyed.get() {
            panic!("A double destruction has occurred");
        }
        self.destroyed.set(true);
    }
}

#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
