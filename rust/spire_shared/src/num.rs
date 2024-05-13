use comfy_bounded_ints::prelude::*;
use util::prelude::*;

pub type PercentageU8 = Bound_u8<0, 100>;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub struct Percentage_0_100 {
	inner_value: f64,
}

bound_f64_impl!(Percentage_0_100, 0.0, 100.0);

pub trait ToSaturatedI64 {
	fn to_sat_i64(self) -> SaturatedI64; 
}

pub trait ToSaturatedU64 {
	fn to_sat_u64(self) -> SaturatedU64;
}

pub trait ToU8Percentage {
	fn to_percent_u8(self) -> PercentageU8;
}

macro_rules! impl_to_sat {
    ($ty_name: ty) => {
	    impl ToSaturatedI64 for $ty_name {
		    fn to_sat_i64(self) -> SaturatedI64 {
			    return SaturatedI64::new(self.squeeze_to());
		    }
	    }
	    
	    impl ToSaturatedU64 for $ty_name {
		    fn to_sat_u64(self) -> SaturatedU64 {
			    return SaturatedU64::new(self.squeeze_to());
		    }
	    }
	    
	    impl ToU8Percentage for $ty_name {
		    fn to_percent_u8(self) -> PercentageU8 {
			    return PercentageU8::new(self.squeeze_to());
		    }
	    }
    };
}

impl_to_sat!(i8);
impl_to_sat!(i16);
impl_to_sat!(i32);
impl_to_sat!(i64);
impl_to_sat!(isize);

impl_to_sat!(u8);
impl_to_sat!(u16);
impl_to_sat!(u32);
impl_to_sat!(u64);
impl_to_sat!(usize);

macro_rules! impl_to_sat_generic {
    ($ty_name: ty, $int_ty: ty) => {
	    impl<const MIN: $int_ty, const MAX: $int_ty> ToSaturatedI64 for $ty_name {
		    fn to_sat_i64(self) -> SaturatedI64 {
			    return SaturatedI64::new(self.squeeze_to());
		    }
	    }
	    
	    impl<const MIN: $int_ty, const MAX: $int_ty> ToSaturatedU64 for $ty_name {
		    fn to_sat_u64(self) -> SaturatedU64 {
			    return SaturatedU64::new(self.squeeze_to());
		    }
	    }
	    
	    impl<const MIN: $int_ty, const MAX: $int_ty> ToU8Percentage for $ty_name {
		    fn to_percent_u8(self) -> PercentageU8 {
			    return PercentageU8::new(self.squeeze_to());
		    }
	    }
    };
}

impl_to_sat_generic!(Bound_i8<MIN, MAX>, i8);
impl_to_sat_generic!(Bound_i16<MIN, MAX>, i16);
impl_to_sat_generic!(Bound_i32<MIN, MAX>, i32);
impl_to_sat_generic!(Bound_i64<MIN, MAX>, i64);
impl_to_sat_generic!(Bound_i128<MIN, MAX>, i128);
impl_to_sat_generic!(Bound_isize<MIN, MAX>, isize);

impl_to_sat_generic!(Bound_u8<MIN, MAX>, u8);
impl_to_sat_generic!(Bound_u16<MIN, MAX>, u16);
impl_to_sat_generic!(Bound_u32<MIN, MAX>, u32);
impl_to_sat_generic!(Bound_u64<MIN, MAX>, u64);
impl_to_sat_generic!(Bound_u128<MIN, MAX>, u128);
impl_to_sat_generic!(Bound_usize<MIN, MAX>, usize);