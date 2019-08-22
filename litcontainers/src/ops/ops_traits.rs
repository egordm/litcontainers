use crate::{Scalar};

macro_rules! unary_op_trait (
	($Trait: ident, $method: ident, $TraitAssign: ident, $method_assign: ident) => {
		pub trait $Trait {
			type Output;

			fn $method(self) -> Self::Output;
		}

		pub trait $TraitAssign {
			fn $method_assign(&mut self);
		}
	}
);

unary_op_trait!(ASin, asin, ASinAssign, asin_assign);
unary_op_trait!(Sin, sin, SinAssign, sin_assign);
unary_op_trait!(ACos, acos, ACosAssign, acos_assign);
unary_op_trait!(Cos, cos, CosAssign, cos_assign);
unary_op_trait!(Tan, tan, TanAssign, tan_assign);
unary_op_trait!(ATan, atan, ATanAssign, atan_assign);
unary_op_trait!(Exp, exp, ExpAssign, exp_assign);
unary_op_trait!(Exp2, exp2, Exp2Assign, exp2_assign);
unary_op_trait!(Ceil, ceil, CeilAssign, ceil_assign);
unary_op_trait!(Floor, floor, FloorAssign, floor_assign);
unary_op_trait!(Round, round, RoundAssign, round_assign);
unary_op_trait!(Abs, abs, AbsAssign, abs_assign);
unary_op_trait!(Sqrt, sqrt, SqrtAssign, sqrt_assign);
unary_op_trait!(Log2, log2, Log2Assign, log2_assign);
unary_op_trait!(Log10, log10, Log10Assign, log10_assign);
unary_op_trait!(Ln, ln, LnAssign, ln_assign);
unary_op_trait!(Norm, norm, NormAssign, norm_assign);

macro_rules! binary_op_trait (
	($Trait: ident, $method: ident, $TraitAssign: ident, $method_assign: ident) => {
		pub trait $Trait<RHS=Self> {
			type Output;

			fn $method(self, rhs: RHS) -> Self::Output;
		}

		pub trait $TraitAssign<RHS=Self> {
		    fn $method_assign(&mut self, rhs: RHS);
		}
	}
);

binary_op_trait!(Pow, pow, PowAssign, pow_assign);
binary_op_trait!(Log, log, LogAssign, log_assign);
binary_op_trait!(Max, max, MaxAssign, max_assign);
binary_op_trait!(Min, min, MinAssign, min_assign);

macro_rules! unary_simple_op_trait (
	($Trait: ident, $method: ident) => {
		pub trait $Trait {
			type Output;

			fn $method(&self) -> Self::Output;
		}
	}
);

pub trait Clamp<R> {
	type Output;

	fn clamp(self, min: R, max: R) -> Self::Output;
}

pub trait ClampAssign<R> {
	fn clamp_assign(&mut self, min: R, max: R);
}

pub fn clamp<T: Scalar>(x: T, min: T, max: T) -> T {
	if x < min { min }
	else if x > max { max }
	else { x }
}

unary_simple_op_trait!(Sum, sum);
unary_simple_op_trait!(RowSum, row_sum);
unary_simple_op_trait!(ColSum, col_sum);
unary_simple_op_trait!(Mean, mean);

unary_simple_op_trait!(Maximum, maximum);
unary_simple_op_trait!(Minimum, minimum);

unary_simple_op_trait!(RowMean, row_mean);
unary_simple_op_trait!(ColMean, col_mean);
unary_simple_op_trait!(RowMax, row_max);
unary_simple_op_trait!(ColMax, col_max);
unary_simple_op_trait!(RowMin, row_min);
unary_simple_op_trait!(ColMin, col_min);