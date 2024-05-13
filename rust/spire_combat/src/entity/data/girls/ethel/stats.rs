#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthelData {
	pub(super) skills: Cow<'static, [Skill]>,
	pub(super) composure: Composure,
	pub(super) orgasm_limit: OrgasmLimit,
	pub(super) size: Size,
    pub(super) dmg: CheckedRange,
    pub(super) spd: Speed,
    pub(super) acc: Accuracy,
    pub(super) crit : CritRate,
    pub(super) dodge: Dodge,
    pub(super) max_stamina: MaxStamina,
    pub(super) toughness  : Toughness,
    pub(super) stun_def   : StunDef,
    pub(super) debuff_res : DebuffRes,
    pub(super) debuff_rate: DebuffRate,
    pub(super) move_res   : MoveRes,
    pub(super) move_rate  : MoveRate,
    pub(super) poison_res : PoisonRes,
    pub(super) poison_rate: PoisonRate,
}

impl GirlData for EthelData {
	fn composure(&self) -> Composure { return self.composure; }
	fn orgasm_limit(&self) -> OrgasmLimit { return self.orgasm_limit; }
}

impl CharacterData for EthelData {
	fn variant(&self) -> CharacterVariant { CharacterVariant::Girl(GirlVariant::Ethel) }

	fn max_stamina(&self, _level: u8) -> MaxStamina { self.max_stamina }

	fn dmg  (&self, _level: u8) -> CheckedRange { self.dmg }
	fn spd  (&self, _level: u8) -> Speed    { self.spd   }
	fn acc  (&self, _level: u8) -> Accuracy { self.acc   }
	fn crit (&self, _level: u8) -> CritRate { self.crit  }
	fn dodge(&self, _level: u8) -> Dodge    { self.dodge }
	
	fn toughness  (&self, _level: u8) -> Toughness  { self.toughness   }
	fn stun_def   (&self, _level: u8) -> StunDef    { self.stun_def    }
	
	fn debuff_res (&self, _level: u8) -> DebuffRes  { self.debuff_res  }
	fn debuff_rate(&self, _level: u8) -> DebuffRate { self.debuff_rate }
	
	fn move_res   (&self, _level: u8) -> MoveRes    { self.move_res    }
	fn move_rate  (&self, _level: u8) -> MoveRate   { self.move_rate   }
	
	fn poison_res (&self, _level: u8) -> PoisonRes  { self.poison_res  }
	fn poison_rate(&self, _level: u8) -> PoisonRate { self.poison_rate }
	
	fn skills<'a>(&'a self) -> &Cow<'a, [Skill]> { &self.skills }
}