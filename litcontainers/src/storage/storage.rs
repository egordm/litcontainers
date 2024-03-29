use crate::format::*;
use crate::storage::*;
use crate::iterator::*;
use crate::slice::{SliceRange};
use std::fmt::Debug;
use std::slice;
use crate::{Sliceable, Container};
use std::ops::Index;

// TODO: implement proper equality?
pub trait Storage<T>: StorageSize + Strided + Debug + Sized + Ownable<T> + Send + Sync + InplaceForeach<T> + Index<usize, Output=T>
	where T: Element
{
	#[inline]
	fn as_ptr(&self) -> *const T;

	#[inline]
	fn as_slice(&self) -> &[T] {
		unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
	}

	#[inline]
	fn get(&self, r: usize, c: usize) -> T {
		assert!(r < self.rows(), "Out of range row!");
		assert!(c < self.cols(), "Out of range col!");
		unsafe { self.get_unchecked(r, c) }
	}

	#[inline]
	fn get_ptr(&self, r: usize, c: usize) -> *const T {
		assert!(r < self.rows(), "Out of range row!");
		assert!(c < self.cols(), "Out of range col!");
		unsafe { self.get_ptr_unchecked(r, c) }
	}

	#[inline]
	unsafe fn get_ptr_unchecked(&self, r: usize, c: usize) -> *const T {
		self.as_ptr().offset(self.get_index(r, c) as isize)
	}

	#[inline]
	unsafe fn get_unchecked(&self, r: usize, c: usize) -> T {
		*self.as_ptr().offset(self.get_index(r, c) as isize)
	}

	#[inline]
	fn get_ref(&self, r: usize, c: usize) -> &T {
		assert!(r < self.rows(), "Out of range row!");
		assert!(c < self.cols(), "Out of range col!");
		unsafe { self.get_ref_unchecked(r, c) }
	}

	#[inline]
	unsafe fn get_ref_unchecked(&self, r: usize, c: usize) -> &T { &*self.as_ptr().offset(self.get_index(r, c) as isize) }

	#[inline]
	fn as_row_ptr(&self, p: usize) -> *const T {
		assert!(p < self.rows(), "Row out of bounds!");
		unsafe { self.as_row_ptr_unchecked(p) }
	}

	#[inline]
	unsafe fn as_row_ptr_unchecked(&self, p: usize) -> *const T { self.as_ptr().offset(self.row_index(p) as isize) }

	#[inline]
	fn as_col_ptr(&self, v: usize) -> *const T {
		assert!(v < self.cols(), "Col out of bounds!");
		unsafe { self.as_col_ptr_unchecked(v) }
	}

	#[inline]
	unsafe fn as_col_ptr_unchecked(&self, p: usize) -> *const T { self.as_ptr().offset(self.col_index(p) as isize) }

	// Iterator
	fn iter(self) -> FullAxisIterOwned<T, Self, RowAxis> {
		FullAxisIterOwned::<T, Self, RowAxis>::from_storage(self, RowAxis)
	}

	fn as_iter(&self) -> FullAxisIter<T, Self, RowAxis> { self.as_row_iter() }

	fn as_row_iter(&self) -> FullAxisIter<T, Self, RowAxis> { FullIter::from_storage(self, RowAxis) }

	fn as_row_slice_iter(&self) -> RowSliceIter<T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride> { RowSliceIter::from_storage(self) }

	fn as_row_range_iter<RR: SliceRange>(&self, range: RR)
		-> FullIter<T, RR::Size, Self::RowStride, Self::ColStride>
	{
		FullIter::from_storage_range(self, RowAxis, range)
	}

	fn as_col_iter(&self) -> FullAxisIter<T, Self, ColAxis> { FullIter::from_storage(self, ColAxis) }

	fn as_col_slice_iter(&self) -> ColSliceIter<T, Self::Rows, Self::RowStride, Self::Cols, Self::ColStride> { ColSliceIter::from_storage(self) }

	fn as_col_range_iter<CR: SliceRange>(&self, range: CR)
		-> FullIter<T, CR::Size, Self::ColStride, Self::RowStride>
	{
		FullIter::from_storage_range(self, ColAxis, range)
	}

	// Container
	fn into_container(self) -> Container<T, Self> { self.into() }
}

impl<T: Element, S: Storage<T>> Sliceable<T> for S {}

impl<T: Element, S: Storage<T>> IntoOrderedIterator<T> for S {
	type IntoIter = FullAxisIterOwned<T, Self, RowAxis>;

	fn into_ordered_iter(self) -> Self::IntoIter { self.iter() }
}
