use std::collections::HashMap;
use proc_macros::get_perk;
use crate::BoundISize;
use crate::combat::effects::persistent::{PersistentDebuff, PersistentEffect};
use crate::combat::entity::{Corpse, Entity};
use crate::combat::entity::data::character::{CharacterData};
use crate::combat::entity::data::EntityData;
use crate::combat::entity::data::girls::ethel::perks::{Category_Bruiser, Category_Crit, Category_Debuffer, Category_Duelist, Category_Tank, EthelPerk};
use crate::combat::entity::data::skill_name::SkillName;
use crate::combat::entity::girl::*;
use crate::combat::entity::position::Position;
use crate::combat::entity::skill_intention::SkillIntention;
use crate::combat::ModifiableStat;
use crate::combat::perk::Perk;
use crate::util::{GUID, I_Range, TrackedTicks};
use crate::util::bounded_u32::*;

#[derive(Debug, Clone)]
pub struct CombatCharacter {
	pub guid: GUID,
	pub data: CharacterData,
	pub last_damager_guid: Option<GUID>,
	pub stamina_cur: isize,
	pub(super) stamina_max: isize,
	pub(super) toughness: BoundISize<-100, 100>,
	pub(super) stun_def : BoundISize<-100, 300>,
	pub stun_redundancy_ms: Option<i64>,
	pub girl_stats: Option<GirlState>,
	pub(super) debuff_res : BoundISize<-300, 300>,
	pub(super) debuff_rate: BoundISize<-300, 300>,
	pub(super) move_res   : BoundISize<-300, 300>,
	pub(super) move_rate  : BoundISize<-300, 300>,
	pub(super) poison_res : BoundISize<-300, 300>,
	pub(super) poison_rate: BoundISize<-300, 300>,
	pub(super) spd        : BoundU32  <  20, 300>,
	pub(super) acc        : BoundISize<-300, 300>,
	pub(super) crit       : BoundISize<-300, 300>,
	pub(super) dodge      : BoundISize<-300, 300>,
	pub dmg: I_Range,
	pub(super) power: BoundU32<0, 500>,
	pub persistent_effects: Vec<PersistentEffect>,
	pub perks: Vec<Perk>,
	pub state: CharacterState,
	pub position: Position,
	pub on_defeat: OnDefeat,
	pub skill_use_counters: HashMap<SkillName, usize>,
}

impl CombatCharacter {
	pub fn position(&self) -> &Position {
		return &self.position;
	}

	pub fn guid(&self) -> GUID {
		return self.guid;
	}
	
