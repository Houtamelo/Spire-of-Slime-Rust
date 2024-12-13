use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummonApplier {
	pub character_key: String,
}

impl IApplyOnCaster for SummonApplier {
	fn apply_on_caster(&self, ctx: &mut ActorContext, caster: &mut Ptr<Actor>, is_crit: bool) {
		todo!()
	}
}
