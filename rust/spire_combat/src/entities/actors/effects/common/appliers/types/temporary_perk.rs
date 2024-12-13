use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct PerkApplier {
	pub perk: Perk,
}

impl IApplyOnAny for PerkApplier {
	fn apply_on_any(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) {
		target.add_perk(self.perk.clone())
	}
}
