use std::collections::HashMap;
use comfy_bounded_ints::prelude;
use gdnative::api::{Time};
use gdnative::log::godot_warn;
use houta_utils::prelude::IndexedSet;
use prelude::Bound_u8;
use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};
use crate::combat::entity::data::character::CharacterDataTrait;
use crate::combat::entity::data::girls;
use crate::combat::entity::data::girls::ethel::skills::EthelSkill;
use crate::combat::entity::data::girls::{GirlName, GirlTrait};
use crate::combat::entity::data::girls::nema::skills::NemaSkill;
use crate::combat::entity::data::npc::NPCName;
use crate::combat::entity::stat::*;
use crate::MapLocation;

#[derive(Serialize, Deserialize, Clone)]
pub struct DateTime {
	year: u64, month: u64, day: u64,
	hour: u64, minute: u64, second: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveFile {
	pub(super) name: String,
	pub(super) date_time: DateTime,
	pub(super) rng: Xoshiro256PlusPlus,
	pub(super) map_location: MapLocation,
	pub(super) ethel: EthelFile,
	pub(super) nema: NemaFile,
	pub(super) combat_order: IndexedSet<GirlName>,
	pub(super) other_vars: OtherVariables,
	pub(super) is_dirty: bool,
}

impl SaveFile {
	pub fn new(name: String) -> SaveFile {
		let time_singleton = Time::godot_singleton();
		let date = time_singleton.get_date_dict_from_system(true);
		let time = time_singleton.get_time_dict_from_system(true);
		
		let year = if let Some(year_variant) = date.get("year") && let Ok(year_value) = year_variant.try_to::<u64>() {
			year_value
		} else { godot_warn!("SaveFile::new: Failed to get year from date"); 0 };
		
		let month = if let Some(month_variant) = date.get("month") && let Ok(month_value) = month_variant.try_to::<u64>() {
			month_value
		} else { godot_warn!("SaveFile::new: Failed to get month from date"); 0 };
		
		let day = if let Some(day_variant) = date.get("day") && let Ok(day_value) = day_variant.try_to::<u64>() {
			day_value
		} else { godot_warn!("SaveFile::new: Failed to get day from date"); 0 };
		
		let hour = if let Some(hour_variant) = time.get("hour") && let Ok(hour_value) = hour_variant.try_to::<u64>() {
			hour_value
		} else { godot_warn!("SaveFile::new: Failed to get hour from time"); 0 };
		
		let minute = if let Some(minute_variant) = time.get("minute") && let Ok(minute_value) = minute_variant.try_to::<u64>() {
			minute_value
		} else { godot_warn!("SaveFile::new: Failed to get minute from time"); 0 };
		
		let second = if let Some(second_variant) = time.get("second") && let Ok(second_value) = second_variant.try_to::<u64>() {
			second_value
		} else { godot_warn!("SaveFile::new: Failed to get second from time"); 0 };
		
		let date_time = DateTime { 
			year, month, day,
			hour, minute, second,
		};

		let mut rng = {
			let mut seed = <Xoshiro256PlusPlus as SeedableRng>::Seed::default();
			if let Err(err) = getrandom::getrandom(seed.as_mut()) {
				eprintln!("SaveFile::new: Failed to get random seed: {}", err);
				Xoshiro256PlusPlus::seed_from_u64(1337)
			} else {
				Xoshiro256PlusPlus::from_seed(seed)
			}
		};
		
		let ethel = EthelFile { 
			stats: GenericStats::from_data(Xoshiro256PlusPlus::seed_from_u64(rng.next_u64()), 
										   &girls::ethel::DEFAULT_ETHEL),
			skill_set: [Some(EthelSkill::Clash), Some(EthelSkill::Jolt), Some(EthelSkill::Safeguard), None],
		};
		
		let nema = NemaFile { 
			stats: GenericStats::from_data(Xoshiro256PlusPlus::seed_from_u64(rng.next_u64()), 
										   &girls::nema::DEFAULT_NEMA),
			skill_set: [Some(NemaSkill::Gawky), Some(NemaSkill::Calm), None, None],
			nema_exhaustion: 0.into(),
			nema_clearing_mist: false,
		};
		
		return SaveFile {
			name,
			date_time,
			rng,
			map_location: MapLocation::Chapel,
			ethel,
			nema,
			combat_order: IndexedSet::from_iter([GirlName::Ethel, GirlName::Nema]),
			other_vars: OtherVariables::default(),
			is_dirty: true,
		};
	}

