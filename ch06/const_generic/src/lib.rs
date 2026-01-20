#[derive(Debug)]
pub struct Buffer<T, const LENGTH: usize> {
    buf: [T; LENGTH],
}

impl<T, const LENGTH: usize> From<[T; LENGTH]> for Buffer<T, LENGTH> {
    fn from(buf: [T; LENGTH]) -> Self {
        Self { buf }
    }
}

impl<T: Default + Copy, const LENGTH: usize> From<Vec<T>> for Buffer<T, LENGTH> {
    fn from(v: Vec<T>) -> Self {
        assert_eq!(LENGTH, v.len());
        let mut ret = Self {
            buf: [T::default(); LENGTH],
        };
        ret.buf.copy_from_slice(&v);
        ret
    }
}
