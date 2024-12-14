use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buff {
	pub duration_ms: Int,
	pub stat: StatEnum,
	pub stat_increase: Int,
}

impl IStatusEffect for Buff {
	fn duration_ms(&self) -> Int { self.duration_ms }
	fn set_duration(&mut self, ms: Int) { self.duration_ms = ms; }
}
