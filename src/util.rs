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

pub struct DrainFilterMap<'a, T: 'a, U, F>
where
    F: FnMut(&mut T) -> Option<U>,
{
    vec: &'a mut Vec<T>,
    idx: usize,
    del: usize,
    old_len: usize,
    filter: F,
}

impl<'a, T, U, F> Iterator for DrainFilterMap<'a, T, U, F>
where
    F: FnMut(&mut T) -> Option<U>,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx != self.old_len {
            let i = self.idx;
            self.idx += 1;
            let v = unsafe { std::slice::from_raw_parts_mut(self.vec.as_mut_ptr(), self.old_len) };
            match (self.filter)(&mut v[i]) {
                Some(o) => {
                    self.del += 1;
                    return Some(o);
                }
                None => v.swap(i - self.del, i),
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.old_len - self.idx))
    }
}

impl<'a, T, U, F> Drop for DrainFilterMap<'a, T, U, F>
where
    F: FnMut(&mut T) -> Option<U>,
{
    fn drop(&mut self) {
        for _ in self.by_ref() {}
        unsafe { self.vec.set_len(self.old_len - self.del) }
    }
}

pub trait DrainFilterMappable<T, U> {
    /// Remove elements returning Some(U) and iterate over the returned U's.
    fn drain_filter_map<F: FnMut(&mut T) -> Option<U>>(
        &mut self,
        filter: F,
    ) -> DrainFilterMap<T, U, F>;
}

impl<T, U> DrainFilterMappable<T, U> for Vec<T> {
    fn drain_filter_map<F: FnMut(&mut T) -> Option<U>>(
        &mut self,
        filter: F,
    ) -> DrainFilterMap<T, U, F> {
        let old_len = self.len();
        unsafe { self.set_len(0) };
        DrainFilterMap {
            vec: self,
            idx: 0,
            del: 0,
            old_len,
            filter,
        }
    }
}

pub fn unradix(rev_digits: impl Iterator<Item = usize>, radix: usize) -> usize {
    rev_digits
        .zip(itertools::iterate(1, |i| radix * i))
        .map(|p| p.0 * p.1)
        .sum()
}
