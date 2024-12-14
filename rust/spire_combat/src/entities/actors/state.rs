use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub enum ActorState {
	Idle,
	Downed {
		ticks: TrackedTicks,
	},
	Stunned {
		ticks: TrackedTicks,
		state_before_stunned: StateBeforeStunned,
	},
	Charging {
		skill_intention: SkillIntention,
	},
	Recovering {
		ticks: TrackedTicks,
	},
	Grappling(GrapplingState),
	Defeated,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GrapplingState {
	pub lust_per_interval: Int,
	pub temptation_per_interval: Int,
	pub duration_ms: Int,
	pub accumulated_ms: Int,
	pub victim_id: Id,
	pub victim_defeated: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum StateBeforeStunned {
	Recovering { ticks: TrackedTicks },
	Charging { skill_intention: SkillIntention },
	Idle,
}

impl ActorState {
	pub fn calc_spd_charge_ms(remaining_ms: Int, character_speed: Speed) -> Int {
		let mut result = remaining_ms;
		result *= 100;
		result /= character_speed;
		result
	}

	pub fn calc_spd_recovery_ms(remaining_ms: Int, character_speed: Speed) -> Int {
		let mut result = remaining_ms;
		result *= 100;
		result /= character_speed;
		result
	}
}
