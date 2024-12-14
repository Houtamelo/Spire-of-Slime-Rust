use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mark {
	pub duration_ms: Int,
}

impl IStatusEffect for Mark {
	fn duration_ms(&self) -> Int { self.duration_ms }
	fn set_duration(&mut self, ms: Int) { self.duration_ms = ms; }
}
