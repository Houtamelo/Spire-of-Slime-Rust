use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct GirlBuff {
	pub duration_ms: Int,
	pub stat: GirlStatEnum,
	pub stat_increase: Int,
}

impl IGirlStatusEffect for GirlBuff {
	fn duration_ms(&self) -> Int { self.duration_ms }
	fn set_duration(&mut self, ms: Int) { self.duration_ms = ms; }
}
