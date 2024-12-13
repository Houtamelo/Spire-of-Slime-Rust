use super::*;

pub(super) trait IApplyOnAnyGirl {
	fn apply_on_any_girl(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		is_crit: bool,
	);
}

impl<T: IApplyOnAnyGirl> IApplyOnCasterGirl for T {
	fn apply_on_caster_girl(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		is_crit: bool,
	) {
		let mut target = caster.clone();
		self.apply_on_any_girl(ctx, caster, &mut target, girl, is_crit);
	}
}

impl<T: IApplyOnAnyGirl> IApplyOnTargetGirl for T {
	fn apply_on_target_girl(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		is_crit: bool,
	) {
		self.apply_on_any_girl(ctx, caster, target, girl, is_crit);
	}
}
