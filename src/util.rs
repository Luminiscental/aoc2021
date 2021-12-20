use hashbrown::HashSet;
use std::{
    cmp::Ordering,
    collections::VecDeque,
    hash::Hash,
    iter::{self, Sum},
    ops::AddAssign,
};

pub struct BitSet(Vec<u64>);

impl BitSet {
    fn unpack(value: u32) -> (usize, usize) {
        ((value / 64) as usize, (value & 63) as usize)
    }

    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.iter().map(|block| block.count_ones() as usize).sum()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn reserve(&mut self, capacity: usize) {
        self.0.extend(iter::repeat(0).take((capacity + 63) / 64));
    }

    pub fn insert(&mut self, value: u32) {
        let (chunk, index) = Self::unpack(value);
        if chunk >= self.0.len() {
            self.0
                .extend(iter::repeat(0).take(1 + chunk - self.0.len()));
        }
        self.0[chunk] |= 1 << index;
    }

    #[allow(dead_code)]
    pub fn contains(&self, value: u32) -> bool {
        let (chunk, index) = Self::unpack(value);
        self.0
            .get(chunk)
            .map_or(false, |chunk| chunk & (1 << index) != 0)
    }

    #[inline(always)]
    pub unsafe fn contains_unchecked(&self, value: u32) -> bool {
        let (chunk, index) = Self::unpack(value);
        (self.0.get_unchecked(chunk) & (1 << index)) != 0
    }
}

pub fn grid_neighbours(
    point: (u32, u32),
    width: u32,
    height: u32,
) -> impl Iterator<Item = (u32, u32)> {
    [(1, 0), (0, 1), (u32::MAX, 0), (0, u32::MAX)]
        .iter()
        .map(move |delta| (point.0.wrapping_add(delta.0), point.1.wrapping_add(delta.1)))
        .filter(move |&point| point.0 < width && point.1 < height)
}

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

pub fn unradix(rev_digits: impl Iterator<Item = u64>, radix: u64) -> u64 {
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
