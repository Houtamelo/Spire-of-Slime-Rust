use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakeSelfGuardTarget {
	pub base_duration_ms: Int,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakeTargetGuardSelf {
	pub base_duration_ms: Int,
}

impl IApplyOnTarget for MakeSelfGuardTarget {
	fn apply_on_target(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) {
		let status = Guarded {
			duration_ms: self.base_duration_ms,
			guarder: caster.id,
		};

		target.add_status(status);
	}
}

impl IApplyOnTarget for MakeTargetGuardSelf {
	fn apply_on_target(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) {
		let status = Guarded {
			duration_ms: self.base_duration_ms,
			guarder: target.id,
		};

		caster.add_status(status);
	}
}