	pub fn name(&self) -> &String { return &self.name; }
}

#[derive(Serialize, Deserialize, Clone, Default)] // maybe make keys &'static str?
pub(super) struct OtherVariables {
	pub(super) bools: HashMap<String, bool>,
	pub(super) ints: HashMap<String, i32>,
	pub(super) strings: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(super) struct EthelFile {
	stats: GenericStats,
	skill_set: [Option<EthelSkill>; 4],
}

#[derive(Serialize, Deserialize, Clone)]
pub(super) struct NemaFile {
	stats: GenericStats,
	skill_set: [Option<NemaSkill>; 4],
	nema_exhaustion: Bound_u8<0, 100>,
	nema_clearing_mist: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub(super) struct GenericStats {
	rng: Xoshiro256PlusPlus,
	total_exp: u64,
	dmg: CheckedRange,
	stamina: MaxStamina, 
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
	crit: CritChance,
	dodge: Dodge,
	lust: Lust, 
	temptation: Temptation, 
	composure: Composure,
	corruption: Bound_u8<0, 100>,
	orgasm_limit: OrgasmLimit, 
	orgasm_count: OrgasmCount,
	primary_upgrades: UpgradesCount_Primary, 
	secondary_upgrades: UpgradesCount_Secondary,
	available_points_primary: u8, 
	available_points_secondary: u8, 
	next_upgrade_options_primary: Option<Vec<Upgrade_Primary>>,
	next_upgrade_options_secondary: Option<Vec<Upgrade_Secondary>>,
	available_points_perk: u8,
	sexual_exp: HashMap<NPCName, u16>,
}

impl GenericStats {
	fn from_data(rng: Xoshiro256PlusPlus, data: &(impl CharacterDataTrait + GirlTrait)) -> GenericStats {
		return GenericStats {
			rng,
			total_exp: 0,
			dmg: data.dmg(0),
			stamina: data.max_stamina(0, None),
			toughness: data.toughness(0),
			stun_def: data.stun_def(0),
			debuff_res: data.debuff_res(0),
			debuff_rate: data.debuff_rate(0),
			move_res: data.move_res(0),
			move_rate: data.move_rate(0),
			poison_res: data.poison_res(0),
			poison_rate: data.poison_rate(0),
			speed: data.spd(0),
			accuracy: data.acc(0),
			crit: data.crit(0),
			dodge: data.dodge(0),
			lust: Lust::new(0),
			temptation: Temptation::new(0),
			composure: data.composure(),
			corruption: 0.into(),
			orgasm_limit: data.orgasm_limit(),
			orgasm_count: OrgasmCount::new(0),
			primary_upgrades: UpgradesCount_Primary::default(),
			secondary_upgrades: UpgradesCount_Secondary::default(),
			available_points_primary: 0,
			available_points_secondary: 0,
			next_upgrade_options_primary: None,
			next_upgrade_options_secondary: None,
			available_points_perk: 0,
			sexual_exp: HashMap::new(),
		};
	}
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub(super) struct UpgradesCount_Primary {
	acc: u8,
	dodge: u8,
	crit: u8,
	toughness: u8,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub(super) struct UpgradesCount_Secondary {
	stun_def: u8,
	move_res: u8,
	debuff_res: u8,
	poison_res: u8,
	move_rate: u8,
	debuff_rate: u8,
	poison_rate: u8,
	composure: u8,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum Upgrade_Primary {
	Acc = 0,
	Dodge = 1,
	Crit = 2,
	Toughness = 3,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum Upgrade_Secondary {
	StunDef = 0,
	MoveRes = 1,
	DebuffRes = 2,
	PoisonRes = 3,
	MoveRate = 4,
	DebuffRate = 5,
	PoisonRate = 6,
	Composure = 8,
}