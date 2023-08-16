use std::{
    alloc::Layout,
    fmt::Debug,
    future::Future,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    slice,
};

use crate::{buffer_sel::AlignedBuffer, coerce::Coerce};

pub struct SpecifiedBox<B, T>
where
    B: AlignedBuffer,
    T: ?Sized,
{
    buffer: B,
    destructor: unsafe fn(*mut ()),
    as_ref: unsafe fn(*const ()) -> *const T,
    as_mut: unsafe fn(*mut ()) -> *mut T,
}

impl<B, T> SpecifiedBox<B, T>
where
    B: AlignedBuffer,
    T: ?Sized,
{
    pub fn new<U>(value: U) -> Self
    where
        U: Coerce<T>,
    {
        let layout = Layout::new::<U>();
        assert_eq!(layout, B::LAYOUT);

        let value = ManuallyDrop::new(value);
        let byte_ptr = &*value as *const U as *const MaybeUninit<u8>;
        let mut buffer = B::default();

        unsafe {
            let bytes = slice::from_raw_parts(byte_ptr, layout.size());
            buffer.get_buffer_mut().copy_from_slice(bytes);
            Self {
                buffer,
                destructor: |buffer: *mut ()| {
                    ManuallyDrop::drop(&mut *(buffer as *mut ManuallyDrop<U>))
                },
                as_ref: |ptr| (&*(ptr as *const U)).coerce(),
                as_mut: |ptr| (&mut *(ptr as *mut U)).coerce_mut(),
            }
        }
    }
}

impl<B, T> Deref for SpecifiedBox<B, T>
where
    B: AlignedBuffer,
    T: ?Sized,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.as_ref)(self.buffer.get_buffer() as *const [MaybeUninit<u8>] as _) }
    }
}

impl<B, T> DerefMut for SpecifiedBox<B, T>
where
    B: AlignedBuffer,
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.as_mut)(self.buffer.get_buffer_mut() as *mut [MaybeUninit<u8>] as _) }
    }
}

impl<B, T> Drop for SpecifiedBox<B, T>
where
    B: AlignedBuffer,
    T: ?Sized,
{
    fn drop(&mut self) {
        unsafe {
            (self.destructor)(self.buffer.get_buffer_mut() as *mut [MaybeUninit<u8>] as *mut ());
        }
    }
}

impl<B, T> Future for SpecifiedBox<B, T>
where
    B: AlignedBuffer,
    T: Future + ?Sized,
{
    type Output = T::Output;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        unsafe { self.map_unchecked_mut(|this| &mut **this).poll(cx) }
    }
}

impl<B, T> Debug for SpecifiedBox<B, T>
where
    B: AlignedBuffer,
    T: Debug + ?Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let layout = B::LAYOUT;
        f.debug_struct("SpecifiedBox")
            .field("[BUF_SIZE]", &layout.size())
            .field("[BUF_REPR]", &layout.align())
            .finish()
    }
}
