use super::*;

#[derive(Serialize, Deserialize)]
pub struct ActorStats {
	stamina_max: MaxStamina,
	toughness: Toughness,
	stun_def: StunDef,
	debuff_res: DebuffRes,
	debuff_rate: DebuffRate,
	move_res: MoveRes,
	move_rate: MoveRate,
	poison_res: PoisonRes,
	poison_rate: PoisonRate,
	speed: Speed,
	accuracy: Accuracy,
	crit_rate: CritRate,
	dodge: Dodge,
	power: Power,
	stamina_cur: CurrentStamina,
	damage_range: SaneRange,
	pub last_damager_guid: Option<Uuid>,
	pub stun_redundancy_ms: Option<Int>,
	pub skill_use_counters: HashMap<SkillIdent, u16>,
}

impl ActorBase {
	pub fn increment_skill_counter(&mut self, skill_name: impl Into<SkillIdent>) {
		self.skill_use_counters
			.entry(skill_name.into())
			.and_modify(|c| *c += 1)
			.or_insert(1);
	}

	/// Used to check if an actor died after losing stamina.
	pub fn stamina_alive(&self) -> bool { self.raw_stat::<CurrentStamina>() > 0 }

	pub fn stamina_dead(&self) -> bool { !self.stamina_alive() }

	pub fn skill_counter_bellow_limit(&self, skill_name: SkillIdent, limit: u16) -> bool {
		self.skill_use_counters
			.get(&skill_name)
			.is_none_or(|count| *count < limit)
	}
}
