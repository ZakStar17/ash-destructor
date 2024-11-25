pub use ash_destructor_derive::DeviceDestroyable;

pub trait DeviceDestroyable {
    unsafe fn destroy_self(&self, device: &ash::Device);
}

pub trait Destroyable {
    unsafe fn destroy_self(&self);
}

// I tried to impl generic implementation for Destroyable traits based on priority, like what
// happens in this rfc https://rust-lang.github.io/rfcs/1210-impl-specialization.html
//
// For example, DeviceDestroyable can be implemented for objects that implement DoubleEndedIterator as well
// Deref, however the implementation becomes ambiguous for types that implement both, like Box
// 
// I went to impl exclusive traits (https://geo-ant.github.io/blog/2021/mutually-exclusive-traits-rust/)
// which wasn't completely a waste of time but it still missed the point that there is no way to 
// specify which exclusive trait a type implements if the type already has both traits (in contrary
// to a self-made object that we can specify implements one or another) 
//
// There probably is a way to do this with some unstable feature, like for example the 
// impl-specialization mentioned above, but not in a way that seemed sound, as far as I have found.

// default iter DeviceDestroyable implementation trait
pub trait DeviceDestroyableIter<T>
where
    T: DeviceDestroyable,
    for<'a> &'a Self: IntoIterator<Item = &'a T>,
    for<'a> <&'a Self as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    unsafe fn iterate_and_destroy(&self, device: &ash::Device) {
        for val in self.into_iter().rev() {
            val.destroy_self(device);
        }
    }
}

// default deref DeviceDestroyable implementation trait
pub trait DeviceDestroyableDeref
where
    Self: std::ops::Deref<Target = Self::DestroyTarget>,
{
    type DestroyTarget: DeviceDestroyable;

    unsafe fn deref_and_destroy(&self, device: &ash::Device) {
        self.deref().destroy_self(device);
    }
}

impl<T: DeviceDestroyable> DeviceDestroyableIter<T> for Vec<T> {}

impl<T: DeviceDestroyable> DeviceDestroyableDeref for Box<T> {
    type DestroyTarget = T;
}

// error
// impl<T: DeviceDestroyable> DeviceDestroyableIter<T> for Box<T> {}

pub trait DeviceDestroyableGenericImpl {
    type Type: DeviceDestroyableGenericImplType;

    unsafe fn generic_destroy_self_impl(&self, device: &ash::Device);
}

pub trait DeviceDestroyableGenericImplType {}

pub struct DeviceDestroyableGenericIterType;
pub struct DeviceDestroyableGenericDerefType;

impl DeviceDestroyableGenericImplType for DeviceDestroyableGenericIterType {}
impl DeviceDestroyableGenericImplType for DeviceDestroyableGenericDerefType {}

impl<T, I: DeviceDestroyableIter<T>> DeviceDestroyableGenericImpl for I
where
    T: DeviceDestroyable,
    for<'a> &'a I: IntoIterator<Item = &'a T>,
    for<'a> <&'a I as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    type Type = DeviceDestroyableGenericIterType;

    unsafe fn generic_destroy_self_impl(&self, device: &ash::Device) {
        self.iterate_and_destroy(device);
    }
}

impl<T, D> DeviceDestroyableGenericImpl for D
where 
    T: DeviceDestroyable,
    D: DeviceDestroyableDeref<DestroyTarget = T>
{
    type Type = DeviceDestroyableGenericDerefType;

    unsafe fn generic_destroy_self_impl(&self, device: &ash::Device) {
        self.deref_and_destroy(device);
    }
}

impl<T: DeviceDestroyableGenericImpl<Type = DeviceDestroyableGenericDerefType>> DeviceDestroyable for T {
    unsafe fn destroy_self(&self, device: &ash::Device) {
        self.generic_destroy_self_impl(device);
    }
}

// impl<T: Destroyable> DeviceDestroyable for T {
//     unsafe fn destroy_self(&self, _device: &ash::Device) {
//         Destroyable::destroy_self(self);
//     }
// }

// impl<T: DeviceDestroyable> DeviceDestroyable for &T {
//     unsafe fn destroy_self(&self, device: &ash::Device) {
//        (*self).destroy_self(device);
//     }
// }

// impl<T: DeviceDestroyable> DeviceDestroyable for dyn AsRef<T> {
//     unsafe fn destroy_self(&self, device: &ash::Device) {
//         self.as_ref().destroy_self(device);
//     }
// }

// todo: make a destroyable trait for iterables that is separated
// https://geo-ant.github.io/blog/2021/mutually-exclusive-traits-rust/

// impl<T: DeviceDestroyable, D: std::ops::Deref<Target = T>> DeviceDestroyable for D {
//     unsafe fn destroy_self(&self, device: &ash::Device) {
//         self.deref().destroy_self(device);
//     }
// }

// impl<T: DeviceDestroyable> DeviceDestroyable for &[T] {
//     unsafe fn destroy_self(&self, device: &ash::Device) {
//         for item in self.iter().rev() {
//             item.destroy_self(device);
//         }
//     }
// }

// I really had to dig deep to make this work https://github.com/rust-lang/rust/issues/56556
// implement DeviceDestroyable for all collections that can be iterated backwards
// without consuming the collection
// impl<T: DeviceDestroyable, I> DeviceDestroyable for I
// where
//     for<'a> &'a I: IntoIterator<Item = &'a T>,
//     for<'a> <&'a I as IntoIterator>::IntoIter: DoubleEndedIterator,
// {
//     unsafe fn destroy_self(&self, device: &ash::Device) {
//         for val in self.into_iter().rev() {
//             val.destroy_self(device);
//         }
//     }
// }

// impl <T: DeviceDestroyable> DeviceDestroyable for Box<T> {
//     unsafe fn destroy_self(&self, device: &ash::Device) {
//         self.as_ref().destroy_self(device);
//     }
// }

// impl<T: DeviceDestroyable> DeviceDestroyable for Option<T> {
//     unsafe fn destroy_self(&self, device: &ash::Device) {
//         if let Some(val) = self {
//             val.destroy_self(device);
//         }
//     }
// }

// impl<T: DeviceDestroyable> DeviceDestroyable for Vec<T> {
//     unsafe fn destroy_self(&self, device: &ash::Device) {
//         self.as_slice().destroy_self(device);
//     }
// }

// impl<T: DeviceDestroyable> DeviceDestroyable for std::cell::LazyCell<T> {
//     unsafe fn destroy_self(&self, device: &ash::Device) {
//         std::cell::LazyCell::force(self).destroy_self(device);
//     }
// }

// impl<T: DeviceDestroyable> DeviceDestroyable for std::cell::OnceCell<T> {
//     unsafe fn destroy_self(&self, device: &ash::Device) {
//         self.get().destroy_self(device);
//     }
// }
