pub trait CollectArray<T, U: Default + AsMut<[T]>>: Sized + Iterator<Item = T> {
    fn collect_array(self) -> U {
        let mut array = U::default();
        let slice = array.as_mut();
        for (index, value) in self.take(slice.len()).enumerate() {
            slice[index] = value;
        }
        array
    }
}

impl<T, U: Iterator<Item = T>, V: Default + AsMut<[T]>> CollectArray<T, V> for U {}

pub trait IntoUnit: Sized {
    fn into_unit(self) {}
}

impl<T> IntoUnit for T {}

pub fn side_effect<T, F: FnOnce()>(f: F) -> impl FnOnce(T) -> T {
    |t| {
        f();
        t
    }
}

pub fn unradix(rev_digits: impl Iterator<Item = usize>, radix: usize) -> usize {
    rev_digits
        .zip(itertools::iterate(1, |i| radix * i))
        .map(|p| p.0 * p.1)
        .sum()
}
