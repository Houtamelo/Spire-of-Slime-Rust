use super::*;

pub trait IApplyOnCaster {
	fn apply_on_caster(&self, ctx: &mut ActorContext, caster: &mut Ptr<Actor>, is_crit: bool);
}

delegated_enum! {
	ENUM_OUT: {
		#[derive(Clone, Serialize, Deserialize)]
		pub enum CasterApplier {
			Buff(BuffApplier),
			Debuff(DebuffApplier),
			Heal(HealApplier),
			Mark(MarkApplier),
			Move(MoveApplier),
			PersistentHeal(PersistentHealApplier),
			Poison(PoisonApplier),
			Riposte(RiposteApplier),
			Stun(StunApplier),
			Summon(SummonApplier),
			Perk(PerkApplier),
		}
	}

	DELEGATES: {
		impl trait IApplyOnCaster {
			[fn apply_on_caster(
				&self,
				ctx: &mut ActorContext,
				caster: &mut Ptr<Actor>,
				is_crit: bool,
			)]
		}
	}
}
