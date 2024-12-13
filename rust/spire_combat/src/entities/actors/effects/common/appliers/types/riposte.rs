use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiposteApplier {
	pub base_duration_ms: Int,
	pub base_skill_power: Power,
	pub acc_mode: AccuracyMode,
	pub crit_mode: CritMode,
}

impl IApplyOnAny for RiposteApplier {
	fn apply_on_any(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) {
		let skill_power = {
			let mut temp = Int::from(self.base_skill_power);

			if target.has_perk::<EnGarde>() {
				temp += 30;
			}

			temp
		};

		if skill_power <= 0 {
			return;
		}

		let status = Riposte {
			duration_ms: self.base_duration_ms,
			skill_power: skill_power.into(),
			acc_mode: self.acc_mode,
			crit_mode: self.crit_mode,
		};

		target.add_status(status);
	}
}
