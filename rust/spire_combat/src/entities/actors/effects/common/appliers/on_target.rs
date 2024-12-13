use super::*;
pub trait IApplyOnTarget {
	fn apply_on_target(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	);
}

delegated_enum! {
	ENUM_OUT: {
		#[derive(Clone, Serialize, Deserialize)]
		pub enum TargetApplier {
			Buff(BuffApplier),
			Debuff(DebuffApplier),
			MakeTargetGuardSelf(MakeTargetGuardSelf),
			MakeSelfGuardTarget(MakeSelfGuardTarget),
			Heal(HealApplier),
			Mark(MarkApplier),
			Move(MoveApplier),
			PersistentHeal(PersistentHealApplier),
			Poison(PoisonApplier),
			MakeTargetRiposte(RiposteApplier),
			Stun(StunApplier),
			Perk(PerkApplier),
		}
	}

	DELEGATES: {
		impl trait IApplyOnTarget {
			[fn apply_on_target(
				&self,
				ctx: &mut ActorContext,
				caster: &mut Ptr<Actor>,
				target: &mut Ptr<Actor>,
				is_crit: bool,
			)]
		}
	}
}
