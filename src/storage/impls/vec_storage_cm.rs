use crate::format::*;
use crate::storage::{Storage, SizedStorage, StorageMut, DynamicRowStorage, DynamicColStorage, StorageConstructor, Ownable};

#[repr(C)]
#[derive(Eq, Debug, Clone, PartialEq)]
pub struct VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	data: Vec<T>,
	row_dim: R,
	col_dim: C,
}

impl<T, R, C> VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	unsafe fn resize_element_count(&mut self, size: usize) {
		if self.data.len() > size {
			self.data.set_len(size);
			self.data.shrink_to_fit();
		} else {
			self.data.reserve_exact(size - self.data.len());
			self.data.set_len(size);
		}
	}
}

impl<T, R, C> SizedStorage<R, C> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	fn row_dim(&self) -> R { self.row_dim }

	fn col_dim(&self) -> C { self.col_dim }
}

impl<T, R, C> Storage<T, R, C> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type RStride = U1;
	type CStride = R;

	fn row_stride_dim(&self) -> Self::RStride { U1 }

	fn col_stride_dim(&self) -> Self::CStride { self.row_dim() }

	unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T { self.data.as_ptr().offset(i as isize) }
}

impl<T, R, C> StorageMut<T, R, C> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T {
		self.data.as_mut_ptr().offset(i as isize)
	}
}

impl<T, R> DynamicColStorage<T, R> for VecStorageCM<T, R, Dynamic>
	where T: Scalar, R: Dim
{
	unsafe fn set_col_count(&mut self, count: usize) {
		self.resize_element_count(count * self.row_count());
		self.col_dim = Dynamic::from(count);
	}
}

impl<T, C> DynamicRowStorage<T, C> for VecStorageCM<T, Dynamic, C>
	where T: Scalar, C: Dim
{
	unsafe fn set_row_count(&mut self, count: usize) {
		self.row_dim = Dynamic::from(count);
		let mut new_data = vec![T::default(); self.col_count() * count];

		for ci in 0..self.col_count() {
			let to = &mut new_data[ci * count..ci * count + self.row_count()];
			let from = &self.data[ci * self.row_count()..ci * self.row_count() + self.row_count()];
			to.clone_from_slice(from)
		}
	}
}

impl<T, R> StorageConstructor<T, R, Dynamic> for VecStorageCM<T, R, Dynamic>
	where T: Scalar, R: Dim
{
	fn from_value(rows: R, cols: Dynamic, value: T) -> Self {
		Self::new(rows, cols, vec![value; rows.value() * cols.value()])
	}
}

impl<T, R, C> VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	pub fn new(rows: R, cols: C, data: Vec<T>) -> Self {
		assert_eq!(rows.value() * cols.value(), data.len(), "Data size must match dimensions!");
		Self { data, row_dim: rows, col_dim: cols }
	}
}

impl<T, R, C> Ownable<T, R, C> for VecStorageCM<T, R, C>
	where T: Scalar, R: Dim, C: Dim
{
	type OwnedType = Self;

	fn owned(self) -> Self::OwnedType { self }

	fn clone_owned(&self) -> Self::OwnedType {
		Self::new(self.row_dim(), self.col_dim(), self.data.clone())
	}
}