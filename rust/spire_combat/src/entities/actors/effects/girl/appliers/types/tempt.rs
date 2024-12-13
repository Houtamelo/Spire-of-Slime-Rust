use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemptApplier {
	pub base_intensity: Int,
}

impl IApplyOnTargetGirl for TemptApplier {
	fn apply_on_target_girl(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		is_crit: bool,
	) {
		todo!()
	}
}
