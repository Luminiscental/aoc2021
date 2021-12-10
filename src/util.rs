use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    hash::Hash,
    iter::Sum,
    ops::AddAssign,
};

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

pub fn qselect<T: Ord>(k: usize, slice: &mut [T]) -> &T {
    fn median_of_three<T: Ord>(slice: &[T]) -> usize {
        let (i1, i2, i3) = (0, slice.len() / 2, slice.len() - 1);
        let (x1, x2, x3) = (&slice[i1], &slice[i2], &slice[i3]);
        let is_median = |median, a, b| median < a && median > b || median < b && median > a;
        if is_median(x1, x2, x3) {
            i1
        } else if is_median(x2, x3, x1) {
            i2
        } else {
            i3
        }
    }

    fn partition<T: Ord>(slice: &mut [T], pivot_index: usize) -> usize {
        slice.swap(pivot_index, slice.len() - 1);
        let mut store_index = 0;
        for i in 0..slice.len() {
            if slice[i] < slice[slice.len() - 1] {
                slice.swap(i, store_index);
                store_index += 1;
            }
        }
        slice.swap(slice.len() - 1, store_index);
        store_index
    }

    assert!(k < slice.len());
    match slice.len() {
        1 => &slice[k],
        _ => {
            let pivot_index = partition(slice, median_of_three(slice));
            match k.cmp(&pivot_index) {
                Ordering::Equal => &slice[k],
                Ordering::Less => qselect(k, &mut slice[..pivot_index]),
                Ordering::Greater => qselect(k - pivot_index - 1, &mut slice[pivot_index + 1..]),
            }
        }
    }
}

pub fn bfs<N, F, G, I>(root: N, adjacents: F, mut visit: G) -> HashSet<N>
where
    N: Eq + Hash + Copy,
    F: Fn(N) -> I,
    I: Iterator<Item = N>,
    G: FnMut(N),
{
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_front(root);
    visited.insert(root);
    while let Some(node) = queue.pop_back() {
        for x in adjacents(node) {
            if !visited.contains(&x) {
                visit(x);
                queue.push_front(x);
                visited.insert(x);
            }
        }
    }
    visited
}

pub struct Summation<T>(pub T);

impl<T: AddAssign + Sum> Extend<T> for Summation<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0 += iter.into_iter().sum();
    }
}

impl<T: Default> Default for Summation<T> {
    fn default() -> Self {
        Self(T::default())
    }
}
