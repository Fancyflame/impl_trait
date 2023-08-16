use std::future::Future;

pub unsafe trait Coerce<T: ?Sized> {
    fn coerce(&self) -> &T;
    fn coerce_mut(&mut self) -> &mut T;
}

unsafe impl<'a, T, O> Coerce<dyn Future<Output = O> + 'a> for T
where
    T: Future<Output = O> + 'a,
{
    fn coerce(&self) -> &(dyn Future<Output = O> + 'a) {
        self as _
    }

    fn coerce_mut(&mut self) -> &mut (dyn Future<Output = O> + 'a) {
        self as _
    }
}
