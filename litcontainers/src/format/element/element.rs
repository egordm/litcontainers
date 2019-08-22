use std::fmt::{Debug, Display};
use std::fmt;
use num_traits::{Float, cast::cast};
use num_complex::Complex;
use super::numeric::*;
use super::super::print::*;
use crate::{NumericElement, Scalar, ScalarType};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ElementType {
	Bool,
	U8,
	I8,
	U16,
	I16,
	U32,
	I32,
	U64,
	I64,
	U128,
	I128,
	F32,
	F64,
	Complex(ScalarType)
}

pub trait Element: Copy + Clone + Debug + Sized + Default + Send + Sync
{
	fn element_type() -> ElementType;

	fn fmt_elem(&self, f: &mut fmt::Formatter, precision: usize, sci: bool) -> Result<(), fmt::Error>;

	fn numeric() -> bool { true }

	type NumericType: NumericElement + From<Self>;
}

impl Element for bool {
	fn element_type() -> ElementType { ElementType::Bool }

	fn fmt_elem(&self, f: &mut fmt::Formatter, _: usize, _: bool) -> Result<(), fmt::Error> { write!(f, "{}", self) }

	fn numeric() -> bool { false }

	type NumericType = u8;
}

impl Element for u8 {
	fn element_type() -> ElementType { ElementType::U8 }

	fn fmt_elem(&self, f: &mut fmt::Formatter, _: usize, _: bool) -> Result<(), fmt::Error> { write!(f, "{}", self) }

	type NumericType = Self;
}

impl Element for i8 {
	fn element_type() -> ElementType { ElementType::I8 }

	fn fmt_elem(&self, f: &mut fmt::Formatter, _: usize, _: bool) -> Result<(), fmt::Error> { write!(f, "{}", self) }

	type NumericType = Self;
}

impl Element for u16 {
	fn element_type() -> ElementType { ElementType::U16 }

	fn fmt_elem(&self, f: &mut fmt::Formatter, _: usize, _: bool) -> Result<(), fmt::Error> { write!(f, "{}", self) }

	type NumericType = Self;
}

impl Element for i16 {
	fn element_type() -> ElementType { ElementType::I16 }

	fn fmt_elem(&self, f: &mut fmt::Formatter, _: usize, _: bool) -> Result<(), fmt::Error> { write!(f, "{}", self) }

	type NumericType = Self;
}

impl Element for u32 {
	fn element_type() -> ElementType { ElementType::U32 }

	fn fmt_elem(&self, f: &mut fmt::Formatter, precision: usize, sci: bool) -> Result<(), fmt::Error> {
		if sci {
			write!(f, "{:+.*e}", precision, *self as f64)
		} else {
			write!(f, "{}", *self)
		}
	}

	type NumericType = Self;
}

impl Element for i32 {
	fn element_type() -> ElementType { ElementType::I32 }

	fn fmt_elem(&self, f: &mut fmt::Formatter, precision: usize, sci: bool) -> Result<(), fmt::Error> {
		if sci {
			write!(f, "{:+.*e}", precision, *self as f64)
		} else {
			write!(f, "{}", *self)
		}
	}

	type NumericType = Self;
}

impl Element for u64 {
	fn element_type() -> ElementType { ElementType::U64 }

	fn fmt_elem(&self, f: &mut fmt::Formatter, precision: usize, sci: bool) -> Result<(), fmt::Error> {
		if sci {
			write!(f, "{:+.*e}", precision, *self as f64)
		} else {
			write!(f, "{}", *self)
		}
	}

	type NumericType = Self;
}

impl Element for i64 {
	fn element_type() -> ElementType { ElementType::I64 }

	fn fmt_elem(&self, f: &mut fmt::Formatter, precision: usize, sci: bool) -> Result<(), fmt::Error> {
		if sci {
			write!(f, "{:+.*e}", precision, *self as f64)
		} else {
			write!(f, "{}", *self)
		}
	}

	type NumericType = Self;
}

impl Element for u128 {
	fn element_type() -> ElementType { ElementType::U128 }

	fn fmt_elem(&self, f: &mut fmt::Formatter, precision: usize, sci: bool) -> Result<(), fmt::Error> {
		if sci {
			write!(f, "{:+.*e}", precision, *self as f64)
		} else {
			write!(f, "{}", *self)
		}
	}

	type NumericType = Self;
}

impl Element for i128 {
	fn element_type() -> ElementType { ElementType::I128 }

	fn fmt_elem(&self, f: &mut fmt::Formatter, precision: usize, sci: bool) -> Result<(), fmt::Error> {
		if sci {
			write!(f, "{:+.*e}", precision, *self as f64)
		} else {
			write!(f, "{}", *self)
		}
	}

	type NumericType = Self;
}

impl Element for f32 {
	fn element_type() -> ElementType { ElementType::F32 }

	fn fmt_elem(&self, f: &mut fmt::Formatter, precision: usize, sci: bool) -> Result<(), fmt::Error> {
		if sci {
			write!(f, "{:+.*e}", precision, *self as f64)
		} else {
			write!(f, "{:.*}", precision, *self as f64)
		}
	}

	type NumericType = Self;
}

impl Element for f64 {
	fn element_type() -> ElementType { ElementType::F64 }

	fn fmt_elem(&self, f: &mut fmt::Formatter, precision: usize, sci: bool) -> Result<(), fmt::Error> {
		if sci {
			write!(f, "{:+.*e}", precision, *self as f64)
		} else {
			write!(f, "{:.*}", precision, *self as f64)
		}
	}

	type NumericType = Self;
}

impl<T: Scalar + Float> Element for Complex<T> {
	fn element_type() -> ElementType { ElementType::Complex(T::scalar_type()) }

	fn fmt_elem(&self, f: &mut fmt::Formatter, precision: usize, sci: bool) -> Result<(), fmt::Error> {
		write!(f, "({: >11},{: >11})",
		       Fmt(|f| self.re.fmt_elem(f, precision, sci)),
		       Fmt(|f| self.im.fmt_elem(f, precision, sci))
		)
	}

	type NumericType = Self;
}