	pub fn get_stat(&self, stat: ModifiableStat) -> isize {
		let mut stat_value = (|character: &CombatCharacter, stat: ModifiableStat| -> isize {
			match stat {
				ModifiableStat::DEBUFF_RES => {
					let mut base = character.debuff_res.get();

					if let Some(Perk::Ethel(EthelPerk::Debuffer(Category_Debuffer::HardNogging))) = get_perk!(character, Perk::Ethel(EthelPerk::Debuffer(Category_Debuffer::HardNogging))) {
						if let CharacterState::Stunned { .. } = character.state {
							base += 25;
						}
					}

					return base;
				},
				ModifiableStat::POISON_RES => character.poison_res.get(),
				ModifiableStat::MOVE_RES => {
					let mut base = character.move_res.get();
					if let Some(Perk::Ethel(EthelPerk::Tank(Category_Tank::Vanguard))) = get_perk!(character, Perk::Ethel(EthelPerk::Tank(Category_Tank::Vanguard))) {
						base += 30;
					}
					return base;
				},
				ModifiableStat::ACC => {
					let mut base = character.acc.get();
					if let Some(Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::Relentless { stacks }))) = get_perk!(character, Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::Relentless { stacks }))) {
						base -= *stacks as isize * 7;
					}

					if let Some(Perk::AffectedByConcentratedToxins(perk)) = get_perk!(character, Perk::AffectedByConcentratedToxins(_)) {
						for persistent_effect in character.persistent_effects.iter() {
							if let PersistentEffect::Poison { caster_guid, .. } = persistent_effect {
								if *caster_guid == perk.caster_guid {
									base += 5;
									break;
								}
							}
						}
					}

					return base;
				},
				ModifiableStat::CRIT => {
					let real_base = character.crit.get();
					let mut base = real_base;
					if let Some(Perk::Ethel(EthelPerk::Crit(Category_Crit::Vicious { stacks }))) = get_perk!(character, Perk::Ethel(EthelPerk::Crit(Category_Crit::Vicious { .. }))) {
						base += *stacks as isize * 10;
					}

					if let Some(Perk::Ethel(EthelPerk::Crit(Category_Crit::Reliable))) = get_perk!(character, Perk::Ethel(EthelPerk::Crit(Category_Crit::Reliable))) {
						base -= real_base;
					}

					return base;
				},
				ModifiableStat::DODGE => {
					let mut base = character.dodge.get();

					if let Some(Perk::Ethel(EthelPerk::Duelist(Category_Duelist::Anticipation))) = get_perk!(character, Perk::Ethel(EthelPerk::Duelist(Category_Duelist::Anticipation))) {
						if character.persistent_effects.iter().any(|effect| return if let PersistentEffect::Riposte { .. } = effect { true } else { false }) { // is any riposte active?
							base += 15;
						}
					}

					return base;
				},
				ModifiableStat::TOUGHNESS => {
					let mut base = character.toughness.get();
					if let Some(Perk::Ethel(EthelPerk::Tank(Category_Tank::ReactiveDefense { stacks }))) = get_perk!(character, Perk::Ethel(EthelPerk::Tank(Category_Tank::ReactiveDefense { .. }))) {
						base += stacks.get() as isize * 4;
					}
					return base;
				},
				ModifiableStat::COMPOSURE => match &character.girl_stats {
					None => 0,
					Some(girl) => { girl.composure.get() }
				},
				ModifiableStat::POWER => {
					let mut base = character.power.get() as isize;
					if let Some(Perk::Ethel(EthelPerk::Tank(Category_Tank::Spikeful))) = get_perk!(character, Perk::Ethel(EthelPerk::Tank(Category_Tank::Spikeful))) {
						base += isize::clamp(character.toughness.get(), 0, 30); // we care about the base toughness, not the modified one.
					}
					if let Some(Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::EnragingPain { stacks }))) = get_perk!(character, Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::EnragingPain { .. }))) {
						base += stacks.get() as isize * 5;
					}
					if let Some(Perk::Ethel(EthelPerk::Crit(Category_Crit::Reliable))) = get_perk!(character, Perk::Ethel(EthelPerk::Crit(Category_Crit::Reliable))) {
						let base_crit = character.crit.get();
						if base_crit > 0 {
							base += base_crit;
						}
					}
					return base;
				},
				ModifiableStat::SPD => {
					let mut base = character.spd.get() as isize;

					if let Some(Perk::AffectedByParalyzingToxins(perk)) = get_perk!(character, Perk::AffectedByParalyzingToxins(_)) {
						for persistent_effect in character.persistent_effects.iter() {
							if let PersistentEffect::Poison { dmg_per_sec, caster_guid, .. } = persistent_effect {
								if *caster_guid == perk.caster_guid {
									base -= (*dmg_per_sec as isize) * 3;
								}
							}
						}
					}

					if let Some(Perk::Ethel(EthelPerk::Duelist(Category_Duelist::EnGarde))) = get_perk!(character, Perk::Ethel(EthelPerk::Duelist(Category_Duelist::EnGarde))) {
						if character.persistent_effects.iter().any(|effect| return if let PersistentEffect::Riposte { .. } = effect { true } else { false }) { // is any riposte active?
							base -= 20;
						}
					}

					return base;
				},
				ModifiableStat::DEBUFF_RATE => character.debuff_rate.get(),
				ModifiableStat::POISON_RATE => character.poison_rate.get(),
				ModifiableStat::MOVE_RATE => character.move_rate.get(),
				ModifiableStat::STUN_DEF => {
					let mut base = character.stun_def.get();
					if let Some(Perk::Ethel(EthelPerk::Tank(Category_Tank::Vanguard))) = get_perk!(character, Perk::Ethel(EthelPerk::Tank(Category_Tank::Vanguard))) {
						base += 30;
					}

					if let Some(Perk::Ethel(EthelPerk::Debuffer(Category_Debuffer::HardNogging))) = get_perk!(character, Perk::Ethel(EthelPerk::Debuffer(Category_Debuffer::HardNogging))) {
						if character.persistent_effects.iter().any(|effect| return if let PersistentEffect::Debuff(_) = effect { true } else { false }) {
							base += 25;
						}
					}

					return base;
				},
			}
		})(self, stat);

		for effect in self.persistent_effects.iter() {
			if let PersistentEffect::Buff { stat: buffed_stat, stat_increase, .. } = effect {
				if stat == *buffed_stat {
					stat_value += *stat_increase as isize;
				}
			}
			else if let PersistentEffect::Debuff(debuff) = effect {
				match debuff {
					PersistentDebuff::Standard { stat: debuff_stat, stat_decrease, .. } => {
						if stat == *debuff_stat {
							stat_value -= *stat_decrease as isize;
						}
					}
					PersistentDebuff::StaggeringForce { .. } => {
						match stat {
							ModifiableStat::TOUGHNESS  | ModifiableStat::COMPOSURE  | ModifiableStat::STUN_DEF
						  | ModifiableStat::DEBUFF_RES | ModifiableStat::POISON_RES | ModifiableStat::MOVE_RES => {
								stat_value -= 10;
							},
							_ => {}
						}
					}
				}
			}
		}

		return stat_value;
	}

	pub fn get_max_stamina(&self) -> isize {
		let mut base = self.stamina_max;
		if let Some(Perk::Ethel(EthelPerk::Tank(Category_Tank::Energetic))) = get_perk!(self, Perk::Ethel(EthelPerk::Tank(Category_Tank::Energetic))) {
			base = (base * 125) / 100;
		}
		return base;
	}

	pub fn clamp_stamina(current: isize, max: isize) -> isize {
		return isize::clamp(current, 0, max);
	}

	pub fn can_be_grappled(&self) -> bool {
		if let CharacterData::Girl(_) = &self.data {
			return self.girl_stats.is_some();
		} else {
			return false;
		}
	}

	/// check "can_be_grappled" before calling this function.
	pub fn into_grappled_unchecked(self) -> GrappledGirlEnum {
		let girl = self.girl_stats.unwrap();
		let CharacterData::Girl(girl_data) = self.data else { panic!("impossible"); };

		return GrappledGirlEnum::Alive(AliveGirl_Grappled {
			guid: self.guid,
			data: girl_data,
			stamina_cur: self.stamina_cur,
			stamina_max: self.stamina_max,
			toughness: self.toughness,
			stun_def: self.stun_def,
			debuff_res: self.debuff_res,
			debuff_rate: self.debuff_rate,
			move_res: self.move_res,
			move_rate: self.move_rate,
			poison_res: self.poison_res,
			poison_rate: self.poison_rate,
			spd: self.spd,
			acc: self.acc,
			crit: self.crit,
			dodge: self.dodge,
			damage: self.dmg,
			power: self.power,
			lust: girl.lust,
			temptation: girl.temptation,
			composure: girl.composure,
			orgasm_limit: girl.orgasm_limit,
			orgasm_count: girl.orgasm_count,
			exhaustion: girl.exhaustion,
			position_before_grappled: self.position,
			on_defeat: self.on_defeat,
			skill_use_counters: self.skill_use_counters,
			perks: self.perks,
		});
	}

	pub fn into_grappled(self) -> Option<GrappledGirlEnum> {
		if self.can_be_grappled() {
			return Some(self.into_grappled_unchecked());
		} else {
			return None;
		}
	}
	
	pub fn entity_on_defeat(self) -> Option<Entity> {
		return match self.on_defeat {
			OnDefeat::Vanish => None,
			OnDefeat::CorpseOrDefeatedGirl => {
				match (self.girl_stats, self.data) {
					(Some(girl), CharacterData::Girl(girl_data)) => Some(Entity::DefeatedGirl(DefeatedGirl_Entity {
						guid: self.guid,
						data: girl_data,
						lust: girl.lust,
						temptation: girl.temptation,
						orgasm_limit: girl.orgasm_limit,
						orgasm_count: girl.orgasm_count,
						exhaustion: girl.exhaustion,
						position: self.position,
					})),
					(_, generic_data) => Some(Entity::Corpse(Corpse {
						guid: self.guid,
						position: self.position,
						data: EntityData::Character(generic_data),
					}))
				}
			}
		}
	}

	pub fn increment_skill_counter(&mut self, skill_name: SkillName) {
		self.skill_use_counters.entry(skill_name).and_modify(|c| *c += 1).or_insert(1);
	}
	
	pub fn skill_counter_bellow_limit(&self, skill_name: SkillName, limit: usize) -> bool {
		return match self.skill_use_counters.get(&skill_name) {
			None => true,
			Some(count) => *count < limit,
		};
	}
	
	/// Used to check if character died after losing stamina.
	pub fn is_alive(&self) -> bool {
		return self.stamina_cur > 0;
	}
	
	pub fn is_dead(&self) -> bool {
		return !self.is_alive();
	}

	pub fn iter_perks(&self) -> impl Iterator<Item=&Perk> {
		return self.perks.iter().chain(self.persistent_effects.iter().filter_map(|effect| {
			match effect {
				PersistentEffect::TemporaryPerk { perk, .. } => Some(perk),
				_ => None,
			}
		}));
	}
}

