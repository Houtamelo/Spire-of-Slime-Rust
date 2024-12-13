use super::*;
use crate::internal_prelude::*;
pub trait IApplyOnCasterGirl {
	fn apply_on_caster_girl(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		is_crit: bool,
	);
}

delegated_enum! {
	ENUM_OUT: {
		#[derive(Clone, Serialize, Deserialize)]
		pub enum CasterGirlApplier {
			Arousal(ArousalApplier),
			Buff(GirlBuffApplier),
			ChangeExhaustion(ChangeExhaustionApplier),
			Lust(LustApplier),
		}
	}

	DELEGATES: {
		impl trait IApplyOnCasterGirl {
			[fn apply_on_caster_girl(
				&self,
				ctx: &mut ActorContext,
				caster: &mut Ptr<Actor>,
				girl: &mut Ptr<Girl>,
				is_crit: bool,
			)]
		}
	}
}
