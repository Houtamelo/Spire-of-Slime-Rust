#[allow(unused_imports)]
use crate::*;

use std::num::{NonZeroI8, NonZeroU16, NonZeroU8};

use rand_xoshiro::Xoshiro256PlusPlus;

use crate::combat::shared::*;

use crate::combat::effects::{
	onSelf::SelfApplier,
	onTarget::{DebuffApplierKind, TargetApplier},
	persistent::{PersistentDebuff, PersistentEffect, PoisonAdditive},
};

use crate::combat::entity::data::girls::{
	ethel::perks::*,
	nema::perks::*,
};

use crate::combat::entity::skill_intention::SkillIntention;
use crate::misc::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatCharacter {
	pub guid: Uuid,
	pub data: CharacterData,
	pub last_damager_guid: Option<Uuid>,
	pub stamina_cur: CurrentStamina,
	pub(super) stamina_max: MaxStamina,
	pub(super) toughness: Toughness,
	pub(super) stun_def : StunDef,
	pub stun_redundancy_ms: Option<SaturatedU64>,
	pub girl_stats: Option<GirlState>,
	pub(super) debuff_res : DebuffRes,
	pub(super) debuff_rate: DebuffRate,
	pub(super) move_res   : MoveRes,
	pub(super) move_rate  : MoveRate,
	pub(super) poison_res : PoisonRes,
	pub(super) poison_rate: PoisonRate,
	pub(super) spd  : Speed,
	pub(super) acc  : Accuracy,
	pub(super) crit : CritRate,
	pub(super) dodge: Dodge,
	pub dmg: CheckedRange,
	pub(super) power: Power,
	pub persistent_effects: Vec<PersistentEffect>,
	pub perks: Vec<Perk>,
	pub state: CharacterState,
	pub position: Position,
	pub on_zero_stamina: OnZeroStamina,
	pub skill_use_counters: HashMap<SkillName, u16>,
}

impl CombatCharacter {
	pub fn position(&self) -> &Position {
		return &self.position;
	}

	pub fn guid(&self) -> Uuid {
		return self.guid;
	}
	
