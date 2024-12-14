use combat::prelude::{EthelSkill, Exhaustion, GirlName, NemaSkill};

use super::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct DateTime {
	year: u64,
	month: u64,
	day: u64,
	hour: u64,
	minute: u64,
	second: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveFile {
	pub name: String,
	pub date_time: DateTime,
	pub rng: Xoshiro256PlusPlus,
	pub state: SaveState,
	pub map_location: WorldLocation,
	pub ethel: EthelFile,
	pub nema: NemaFile,
	pub combat_order: IndexedSet<GirlName>,
	pub other_vars: OtherVariables,
	pub affairs: AffairMap,
	pub is_dirty: bool,
}

impl SaveFile {
	pub fn new(name: String) -> SaveFile {
		let time_singleton = Time::singleton();
		let date = &time_singleton.get_date_dict_from_system();
		let time = &time_singleton.get_time_dict_from_system();

		let year = get_from_dict(date, "year");
		let month = get_from_dict(date, "month");
		let day = get_from_dict(date, "day");

		let hour = get_from_dict(time, "hour");
		let minute = get_from_dict(time, "minute");
		let second = get_from_dict(time, "second");

		let date_time = DateTime {
			year,
			month,
			day,
			hour,
			minute,
			second,
		};

		let mut rng = {
			let mut seed = <Xoshiro256PlusPlus as SeedableRng>::Seed::default();
			if let Err(err) = getrandom(&mut seed) {
				godot_error!("Failed to get random seed: {err}");
				Xoshiro256PlusPlus::seed_from_u64(1337)
			} else {
				Xoshiro256PlusPlus::from_seed(seed)
			}
		};

		let ethel = EthelFile {
			stats: GenericStats::from_data(
				Xoshiro256PlusPlus::seed_from_u64(rng.next_u64()),
				combat::prelude::DEFAULT_ETHEL.deref(),
			),
			skill_set: [
				Some(EthelSkill::Clash),
				Some(EthelSkill::Jolt),
				Some(EthelSkill::Safeguard),
				None,
			],
		};

		let nema = NemaFile {
			stats: GenericStats::from_data(
				Xoshiro256PlusPlus::seed_from_u64(rng.next_u64()),
				combat::prelude::DEFAULT_NEMA.deref(),
			),
			skill_set: [Some(NemaSkill::Gawky), Some(NemaSkill::Calm), None, None],
			exhaustion: Exhaustion::from(0),
		};

		SaveFile {
			name,
			date_time,
			rng,
			state: SaveState::WorldMap_Event {
				event: String::new(),
			}, // MemberType
			map_location: WorldLocation::Chapel,
			ethel,
			nema,
			combat_order: IndexedSet::from_iter([GirlName::Ethel, GirlName::Nema]),
			other_vars: OtherVariables::default(),
			affairs: AffairMap::default(),
			is_dirty: true,
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct OtherVariables {
	pub bools: HashMap<String, bool>,
	pub ints: HashMap<String, i32>,
	pub strings: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EthelFile {
	pub stats: GenericStats,
	pub skill_set: [Option<EthelSkill>; 4],
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NemaFile {
	pub stats: GenericStats,
	pub skill_set: [Option<NemaSkill>; 4],
	pub exhaustion: Exhaustion,
}

fn get_from_dict(dict: &Dictionary, key: &str) -> u64 {
	dict.get(key)
		.and_then(|var| var.try_to::<u64>().ok())
		.unwrap_or_else(|| {
			godot_warn!("Failed to get u64 value at key \"{key}\" from dictionary");
			0
		})
}