impl PartialEq<Self> for CombatCharacter {
	fn eq(&self, other: &Self) -> bool { return self.guid == other.guid; }
}

impl Eq for CombatCharacter { }

#[derive(Debug, Clone)]
pub struct State_Grappling {
	pub victim: GrappledGirlEnum,
	pub lust_per_sec: usize,
	pub temptation_per_sec: isize,
	pub duration_ms: i64,
	pub accumulated_ms: i64
}

#[derive(Debug, Clone)]
pub enum CharacterState {
	Idle,
	Grappling(State_Grappling),
	Downed  { ticks: TrackedTicks },
	Stunned { ticks: TrackedTicks, state_before_stunned: StateBeforeStunned },
	Charging { skill_intention: SkillIntention },
	Recovering { ticks: TrackedTicks },
}

impl CharacterState {
	pub fn spd_charge_ms(remaining_ms: i64, character_speed: isize) -> i64 {
		return (remaining_ms * 100) / character_speed as i64;
	}

	pub fn spd_recovery_ms(remaining_ms: i64, character_speed: isize) -> i64 {
		return (remaining_ms * 100) / character_speed as i64;
	}
}

#[derive(Debug, Clone)]
pub enum StateBeforeStunned {
	Recovering { ticks: TrackedTicks },
	Charging { skill_intention: SkillIntention },
	Idle,
}

#[derive(Debug, Copy, Clone)]
pub enum OnDefeat {
	Vanish,
	CorpseOrDefeatedGirl,
}