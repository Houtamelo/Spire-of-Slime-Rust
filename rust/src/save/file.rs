use std::collections::HashMap;
use comfy_bounded_ints::prelude;
use gdnative::api::{Time};
use gdnative::log::godot_warn;
use houta_utils::prelude::IndexedSet;
use prelude::Bound_u8;
use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};
use crate::combat::entity::data::girls;
use crate::combat::entity::data::girls::ethel::skills::EthelSkill;
use crate::combat::entity::data::girls::GirlName;
use crate::combat::entity::data::girls::nema::skills::NemaSkill;
use crate::WorldLocation;
use crate::save::affairs::AffairMap;
use crate::save::stats::GenericStats;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateTime {
	year: u64, month: u64, day: u64,
	hour: u64, minute: u64, second: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaveFile {
	pub(super) name: String,
	pub(super) date_time: DateTime,
	pub(super) rng: Xoshiro256PlusPlus,
	pub(super) map_location: WorldLocation,
	pub(super) ethel: EthelFile,
	pub(super) nema: NemaFile,
	pub(super) combat_order: IndexedSet<GirlName>,
	pub(super) other_vars: OtherVariables,
	pub(super) affairs: AffairMap,
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
			map_location: WorldLocation::Chapel,
			ethel,
			nema,
			combat_order: IndexedSet::from_iter([GirlName::Ethel, GirlName::Nema]),
			other_vars: OtherVariables::default(),
			affairs: AffairMap::default(),
			is_dirty: true,
		};
	}

	pub fn name(&self) -> &str { return &self.name; }
	pub fn map_location(&self) -> WorldLocation { return self.map_location; }
	pub fn ethel(&self) -> &EthelFile { return &self.ethel; }
	pub fn nema(&self) -> &NemaFile { return &self.nema; }
	pub fn combat_order(&self) -> &IndexedSet<GirlName> { return &self.combat_order; }
	pub fn other_vars(&self) -> &OtherVariables { return &self.other_vars; }
	pub fn affairs(&self) -> &AffairMap { return &self.affairs; }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)] // maybe make keys &'static str?
pub struct OtherVariables {
	pub(super) bools: HashMap<String, bool>,
	pub(super) ints: HashMap<String, i32>,
	pub(super) strings: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthelFile {
	stats: GenericStats,
	skill_set: [Option<EthelSkill>; 4],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NemaFile {
	stats: GenericStats,
	skill_set: [Option<NemaSkill>; 4],
	nema_exhaustion: Bound_u8<0, 100>,
	nema_clearing_mist: bool,
}



