use std::collections::HashMap;
use std::ops::RangeInclusive;
use crate::iter_mut_allies_of;
use houta_utils::prelude::{BoundISize, BoundUSize};
use rand::prelude::StdRng;
use proc_macros::get_perk;
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::{DebuffApplier, TargetApplier};
use crate::combat::effects::persistent::{PersistentDebuff, PersistentEffect, PoisonAdditive};
use crate::combat::entity::{Corpse, Entity};
use crate::combat::entity::character::CharacterState::Grappling;
use crate::combat::entity::data::character::{CharacterData};
use crate::combat::entity::data::EntityData;
use crate::combat::entity::data::girls::ethel::perks::*;
use crate::combat::entity::data::girls::nema::perks::NemaPerk;
use crate::combat::entity::data::skill_name::SkillName;
use crate::combat::entity::girl::*;
use crate::combat::entity::position::{Direction, Position};
use crate::combat::entity::skill_intention::SkillIntention;
use crate::combat::ModifiableStat;
use crate::combat::perk::Perk;
use crate::util::{GUID, TrackedTicks};

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
	pub(super) spd        : BoundUSize < 20, 300>,
	pub(super) acc        : BoundISize<-300, 300>,
	pub(super) crit       : BoundISize<-300, 300>,
	pub(super) dodge      : BoundISize<-300, 300>,
	pub dmg: RangeInclusive<usize>,
	pub(super) power: BoundUSize<0, 500>,
	pub persistent_effects: Vec<PersistentEffect>,
	pub perks: Vec<Perk>,
	pub state: CharacterState,
	pub position: Position,
	pub on_zero_stamina: OnZeroStamina,
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

					if let Some(Perk::Ethel(EthelPerk::Debuffer_HardNogging)) = get_perk!(character, Perk::Ethel(EthelPerk::Debuffer_HardNogging)) {
						if let CharacterState::Stunned { .. } = character.state {
							base += 25;
						}
					}

					return base;
				},
				ModifiableStat::POISON_RES => {
					let base = character.poison_res.get();
					return base;
				},
				ModifiableStat::MOVE_RES => {
					let base = character.move_res.get();
					return base;
				},
				ModifiableStat::ACC => {
					let mut base = character.acc.get();

					// Perks
					{
						if let Some(Perk::Ethel(EthelPerk::Bruiser_Relentless { stacks })) = get_perk!(character, Perk::Ethel(EthelPerk::Bruiser_Relentless { stacks })) {
							base -= *stacks as isize * 5;
						}

						if let Some(Perk::Nema(NemaPerk::BattleMage_Carefree)) = get_perk!(character, Perk::Nema(NemaPerk::BattleMage_Carefree)) {
							if character.persistent_effects.iter().all(|effect| matches!(effect, PersistentEffect::Poison {..}) == false) {
								base += 10;
							}
						}

						if let Some(Perk::Nema(NemaPerk::BattleMage_Trust { accumulated_ms })) = get_perk!(character, Perk::Nema(NemaPerk::BattleMage_Trust{..})) {
							let stacks = i64::clamp(accumulated_ms / 1000, 0, 7) as isize;
							base += stacks * 2;
						}
					}

					if character.persistent_effects.iter().any(|effect| {
						if let PersistentEffect::Poison { additives, .. } = effect {
							return additives.iter().any(|additive| matches!(additive, PoisonAdditive::Ethel_ConcentratedToxins));
						} else {
							return false;
						} }) {
						base += 5;
					}

					return base;
				},
				ModifiableStat::CRIT => {
					let real_base = character.crit.get();
					let mut base = real_base;

					// Perks
					{
						if let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk!(character, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
							base += *stacks as isize * 10;
						}

						if let Some(Perk::Ethel(EthelPerk::Crit_Reliable)) = get_perk!(character, Perk::Ethel(EthelPerk::Crit_Reliable)) {
							base -= real_base;
						}

						if let Some(Perk::Nema(NemaPerk::BattleMage_Trust { accumulated_ms })) = get_perk!(character, Perk::Nema(NemaPerk::BattleMage_Trust{..})) {
							let stacks = i64::clamp(accumulated_ms / 1000, 0, 7) as isize;
							base += stacks * 2;
						}
					}

					return base;
				},
				ModifiableStat::DODGE => {
					let mut base = character.dodge.get();

					// Perks
					{
						if let Some(Perk::Ethel(EthelPerk::Duelist_Anticipation)) = get_perk!(character, Perk::Ethel(EthelPerk::Duelist_Anticipation)) {
							if character.persistent_effects.iter().any(|effect| return if let PersistentEffect::Riposte { .. } = effect { true } else { false }) { // is any riposte active?
								base += 15;
							}
						}

						if let Some(Perk::Nema(NemaPerk::Healer_Alarmed { duration_remaining_ms })) = get_perk!(character, Perk::Nema(NemaPerk::Healer_Alarmed { .. })) {
							if *duration_remaining_ms > 0 {
								base += 50;
							}
						}

						if let Some(Perk::Nema(NemaPerk::BattleMage_Carefree)) = get_perk!(character, Perk::Nema(NemaPerk::BattleMage_Carefree)) {
							if character.persistent_effects.iter().all(|effect| matches!(effect, PersistentEffect::Debuff(_)) == false) {
								base += 10;
							}
						}
					}

					return base;
				},
				ModifiableStat::TOUGHNESS => {
					let mut base = character.toughness.get();

					// Perks
					{
						if let Some(Perk::Ethel(EthelPerk::Tank_ReactiveDefense { stacks })) = get_perk!(character, Perk::Ethel(EthelPerk::Tank_ReactiveDefense {..})) {
							base += stacks.get() as isize * 4;
						}

						if let Some(Perk::Nema(NemaPerk::AOE_Hatred {..})) = get_perk!(character, Perk::Nema(NemaPerk::AOE_Hatred {..})) {
							base += 10;
						}
					}

					return base;
				},
				ModifiableStat::COMPOSURE => {
					if character.girl_stats.is_none() {
						return 0;
					}

					let mut base = character.girl_stats.as_ref().unwrap().composure.get();

					//Perks
					{
						if let Some(Perk::Nema(NemaPerk::BattleMage_Agitation)) = get_perk!(character, Perk::Nema(NemaPerk::BattleMage_Agitation)) {
							let spd_bellow_100 = isize::min(character.get_stat(ModifiableStat::SPD) - 100, 0);
							base += spd_bellow_100;
						}

						if let Some(Perk::Nema(NemaPerk::Grumpiness)) = get_perk!(character, Perk::Nema(NemaPerk::Grumpiness)) {
							if matches!(character.state, CharacterState::Downed {..}) {
								base += 30;
							}
						}
					}

					return base;
				},
				ModifiableStat::POWER => {
					let mut base = character.power.get() as isize;

					// Perks
					{
						if let Some(Perk::Ethel(EthelPerk::Tank_Spikeful)) = get_perk!(character, Perk::Ethel(EthelPerk::Tank_Spikeful)) {
							base += isize::clamp(character.toughness.get(), 0, 30); // we care about the base toughness, not the modified one.
						}

						if let Some(Perk::Ethel(EthelPerk::Bruiser_EnragingPain { stacks })) = get_perk!(character, Perk::Ethel(EthelPerk::Bruiser_EnragingPain { .. })) {
							base += stacks.get() as isize * 5;
						}

						if let Some(Perk::Ethel(EthelPerk::Crit_Reliable)) = get_perk!(character, Perk::Ethel(EthelPerk::Crit_Reliable)) {
							let base_crit = character.crit.get();
							if base_crit > 0 {
								base += base_crit;
							}
						}

						if let Some(Perk::Nema(NemaPerk::BattleMage_Agitation)) = get_perk!(character, Perk::Nema(NemaPerk::BattleMage_Agitation)) {
							let spd_above_100 = isize::max(character.get_stat(ModifiableStat::SPD) - 100, 0);
							base += spd_above_100;
						}
					}

					if character.persistent_effects.iter().any(|effect| {
						let PersistentEffect::Poison { additives, .. } = effect else { return false; };
						return additives.iter().any(|additive| matches!(additive, PoisonAdditive::Nema_Madness));
					}) {
						base += 25;
					}

					return base;
				},
				ModifiableStat::SPD => {
					let mut base = character.spd.get() as isize;

					let total_paralyzing_poison = character.persistent_effects.iter().fold(0, |mut total, effect| {
						if let PersistentEffect::Poison { dmg_per_interval: dmg_per_sec, additives, .. } = effect {
							if additives.iter().any(|additive| matches!(additive, PoisonAdditive::Ethel_ParalyzingToxins)) {
								total += dmg_per_sec;
							}
						}

						return total;
					}) as isize;

					base -= isize::clamp(total_paralyzing_poison * 3, 0, 30);

					if let Some(Perk::Ethel(EthelPerk::Duelist_EnGarde)) = get_perk!(character, Perk::Ethel(EthelPerk::Duelist_EnGarde)) {
						if character.persistent_effects.iter().any(|effect| return if let PersistentEffect::Riposte { .. } = effect { true } else { false }) { // is any riposte active?
							base -= 20;
						}
					}

					if let Some(Perk::Nema(NemaPerk::BattleMage_Trust { accumulated_ms })) = get_perk!(character, Perk::Nema(NemaPerk::BattleMage_Trust{..})) {
						let stacks = i64::clamp(accumulated_ms / 1000, 0, 7) as isize;
						base += stacks * 3;
					}

					return base;
				},
				ModifiableStat::DEBUFF_RATE => character.debuff_rate.get(),
				ModifiableStat::POISON_RATE => {
					let mut base = character.poison_rate.get();

					if let Some(Perk::Nema(NemaPerk::Poison_Melancholy)) = get_perk!(character, Perk::Nema(NemaPerk::Poison_Melancholy)) {
						base += isize::max(character.get_max_stamina() - character.stamina_cur, 0);
					}

					return base;
				},
				ModifiableStat::MOVE_RATE => character.move_rate.get(),
				ModifiableStat::STUN_DEF => {
					let mut base = character.stun_def.get();

					if let Some(Perk::Ethel(EthelPerk::Debuffer_HardNogging)) = get_perk!(character, Perk::Ethel(EthelPerk::Debuffer_HardNogging)) {
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
		if let Some(Perk::Ethel(EthelPerk::Tank_Energetic)) = get_perk!(self, Perk::Ethel(EthelPerk::Tank_Energetic)) {
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
			on_defeat: self.on_zero_stamina,
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

	pub fn do_on_zero_stamina(mut self, killer: Option<&mut CombatCharacter>, others: &mut HashMap<GUID, Entity>, seed: &mut StdRng) {
		// Perk::Ethel_LingeringToxins
		{
			let mut self_adjacent_center: Option<&mut Entity> = None;
			let mut self_adjacent_edge  : Option<&mut Entity> = None;

			let self_pos = &self.position;
			for self_ally in iter_mut_allies_of!(self, others) {
				let ally_pos = self_ally.position();
				match Position::is_adjacent(self_pos, ally_pos) {
					Some(Direction::Center) => {
						debug_assert!({ if let None = self_adjacent_center { true } else { false } });
						self_adjacent_center = Some(self_ally);
					},
					Some(Direction::Edge) => {
						debug_assert!({ if let None = self_adjacent_edge { true } else { false } });
						self_adjacent_edge = Some(self_ally);
					},
					None => {}
				}
			}

			for effect in self.persistent_effects.iter() {
				let PersistentEffect::Poison { duration_ms, accumulated_ms, interval_ms, dmg_per_interval, additives, caster_guid } = effect
						else { continue; };

				if additives.iter().any(|add| matches!(add, PoisonAdditive::Ethel_LingeringToxins)) == false {
					continue;
				}

				let poison = PersistentEffect::Poison {
					duration_ms     : *duration_ms / 2   ,
					accumulated_ms  : accumulated_ms / 2 ,
					interval_ms     : interval_ms.clone(),
					dmg_per_interval: *dmg_per_interval,
					additives       : additives  .clone(),
					caster_guid     : caster_guid.clone(),
				};

				if let Some(Entity::Character(ally)) = self_adjacent_center {
					ally.persistent_effects.push(poison.clone());
				}
				if let Some(Entity::Character(ally)) = self_adjacent_edge {
					ally.persistent_effects.push(poison);
				}
			}
		}

		// OnKill effects
		if let Some(killer) = killer {
			if let Some(Perk::Nema(NemaPerk::BattleMage_Triumph)) = get_perk!(killer, Perk::Nema(NemaPerk::BattleMage_Triumph)) {
				let speed_buff = SelfApplier::Buff {
					duration_ms: 3000,
					stat: ModifiableStat::SPD,
					stat_increase: 25,
				};

				speed_buff.apply(killer, others, seed, false);

				if let Some(girl) = &mut killer.girl_stats {
					girl.lust -= 10;
				}
			}

			if let Some(Perk::Nema(NemaPerk::Regret)) = get_perk!(killer, Perk::Nema(NemaPerk::BattleMage_Triumph)) {
				let composure_debuff = TargetApplier::Debuff(DebuffApplier::Standard {
					duration_ms: 5000,
					stat: ModifiableStat::COMPOSURE,
					stat_decrease: 15,
					apply_chance: None,
				});

				composure_debuff.apply_self(killer, others, seed, false);
			}
		}
		
		if let Grappling(grappling_state) = self.state {
			match grappling_state.victim {
				GrappledGirlEnum::Alive(girl_alive) => {
					let mut girl_standing = girl_alive.to_non_grappled();
					girl_standing.state = CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(2500) }; // girl is downed for 2.5s after being released from a grapple

					*girl_standing.position.order_mut() = 0;

					for girl_ally in iter_mut_allies_of!(girl_standing, others) {
						let ally_order: &mut usize = girl_ally.position_mut().order_mut();
						*ally_order += girl_standing.position.size();
					}

					others.insert(girl_standing.guid, Entity::Character(girl_standing));
				}
				GrappledGirlEnum::Defeated(girl_defeated) => {
					let mut girl_standing = girl_defeated.to_non_grappled();

					*girl_standing.position.order_mut() = 0;

					for girl_ally in iter_mut_allies_of!(girl_standing, others) {
						let mutref_ally_order = girl_ally.position_mut().order_mut();
						*mutref_ally_order += girl_standing.position.size();
					}

					others.insert(girl_standing.guid, Entity::DefeatedGirl(girl_standing));
				}
			}
		}

		match self.on_zero_stamina {
			OnZeroStamina::Corpse => {
				let corpse = Entity::Corpse(Corpse {
					guid: self.guid,
					position: self.position,
					data: EntityData::Character(self.data),
				});

				others.insert(corpse.guid(), corpse);
			},
			OnZeroStamina::Downed => {
				self.state = CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(8000) };
				others.insert(self.guid(), Entity::Character(self));
			},
			OnZeroStamina::Vanish => {},
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
	pub fn stamina_alive(&self) -> bool {
		return self.stamina_cur > 0;
	}
	
	pub fn stamina_dead(&self) -> bool {
		return !self.stamina_alive();
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
pub enum OnZeroStamina {
	Vanish,
	Corpse,
	Downed,
}