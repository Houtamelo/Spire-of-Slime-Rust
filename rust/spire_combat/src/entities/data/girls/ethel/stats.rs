use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct EthelData {
	pub(super) skills: Vec<Skill>,
	pub(super) composure: Composure,
	pub(super) orgasm_limit: OrgasmLimit,
	pub(super) size: Size,
	pub(super) dmg: SaneRange,
	pub(super) spd: Speed,
	pub(super) acc: Accuracy,
	pub(super) crit: CritRate,
	pub(super) dodge: Dodge,
	pub(super) max_stamina: MaxStamina,
	pub(super) toughness: Toughness,
	pub(super) stun_def: StunDef,
	pub(super) debuff_res: DebuffRes,
	pub(super) debuff_rate: DebuffRate,
	pub(super) move_res: MoveRes,
	pub(super) move_rate: MoveRate,
	pub(super) poison_res: PoisonRes,
	pub(super) poison_rate: PoisonRate,
}

impl GirlData for EthelData {
	fn composure(&self) -> Composure { return self.composure; }
	fn orgasm_limit(&self) -> OrgasmLimit { return self.orgasm_limit; }
}

impl CharacterData for EthelData {
	fn variant(&self) -> CharacterVariant { CharacterVariant::Girl(GirlName::Ethel) }

	fn max_stamina(&self, _level: i64) -> MaxStamina { self.max_stamina }

	fn dmg(&self, _level: i64) -> SaneRange { self.dmg }
	fn spd(&self, _level: i64) -> Speed { self.spd }
	fn acc(&self, _level: i64) -> Accuracy { self.acc }
	fn crit(&self, _level: i64) -> CritRate { self.crit }
	fn dodge(&self, _level: i64) -> Dodge { self.dodge }

	fn toughness(&self, _level: i64) -> Toughness { self.toughness }
	fn stun_def(&self, _level: i64) -> StunDef { self.stun_def }

	fn debuff_res(&self, _level: i64) -> DebuffRes { self.debuff_res }
	fn debuff_rate(&self, _level: i64) -> DebuffRate { self.debuff_rate }

	fn move_res(&self, _level: i64) -> MoveRes { self.move_res }
	fn move_rate(&self, _level: i64) -> MoveRate { self.move_rate }

	fn poison_res(&self, _level: i64) -> PoisonRes { self.poison_res }
	fn poison_rate(&self, _level: i64) -> PoisonRate { self.poison_rate }

	fn skills(&self) -> &[Skill] { &self.skills }
}
