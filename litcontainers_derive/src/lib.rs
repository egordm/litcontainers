#![recursion_limit = "128"]

extern crate proc_macro;
extern crate syn;
extern crate quote;
#[macro_use]
extern crate synstructure;

mod utils;
#[macro_use] mod storage_meta_info;
mod storage_proc_macros;

use proc_macro::{TokenStream};

#[proc_macro_derive(Strided, attributes(storage_field, dimension_fields))]
pub fn strided_storage_derive(input: TokenStream) -> TokenStream {
	storage_proc_macros::strided_storage_derive(input)
}

#[proc_macro_derive(StorageSize, attributes(storage_field, dimension_fields))]
pub fn sized_storage_derive(input: TokenStream) -> TokenStream {
	storage_proc_macros::sized_storage_derive(input)
}

#[proc_macro_derive(Storage, attributes(storage_field, dimension_fields))]
pub fn storage_derive(input: TokenStream) -> TokenStream {
	storage_proc_macros::storage_derive(input)
}

#[proc_macro_derive(StorageMut, attributes(storage_field, dimension_fields))]
pub fn storage_mut_derive(input: TokenStream) -> TokenStream {
	storage_proc_macros::storage_mut_derive(input)
}

#[proc_macro_derive(Ownable, attributes(storage_field, dimension_fields))]
pub fn ownable_derive(input: TokenStream) -> TokenStream {
	storage_proc_macros::ownable_derive(input)
}