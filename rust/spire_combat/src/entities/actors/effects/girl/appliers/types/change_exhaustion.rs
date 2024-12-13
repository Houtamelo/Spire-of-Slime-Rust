use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeExhaustionApplier {
	pub base_delta: Int,
}

impl IApplyOnAnyGirl for ChangeExhaustionApplier {
	fn apply_on_any_girl(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		is_crit: bool,
	) {
		*girl.raw_stat_mut::<Exhaustion>() += self.base_delta;
	}
}
