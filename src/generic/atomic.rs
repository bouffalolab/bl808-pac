use portable_atomic::Ordering;
pub trait AtomicOperations {
    unsafe fn atomic_or(ptr: *mut Self, val: Self);
    unsafe fn atomic_and(ptr: *mut Self, val: Self);
    unsafe fn atomic_xor(ptr: *mut Self, val: Self);
}
macro_rules! impl_atomics {
    ( $ U : ty , $ Atomic : ty ) => {
        impl AtomicOperations for $U {
            unsafe fn atomic_or(ptr: *mut Self, val: Self) {
                (*(ptr as *const $Atomic)).or(val, Ordering::SeqCst);
            }
            unsafe fn atomic_and(ptr: *mut Self, val: Self) {
                (*(ptr as *const $Atomic)).and(val, Ordering::SeqCst);
            }
            unsafe fn atomic_xor(ptr: *mut Self, val: Self) {
                (*(ptr as *const $Atomic)).xor(val, Ordering::SeqCst);
            }
        }
    };
}
impl_atomics!(u8, portable_atomic::AtomicU8);
impl_atomics!(u16, portable_atomic::AtomicU16);
#[cfg(not(target_pointer_width = "16"))]
impl_atomics!(u32, portable_atomic::AtomicU32);
#[cfg(any(target_pointer_width = "64", target_has_atomic = "64"))]
impl_atomics!(u64, portable_atomic::AtomicU64);