	pub fn dyn_stat<T: DynamicStatTrait>(&self) -> T {
		let stat_kind = T::stat_enum();
		
		let unbuffed_stat = 'stat_calc: {
			match stat_kind {
				DynamicStat::DebuffRes => {
					let mut temp = self.debuff_res.to_sat_i64();
					
					if let Some(Perk::Ethel(EthelPerk::Debuffer_HardNogging)) = get_perk!(self, Perk::Ethel(EthelPerk::Debuffer_HardNogging)) 
						&& matches!(self.state, CharacterState::Stunned{..}) {
						temp += 25;
					}

					temp
				},
				DynamicStat::PoisonRes => {
					self.poison_res.to_sat_i64()
				},
				DynamicStat::MoveRes => {
					self.move_res.to_sat_i64()
				},
				DynamicStat::Accuracy => {
					let mut temp = self.acc.to_sat_i64();

					// Perks
					{
						if let Some(Perk::Ethel(EthelPerk::Bruiser_Relentless { stacks })) = get_perk!(self, Perk::Ethel(EthelPerk::Bruiser_Relentless {..})) {
							temp -= stacks.squeeze_to_i64() * 5;
						}

						if let Some(Perk::Nema(NemaPerk::BattleMage_Carefree)) = get_perk!(self, Perk::Nema(NemaPerk::BattleMage_Carefree)) 
							&& no_matches!(self.persistent_effects, PersistentEffect::Poison {..}) { 
							temp += 10;
						}

						if let Some(Perk::Nema(NemaPerk::BattleMage_Trust { accumulated_ms })) = get_perk!(self, Perk::Nema(NemaPerk::BattleMage_Trust{..})) {
							let stacks = i64::clamp(accumulated_ms.squeeze_to_i64() / 1000, 0, 7);
							temp += stacks * 2;
						}
					}

					if self.persistent_effects.iter().any(|effect| {
						if let PersistentEffect::Poison { additives, .. } = effect
							&& any_matches!(additives, PoisonAdditive::Ethel_ConcentratedToxins) { true } else { false }
					}) {
						temp += 5;
					}

					temp
				},
				DynamicStat::Crit => {
					let base = self.crit.squeeze_to_i64();
					let mut temp = base.to_sat_i64();

					// Perks
					{
						if let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk!(self, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
							temp += stacks.squeeze_to_i64() * 10;
						}

						if let Some(Perk::Ethel(EthelPerk::Crit_Reliable)) = get_perk!(self, Perk::Ethel(EthelPerk::Crit_Reliable)) {
							temp -= base;
						}

						if let Some(Perk::Nema(NemaPerk::BattleMage_Trust { accumulated_ms })) = get_perk!(self, Perk::Nema(NemaPerk::BattleMage_Trust{..})) {
							let stacks = i64::clamp(accumulated_ms.squeeze_to_i64() / 1000, 0, 7);
							temp += stacks * 2;
						}
					}

					temp
				},
				DynamicStat::Dodge => {
					let mut temp = self.dodge.to_sat_i64();

					// Perks
					{
						if let Some(Perk::Ethel(EthelPerk::Duelist_Anticipation)) = get_perk!(self, Perk::Ethel(EthelPerk::Duelist_Anticipation)) 
							&& any_matches!(self.persistent_effects, PersistentEffect::Riposte { .. }) {
							temp += 15;
						}

						if let Some(Perk::Nema(NemaPerk::Healer_Alarmed { duration_remaining_ms })) = get_perk!(self, Perk::Nema(NemaPerk::Healer_Alarmed { .. }))
							&& duration_remaining_ms.get() > 0 {
							temp += 50;
						}

						if let Some(Perk::Nema(NemaPerk::BattleMage_Carefree)) = get_perk!(self, Perk::Nema(NemaPerk::BattleMage_Carefree)) 
							&& no_matches!(self.persistent_effects, PersistentEffect::Debuff{..}) {
							temp += 10;
						}
					}

					temp
				},
				DynamicStat::Toughness => {
					let mut temp = self.toughness.to_sat_i64();

					// Perks
					{
						if let Some(Perk::Ethel(EthelPerk::Tank_ReactiveDefense { stacks })) = get_perk!(self, Perk::Ethel(EthelPerk::Tank_ReactiveDefense {..})) {
							temp += stacks.squeeze_to_i64() * 4;
						}

						if let Some(Perk::Nema(NemaPerk::AOE_Hatred {..})) = get_perk!(self, Perk::Nema(NemaPerk::AOE_Hatred {..})) {
							temp += 10;
						}
					}

					temp
				},
				DynamicStat::Power => {
					let mut temp = self.power.to_sat_i64();

					// Perks
					{
						if let Some(Perk::Ethel(EthelPerk::Tank_Spikeful)) = get_perk!(self, Perk::Ethel(EthelPerk::Tank_Spikeful)) {
							temp += i64::clamp(self.toughness.squeeze_to(), 0, 30); // we care about the base toughness, not the modified one.
						}

						if let Some(Perk::Ethel(EthelPerk::Bruiser_EnragingPain { stacks })) = get_perk!(self, Perk::Ethel(EthelPerk::Bruiser_EnragingPain { .. })) {
							temp += stacks.squeeze_to_i64() * 5;
						}

						if let Some(Perk::Ethel(EthelPerk::Crit_Reliable)) = get_perk!(self, Perk::Ethel(EthelPerk::Crit_Reliable)) 
							&& let base_crit @ 1.. = self.crit.squeeze_to_i64() {
							temp += base_crit;
						}

						if let Some(Perk::Nema(NemaPerk::BattleMage_Agitation)) = get_perk!(self, Perk::Nema(NemaPerk::BattleMage_Agitation))
							&& let spd_above_100 @ 1.. = (self.dyn_stat::<Speed>().squeeze_to_i64() - 100) {
							temp += spd_above_100;
						}
					}

					if self.persistent_effects.iter().any(|effect| {
						if let PersistentEffect::Poison { additives, .. } = effect
							&& any_matches!(additives, PoisonAdditive::Nema_Madness) { true } else { false }
					}) {
						temp += 25;
					}

					temp
				},
				DynamicStat::Speed => {
					let mut temp = self.spd.to_sat_i64();

					let total_paralyzing_poison = self.persistent_effects.iter()
						.fold(0.to_sat_i64(), |mut sum, effect| {
							if let PersistentEffect::Poison { poison_per_interval: dmg_per_interval, additives, .. } = effect 
								&& any_matches!(additives, PoisonAdditive::Ethel_ParalyzingToxins) { 
								sum += dmg_per_interval.get();
							}

							return sum;
						}).get();

					temp -= i64::clamp(total_paralyzing_poison * 3, 0, 30);

					if let Some(Perk::Ethel(EthelPerk::Duelist_EnGarde)) = get_perk!(self, Perk::Ethel(EthelPerk::Duelist_EnGarde)) 
						&& any_matches!(self.persistent_effects, PersistentEffect::Riposte { .. }) {
						temp -= 20;
					}

					if let Some(Perk::Nema(NemaPerk::BattleMage_Trust { accumulated_ms })) = get_perk!(self, Perk::Nema(NemaPerk::BattleMage_Trust{..})) {
						let stacks = i64::clamp(accumulated_ms.squeeze_to_i64() / 1000, 0, 7).squeeze_to_i64();
						temp += stacks * 3;
					}

					temp
				},
				DynamicStat::DebuffRate => { 
					self.debuff_rate.to_sat_i64()
				},
				DynamicStat::PoisonRate => {
					let mut temp = self.poison_rate.to_sat_i64();

					if let Some(Perk::Nema(NemaPerk::Poison_Melancholy)) = get_perk!(self, Perk::Nema(NemaPerk::Poison_Melancholy)) {
						temp += i64::max(self.max_stamina().squeeze_to_i64() - self.stamina_cur.squeeze_to_i64(), 0);
					}

					temp
				},
				DynamicStat::MoveRate => { 
					self.move_rate.to_sat_i64()
				},
				DynamicStat::StunDef => {
					let mut temp = self.stun_def.to_sat_i64();

					if let Some(Perk::Ethel(EthelPerk::Debuffer_HardNogging)) = get_perk!(self, Perk::Ethel(EthelPerk::Debuffer_HardNogging)) 
						&& any_matches!(self.persistent_effects, PersistentEffect::Debuff{..}) { 
						temp += 25;
					}

					temp
				},
				DynamicStat::Composure => {
					let Some(girl_stats) = &self.girl_stats
						else { break 'stat_calc 0.to_sat_i64(); };
					
					let mut temp = girl_stats.composure.to_sat_i64();

					//Perks
					{
						if let Some(Perk::Nema(NemaPerk::BattleMage_Agitation)) = get_perk!(self, Perk::Nema(NemaPerk::BattleMage_Agitation))
							&& let spd_bellow_100 @ ..=-1 = (100 - self.dyn_stat::<Speed>().squeeze_to_i64()) {
							temp += spd_bellow_100;
						}

						if let Some(Perk::Nema(NemaPerk::Grumpiness)) = get_perk!(self, Perk::Nema(NemaPerk::Grumpiness)) 
							&& matches!(self.state, CharacterState::Downed {..}) { 
							temp += 30;
						}
					}

					temp
				}
			}
		};

		let buffed_stat = self.persistent_effects.iter()
			.fold(unbuffed_stat.get(), |mut sum: i64, effect: &PersistentEffect| {
				if let PersistentEffect::Buff { stat: buffed_stat, stat_increase, .. } = effect
					&& stat_kind == *buffed_stat {
					sum += stat_increase.get().squeeze_to_i64();
				} else if let PersistentEffect::Debuff { duration_ms: _duration_ms, debuff_kind } = effect {
					match debuff_kind {
						PersistentDebuff::Standard { stat: debuff_stat, stat_decrease, .. } => {
							if stat_kind == *debuff_stat {
								sum -= stat_decrease.get().squeeze_to_i64();
							}
						}
						PersistentDebuff::StaggeringForce { .. } => {
							if matches!(stat_kind, DynamicStat::Toughness | DynamicStat::StunDef
							| DynamicStat::DebuffRes | DynamicStat::PoisonRes | DynamicStat::MoveRes) {
								sum -= 10;
							}
						}
					}
				}
				
				return sum;
			});

		return T::from_i64(buffed_stat);
	}

