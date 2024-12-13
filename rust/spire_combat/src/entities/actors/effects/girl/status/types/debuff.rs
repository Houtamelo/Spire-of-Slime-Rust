use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct GirlDebuff {
	pub duration_ms: Int,
	pub stat: GirlStatEnum,
	pub stat_decrease: Int,
}

impl IGirlStatusEffect for GirlDebuff {
	fn duration_ms(&self) -> Int { self.duration_ms }
	fn set_duration(&mut self, ms: Int) { self.duration_ms = ms; }
}
