use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guarded {
	pub duration_ms: Int,
	pub guarder: Id,
}

impl IStatusEffect for Guarded {
	fn duration_ms(&self) -> Int { self.duration_ms }
	fn set_duration(&mut self, ms: Int) { self.duration_ms = ms; }
}