	pub fn max_stamina(&self) -> MaxStamina {
		let mut temp = self.stamina_max.to_sat_i64();
		if let Some(Perk::Ethel(EthelPerk::Tank_Energetic)) = get_perk!(self, Perk::Ethel(EthelPerk::Tank_Energetic)) {
			temp *= 125;
			temp /= 100;
		}
		return MaxStamina::new(temp.squeeze_to());
	}

	pub fn can_be_grappled(&self) -> bool {
		return if let CharacterData::Girl(_) = &self.data {
			self.girl_stats.is_some()
		} else {
			false
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
		return if self.can_be_grappled() {
			Some(self.into_grappled_unchecked())
		} else {
			None
		}
	}

	pub fn do_on_zero_stamina(mut self, killer_option: Option<&mut CombatCharacter>,
	                          others: &mut HashMap<Uuid, Entity>, rng: &mut Xoshiro256PlusPlus) {
		// Perk::Ethel_LingeringToxins
		{
			let self_pos = &self.position;

			let (ally_adjacent_center, ally_adjacent_edge) = iter_mut_allies_of!(self, others)
				.fold((None, None), |(center, edge), ally|
					match Position::is_adjacent(self_pos, ally.position()) {
						Some(Direction::Center) => {
							debug_assert!(center.is_none());
							(Some(ally), edge)
						},
						Some(Direction::Edge) => { 
							debug_assert!(edge.is_none());
							(center, Some(ally))
						},
						None => (center, edge),
					});

			for effect in self.persistent_effects.iter() {
				let PersistentEffect::Poison { duration_ms, accumulated_ms, interval_ms,
					poison_per_interval: dmg_per_interval, additives, caster_guid } = effect
						else { continue; };

				if no_matches!(additives, PoisonAdditive::Ethel_LingeringToxins) {
					continue;
				}

				let poison = PersistentEffect::Poison {
					duration_ms     : SaturatedU64::new(duration_ms.get() / 2),
					accumulated_ms  : SaturatedU64::new(accumulated_ms.get() / 2),
					interval_ms     : interval_ms.clone(),
					poison_per_interval: dmg_per_interval.clone(),
					additives       : additives  .clone(),
					caster_guid     : caster_guid.clone(),
				};

				if let Some(Entity::Character(ally)) = ally_adjacent_center {
					ally.persistent_effects.push(poison.clone());
				}
				
				if let Some(Entity::Character(ally)) = ally_adjacent_edge {
					ally.persistent_effects.push(poison);
				}
			}
		}

		// OnKill effects
		killer_option.touch(|killer| {
			if let Some(Perk::Nema(NemaPerk::BattleMage_Triumph)) = get_perk!(killer, Perk::Nema(NemaPerk::BattleMage_Triumph)) {
				let speed_buff = SelfApplier::Buff {
					duration_ms: 3000.to_sat_u64(),
					stat: DynamicStat::Speed,
					stat_increase: unsafe { NonZeroU16::new_unchecked(25) },
				};

				speed_buff.apply(killer, others, rng, false);
				
				killer.girl_stats.touch(|girl| *girl.lust -= 10);
			}

			if let Some(Perk::Nema(NemaPerk::Regret)) = get_perk!(killer, Perk::Nema(NemaPerk::BattleMage_Triumph)) {
				let composure_debuff = TargetApplier::Debuff {
					duration_ms: 5000.to_sat_u64(),
					apply_chance: None,
					applier_kind: DebuffApplierKind::Standard {
						stat: DynamicStat::Composure,
						stat_decrease: unsafe { NonZeroU16::new_unchecked(15) },
					},
				};

				composure_debuff.apply_self(killer, others, rng, false);
			}
		});
		
		if let CharacterState::Grappling(grappling_state) = self.state {
			match grappling_state.victim {
				GrappledGirlEnum::Alive(girl_alive) => {
					let mut girl_standing = girl_alive.into_non_grappled();
					// girl is downed for 2.5s after being released from a grapple
					girl_standing.state = CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(2500.to_sat_u64()) };

					girl_standing.position.order = 0.into();

					for ally in iter_mut_allies_of!(girl_standing, others) {
						ally.position_mut().order += girl_standing.position.size;
					}

					others.insert(girl_standing.guid, Entity::Character(girl_standing));
				}
				GrappledGirlEnum::Defeated(girl_defeated) => {
					let mut girl_standing = girl_defeated.into_non_grappled();
					
					girl_standing.position.order = 0.into();

					for girl_ally in iter_mut_allies_of!(girl_standing, others) {
						girl_ally.position_mut().order += girl_standing.position.size;
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
				self.state = CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(8000.to_sat_u64()) };
				others.insert(self.guid(), Entity::Character(self));
			},
			OnZeroStamina::Vanish => {}, // poof :)
		}
	}

	pub fn increment_skill_counter(&mut self, skill_name: SkillName) {
		self.skill_use_counters
			.entry(skill_name)
			.and_modify(|c| *c += 1)
			.or_insert(1);
	}
	
	pub fn skill_counter_bellow_limit(&self, skill_name: SkillName, limit: u16) -> bool {
		return self.skill_use_counters
			.get(&skill_name)
			.is_none_or(|count| *count < limit);
	}
	
	/// Used to check if character died after losing stamina.
	pub fn stamina_alive(&self) -> bool {
		return self.stamina_cur.get() > 0;
	}
	
	pub fn stamina_dead(&self) -> bool {
		return !self.stamina_alive();
	}

	pub fn iter_perks(&self) -> impl Iterator<Item= &Perk> {
		return self.perks.iter()
			.chain(self.persistent_effects.iter()
				.filter_map(|effect| {
					if let PersistentEffect::TemporaryPerk { perk, .. } = effect {
						Some(perk)
					} else {
						None
					}
				}));
	}
}

impl PartialEq<Self> for CombatCharacter {
	fn eq(&self, other: &Self) -> bool { return self.guid == other.guid; }
}

impl Eq for CombatCharacter { }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrapplingState {
	pub victim: GrappledGirlEnum,
	pub lust_per_interval: NonZeroU8,
	pub temptation_per_interval: NonZeroI8,
	pub duration_ms: SaturatedU64,
	pub accumulated_ms: SaturatedU64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterState {
	Idle,
	Grappling(GrapplingState),
	Downed  { ticks: TrackedTicks },
	Stunned { ticks: TrackedTicks, state_before_stunned: StateBeforeStunned },
	Charging { skill_intention: SkillIntention },
	Recovering { ticks: TrackedTicks },
}

impl CharacterState {
	pub fn spd_charge_ms(remaining_ms: SaturatedU64, character_speed: Speed) -> SaturatedU64 {
		let result = {
			let mut temp = remaining_ms.clone();
			temp *= 100;
			temp /= character_speed.get();
			temp
		};
		
		return result;
	}

	pub fn spd_recovery_ms(remaining_ms: SaturatedU64, character_speed: Speed) -> SaturatedU64 {
		let result = {
			let mut temp = remaining_ms.clone();
			temp *= 100;
			temp /= character_speed.get();
			temp
		};
		
		return result;
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateBeforeStunned {
	Recovering { ticks: TrackedTicks },
	Charging { skill_intention: SkillIntention },
	Idle,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum OnZeroStamina {
	Vanish,
	Corpse,
	Downed,
}