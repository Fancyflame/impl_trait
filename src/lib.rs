use std::alloc::Layout;

pub use buffer_sel::BufferSelector;
pub use specified_box::SpecifiedBox;

pub mod buffer_sel;
pub mod coerce;
pub mod specified_box;

pub const fn layout_of_ret<F: FnOnce() -> R, R>(_: &F) -> Layout {
    Layout::new::<R>()
}

#[cfg(test)]
mod test {
    //! This is what we need
    //! ```
    //! trait MyTrait {
    //!    async fn call() -> String;
    //! }
    //!
    //! impl MyTrait for Foo {
    //!     async fn call() -> String {
    //!         "hello world".into()
    //!     }
    //! }
    //! ```
    //! It should expanded to the code in
    //! this module.

    use super::*;

    trait MyTrait {
        type Output;
        fn call() -> Self::Output;
    }

    struct Foo;

    /*async fn call() -> String {
        "hello world".into()
    }*/

    #[repr(C)]
    struct WithPadding {
        small: u8,
        large: u32,
    }

    async fn call() -> String {
        let val = WithPadding {
            small: 0,
            large: 10,
        };
        std::future::ready(()).await;
        drop(val);
        "hello world".into()
    }

    impl MyTrait for Foo {
        type Output = SpecifiedBox<
            <() as BufferSelector<
                { layout_of_ret(&call).align() },
                { layout_of_ret(&call).size() },
            >>::Align,
            dyn ::std::future::Future<Output = String>,
        >;

        fn call() -> Self::Output {
            Self::Output::new(call())
        }
    }

    #[test]
    fn main() {
        let boxed = <Foo as MyTrait>::call();
        pollster::block_on(boxed);
        //assert_eq!(pollster::block_on(boxed), "hello world");
    }
}
