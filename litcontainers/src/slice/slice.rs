use crate::format::*;
use crate::storage::*;
use std::marker::PhantomData;
use crate::Container;
use std::ops::{Index, IndexMut};


/// Slice containing references to scalar values.
pub type Slice<'a, T, R, RS, C, CS> = Container<T, SliceBase<'a, T, PtrStorage<'a, T, R, RS, C, CS>>>;
/// Slice containing mutable references to scalar values.
pub type SliceMut<'a, T, R, RS, C, CS> = Container<T, SliceBase<'a, T, PtrStorageMut<'a, T, R, RS, C, CS>>>;

/// Container containing references to scalar values.
#[repr(C)]
#[derive(Debug, Storage, StorageSize, Strided, Ownable, new)]
pub struct SliceBase<'a, T, S>
	where T: Element, S: Storage<T>
{
	pub(crate) storage: S,
	pub(crate) _phantoms: PhantomData<(&'a (), T, S)>
}

impl<'a, T, S> SliceBase<'a, T, S>
	where T: Element, S: Storage<T>
{
	pub fn into_storage(self) -> S { self.storage }

	pub fn storage(&self) -> &S { &self.storage}

	pub fn storage_mut(&mut self) -> &mut S { &mut self.storage}
}

impl<'a, T, S> StorageMut<T> for SliceBase<'a, T, S>
	where T: Element, S: StorageMut<T>
{
	fn as_ptr_mut(&mut self) -> *mut T { self.storage.as_ptr_mut() }
}

impl<'a, T, S> InplaceMap<T> for SliceBase<'a, T, S>
	where T: Element, S: StorageMut<T> + InplaceMap<T>
{
	fn map_inplace<F: FnMut(&mut T)>(&mut self, f: F) { self.storage.map_inplace(f) }
}

impl<'a, T, S> InplaceMapOrdered<T> for SliceBase<'a, T, S>
	where T: Element, S: StorageMut<T> + InplaceMapOrdered<T>
{
	fn map_inplace_ordered<F: FnMut(&mut T)>(&mut self, f: F) { self.storage.map_inplace_ordered(f) }
}

impl<'a, T, S> IndexMut<usize> for SliceBase<'a, T, S>
	where T: Element, S: StorageMut<T> + IndexMut<usize>
{
	fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.storage_mut().index_mut(index) }
}

impl<'a, T, R, RS, C, CS> Slice<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	pub fn split_at_row<P: Dim>(self, pos: P)
		-> (
			Slice<'a, T, P, RS, C, CS>,
			Slice<'a, T, <R as DimSub<P>>::Output, RS, C, CS>,
		)
		where P: Dim, R: DimSub<P>
	{
		let (l, r) = self.into_storage().into_storage().split_at_row(pos);
		(l.into(), r.into())
	}

	pub fn split_at_col<P: Dim>(self, pos: P)
		-> (
			Slice<'a, T, R, RS, P, CS>,
			Slice<'a, T, R, RS, <C as DimSub<P>>::Output, CS>,
		)
		where P: Dim, C: DimSub<P>
	{
		let (l, r) = self.into_storage().into_storage().split_at_col(pos);
		(l.into(), r.into())
	}
}

impl<'a, T, R, RS, C, CS> SliceMut<'a, T, R, RS, C, CS>
	where T: Element, R: Dim, RS: Dim, C: Dim, CS: Dim
{
	pub fn split_at_row<P: Dim>(self, pos: P)
		-> (
			SliceMut<'a, T, P, RS, C, CS>,
			SliceMut<'a, T, <R as DimSub<P>>::Output, RS, C, CS>,
		)
		where P: Dim, R: DimSub<P>
	{
		let (l, r) = self.into_storage().into_storage().split_at_row(pos);
		(l.into(), r.into())
	}

	pub fn split_at_col<P: Dim>(self, pos: P)
		-> (
			SliceMut<'a, T, R, RS, P, CS>,
			SliceMut<'a, T, R, RS, <C as DimSub<P>>::Output, CS>,
		)
		where P: Dim, C: DimSub<P>
	{
		let (l, r) = self.into_storage().into_storage().split_at_col(pos);
		(l.into(), r.into())
	}
}