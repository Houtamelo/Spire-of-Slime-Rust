use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Debuff {
	pub duration_ms: Int,
	pub kind: DebuffKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DebuffKind {
	Standard { stat: StatEnum, stat_decrease: Int },
	StaggeringForce,
}

impl IStatusEffect for Debuff {
	fn duration_ms(&self) -> Int { self.duration_ms }
	fn set_duration(&mut self, ms: Int) { self.duration_ms = ms; }
}
