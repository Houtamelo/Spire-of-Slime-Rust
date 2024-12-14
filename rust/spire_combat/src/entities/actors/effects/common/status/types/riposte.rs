use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Riposte {
	pub duration_ms: Int,
	pub skill_power: Power,
	pub acc_mode: AccuracyMode,
	pub crit_mode: CritMode,
}

impl IStatusEffect for Riposte {
	fn duration_ms(&self) -> Int { self.duration_ms }
	fn set_duration(&mut self, ms: Int) { self.duration_ms = ms; }
}
