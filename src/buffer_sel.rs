use std::alloc::Layout;

pub trait BufferSelector<const REPR: usize, const SIZE: usize> {
    type Align;
}

pub unsafe trait AlignedBuffer: Default + Unpin {
    const LAYOUT: Layout;
    fn get_buffer(&self) -> &[u8];
    fn get_buffer_mut(&mut self) -> &mut [u8];
}

macro_rules! align_n {
    ($($AlignN:ident $align:literal,)*) => {
        $(
            #[repr(align($align))]
            pub struct $AlignN<const SIZE: usize>([u8; SIZE]);

            impl<const SIZE: usize> Default for $AlignN<SIZE> {
                fn default() -> Self {
                    Self([0u8; SIZE])
                }
            }

            impl<const SIZE: usize> BufferSelector<$align, SIZE> for () {
                type Align = $AlignN<SIZE>;
            }

            unsafe impl<const SIZE: usize> AlignedBuffer for $AlignN<SIZE> {
                const LAYOUT: Layout = unsafe {
                    Layout::from_size_align_unchecked(SIZE, $align)
                };

                fn get_buffer(&self) -> &[u8] {
                    &self.0
                }

                fn get_buffer_mut(&mut self) -> &mut [u8] {
                    &mut self.0
                }
            }
        )*
    };
}

align_n! {
    Align1  1,
    Align2  2,
    Align4  4,
    Align8  8,
    Align16 16,
}
