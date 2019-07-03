use crate::format::*;
use crate::storage::*;
use std::marker::PhantomData;
use crate::container::Container;

pub type Slice<'a, T, R, RS, C, CS> = SliceBase<'a, T, R, C, PtrStorage<'a, T, R, RS, C, CS>>;
pub type SliceMut<'a, T, R, RS, C, CS> = SliceBase<'a, T, R, C, PtrMutStorage<'a, T, R, RS, C, CS>>;

pub type RowSlice<'a, T, R, C> = Slice<'a, T, R, C, C, U1>;
pub type RowSliceMut<'a, T, R, C> = SliceMut<'a, T, R, C, C, U1>;
pub type ColSlice<'a, T, R, C> = Slice<'a, T, R, U1, C, R>;
pub type ColSliceMut<'a, T, R, C> = SliceMut<'a, T, R, U1, C, R>;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	pub(crate) storage: S,
	pub(crate) _phantoms: PhantomData<(&'a (), T, R, C, S)>
}

impl<'a, T, R, C, S> SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	pub fn new(storage: S) -> Self {
		SliceBase { storage, _phantoms: PhantomData }
	}
}

impl<'a, T, R, C, S> SizedStorage<R, C> for SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	fn row_dim(&self) -> R { self.storage.row_dim() }

	fn col_dim(&self) -> C { self.storage.col_dim() }
}

impl<'a, T, R, C, S> Storage<T, R, C> for SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	type RStride = S::RStride;
	type CStride = S::CStride;

	fn row_stride_dim(&self) -> Self::RStride { self.storage.row_stride_dim() }

	fn col_stride_dim(&self) -> Self::CStride { self.storage.col_stride_dim() }

	unsafe fn get_index_ptr_unchecked(&self, i: usize) -> *const T { self.storage.get_index_ptr_unchecked(i) }
}

impl<'a, T, R, C, S> Ownable<T, R, C> for SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>
{
	type OwnedType = Container<T, R, C, S::OwnedType>;

	fn owned(self) -> Self::OwnedType {
		Container {
			storage: self.storage.owned(),
			_phantoms: PhantomData
		}
	}

	fn clone_owned(&self) -> Self::OwnedType {
		Container {
			storage: self.storage.clone_owned(),
			_phantoms: PhantomData
		}
	}
}

impl<'a, T, R, C, S> StorageMut<T, R, C> for SliceBase<'a, T, R, C, S>
	where T: Scalar, R: Dim, C: Dim, S: StorageMut<T, R, C>
{
	unsafe fn get_index_mut_ptr_unchecked(&mut self, i: usize) -> *mut T { self. storage.get_index_mut_ptr_unchecked(i) }
}