use crate::format::*;
use crate::storage::StorageMut;
use crate::{InplaceMap, Container};

/// Type can be turned or cloned into a container which owns its data.
pub trait Ownable<T: Element> {
	type OwnedType: StorageMut<T> + InplaceMap<T>;

	/// Converts itself to a container which owns its data. No guarantees that it wont be the same
	/// container if it is already owns its data.
	fn owned(self) -> Container<T, Self::OwnedType>;

	/// Clones it's data into a container which owns its data.
	fn clone_owned(&self) -> Container<T, Self::OwnedType>;
}