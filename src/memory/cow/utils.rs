//! COW Utilities Module
//!
//! This module provides utility data structures and functions for Copy-on-Write
//! memory management, including no_std compatible containers and helper functions.

/// Maximum number of virtual addresses per COW page
pub const MAX_VIRT_ADDRS_PER_PAGE: usize = 8;

/// Maximum number of process IDs per COW page
#[allow(dead_code)]
pub const MAX_PROCESS_IDS_PER_PAGE: usize = 4;

/// Simple array-based vector for no_std environment
#[derive(Debug)]
pub struct SimpleVec<T> {
    data: [T; MAX_VIRT_ADDRS_PER_PAGE],
    len: usize,
}

impl<T: Copy + Default> SimpleVec<T> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); MAX_VIRT_ADDRS_PER_PAGE],
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.len < MAX_VIRT_ADDRS_PER_PAGE {
            self.data[self.len] = item;
            self.len += 1;
            Ok(())
        } else {
            Err("SimpleVec is full")
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<T, &'static str> {
        if index >= self.len {
            return Err("Index out of bounds");
        }

        let item = self.data[index];
        for i in index..self.len - 1 {
            self.data[i] = self.data[i + 1];
        }
        self.len -= 1;
        Ok(item)
    }

    pub fn contains(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        for i in 0..self.len {
            if &self.data[i] == item {
                return true;
            }
        }
        false
    }

    pub fn iter(&self) -> SimpleVecIter<'_, T> {
        SimpleVecIter {
            data: &self.data,
            len: self.len,
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Get item at index
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.data[index])
        } else {
            None
        }
    }
}

/// Iterator for SimpleVec
pub struct SimpleVecIter<'a, T> {
    data: &'a [T; MAX_VIRT_ADDRS_PER_PAGE],
    len: usize,
    index: usize,
}

impl<'a, T> Iterator for SimpleVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = &self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

/// Process ID array for COW pages
pub type ProcessIdArray = SimpleVec<usize>;
