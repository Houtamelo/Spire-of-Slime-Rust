use super::*;

pub(super) trait IApplyOnAny {
	fn apply_on_any(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	);
}

impl<T: IApplyOnAny> IApplyOnCaster for T {
	fn apply_on_caster(&self, ctx: &mut ActorContext, caster: &mut Ptr<Actor>, is_crit: bool) {
		let mut target = caster.clone();
		self.apply_on_any(ctx, caster, &mut target, is_crit);
	}
}

impl<T: IApplyOnAny> IApplyOnTarget for T {
	fn apply_on_target(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) {
		self.apply_on_any(ctx, caster, target, is_crit);
	}
}
