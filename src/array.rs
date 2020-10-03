use std::mem::{self, MaybeUninit};
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Array<T: Copy, const N: usize> {
    len: usize,
    inner: [T; N],
}

impl<T: Copy, const N: usize> Array<T, N> {
    pub fn new() -> Self {
        let inner = unsafe { MaybeUninit::uninit().assume_init() };
        Self { inner, len: 0 }
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn reserve_exact<const M: usize>(mut self) -> Array<T, M> {
        assert!(M >= N, "cannot reserve into an Array of a smaller capacity");
        let len = self.len;
        let mut inner = unsafe { MaybeUninit::<[T; M]>::uninit().assume_init() };
        for i in 0..len {
            mem::swap(&mut inner[i], &mut self.inner[i]);
        }
        mem::forget(self);
        Array { inner, len }
    }

    pub fn truncate(&mut self, len: usize) {
        for i in len..self.len {
            let mut t = unsafe { MaybeUninit::uninit().assume_init() };
            mem::swap(&mut t, &mut self.inner[i]);
            mem::forget(t);
        }
        self.len = self.len.min(len);
    }

    pub fn as_slice(&self) -> &[T] {
        &self.inner[..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.inner[..self.len]
    }

    pub fn as_ptr(&self) -> *const T {
        &self.inner[0] as *const _
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        &mut self.inner[0] as *mut _
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        assert!(self.len > 0, "attempt to remove from an empty array");
        self.len -= 1;
        let mut t = unsafe { MaybeUninit::uninit().assume_init() };
        mem::swap(&mut t, &mut self.inner[self.len]);
        mem::swap(&mut t, &mut self.inner[index]);
        t
    }

    pub fn insert(&mut self, index: usize, mut element: T) {
        let mut t = unsafe { MaybeUninit::uninit().assume_init() };
        mem::swap(&mut t, &mut self.inner[index]);
        mem::swap(&mut self.inner[index], &mut element);
        mem::forget(element);
        self.len += 1;
        for i in index + 1..self.len {
            let mut u = unsafe { MaybeUninit::uninit().assume_init() };
            mem::swap(&mut u, &mut self.inner[i]);
            mem::swap(&mut self.inner[i], &mut t);
            mem::swap(&mut t, &mut u);
            mem::forget(u);
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(self.len > 0, "attempt to remove from an empty array");
        let mut t = unsafe { MaybeUninit::uninit().assume_init() };
        mem::swap(&mut t, &mut self.inner[index]);
        self.len -= 1;
        for i in index..self.len {
            self.inner.swap(i, i + 1);
        }
        t
    }

    pub fn push(&mut self, mut value: T) {
        assert!(
            self.len < N,
            "pushing would result in exceeding the capacity, which is {:?}",
            N
        );

        mem::swap(&mut self.inner[self.len], &mut value);
        mem::forget(value);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            let mut t = unsafe { MaybeUninit::uninit().assume_init() };
            self.len -= 1;
            mem::swap(&mut t, &mut self.inner[self.len]);
            Some(t)
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.len {
            let mut t = unsafe { MaybeUninit::uninit().assume_init() };
            mem::swap(&mut t, &mut self.inner[i]);
            mem::drop(t);
        }
        self.len = 0;
    }
}

impl<T: Copy, const N: usize> IntoIterator for Array<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(mut self) -> IntoIter<T, N> {
        let len = self.len;
        let mut inner = unsafe { MaybeUninit::uninit().assume_init() };
        mem::swap(&mut inner, &mut self.inner);
        mem::forget(self);
        IntoIter { idx: 0, len, inner }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IntoIter<T: Copy, const N: usize> {
    idx: usize,
    len: usize,
    inner: [T; N],
}

impl<T: Copy, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.len {
            None
        } else {
            let mut t = unsafe { MaybeUninit::uninit().assume_init() };
            mem::swap(&mut t, &mut self.inner[self.idx]);
            self.idx += 1;
            Some(t)
        }
    }
}

impl<T: Copy, const N: usize> Deref for Array<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.inner[..self.len]
    }
}

impl<T: Copy, const N: usize> DerefMut for Array<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner[..self.len]
    }
}

impl<T: Copy, const N: usize> Index<usize> for Array<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        assert!(
            idx < self.len,
            "index out of bounds, index is {:?} but len is {:?}",
            idx,
            self.len
        );
        &self.inner[idx]
    }
}

impl<T: Copy, const N: usize> IndexMut<usize> for Array<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        assert!(
            idx < self.len,
            "index out of bounds, index is {:?} but len is {:?}",
            idx,
            self.len
        );
        &mut self.inner[idx]
    }
}
