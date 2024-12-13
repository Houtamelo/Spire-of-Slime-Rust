use super::*;

pub trait IApplyOnTargetGirl {
	fn apply_on_target_girl(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		is_crit: bool,
	);
}

delegated_enum! {
	ENUM_OUT: {
		#[derive(Clone, Serialize, Deserialize)]
		pub enum TargetGirlApplier {
			Arousal(ArousalApplier),
			Buff(GirlBuffApplier),
			ChangeExhaustion(ChangeExhaustionApplier),
			Lust(LustApplier),
			Tempt(TemptApplier),
		}
	}

	DELEGATES: {
		impl trait IApplyOnTargetGirl {
			[fn apply_on_target_girl(
				&self,
				ctx: &mut ActorContext,
				caster: &mut Ptr<Actor>,
				target: &mut Ptr<Actor>,
				girl: &mut Ptr<Girl>,
				is_crit: bool,
			)]
		}
	}
}
