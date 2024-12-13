use super::*;

pub(crate) trait ConvertSelfApplier<From, Into> {
	fn convert_self_applier(from: From) -> CasterApplierEnum;
}

impl<T> ConvertSelfApplier<T, i32> for CasterApplierEnum
where CasterApplier: From<T>
{
	fn convert_self_applier(from: T) -> CasterApplierEnum { Self::from(CasterApplier::from(from)) }
}

impl<T> ConvertSelfApplier<T, i64> for CasterApplierEnum
where CasterGirlApplier: From<T>
{
	fn convert_self_applier(from: T) -> CasterApplierEnum {
		Self::OnGirl(CasterGirlApplier::from(from))
	}
}

pub(crate) trait ConvertTargetApplier<From, Marker> {
	fn convert_target_applier(from: From) -> TargetApplierEnum;
}

impl<T> ConvertTargetApplier<T, i32> for TargetApplierEnum
where TargetApplier: From<T>
{
	fn convert_target_applier(from: T) -> Self { Self::from(TargetApplier::from(from)) }
}

impl<T> ConvertTargetApplier<T, i64> for TargetApplierEnum
where TargetGirlApplier: From<T>
{
	fn convert_target_applier(from: T) -> Self { Self::OnGirl(TargetGirlApplier::from(from)) }
}

macro_rules! self_effs {
    ($($Expr:expr),* $(,)?) => {
	    vec![$( <CasterApplierEnum as ConvertSelfApplier<_, _>>::convert_self_applier($Expr) ),*]
    };
}

pub(crate) use self_effs;

macro_rules! target_effs {
    ($($Expr:expr),* $(,)?) => {
	    vec![$( <TargetApplierEnum as ConvertTargetApplier<_, _>>::convert_target_applier($Expr) ),*]
    };
}

pub(crate) use target_effs;

macro_rules! caster_fx {
	($Expr:expr) => {
		<CasterApplierEnum as ConvertSelfApplier<_, _>>::convert_self_applier($Expr)
	};
}

pub(crate) use caster_fx;

macro_rules! target_fx {
	($Expr:expr) => {
		<TargetApplierEnum as ConvertTargetApplier<_, _>>::convert_target_applier($Expr)
	};
}

pub(crate) use target_fx;
