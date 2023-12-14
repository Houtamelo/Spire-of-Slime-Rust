pub mod bellplant;
pub mod crabdra;

use std::ops::{RangeInclusive};
use rand::prelude::StdRng;
use rand::Rng;
use houta_utils::prelude::{BoundISize, BoundUSize};
use crate::combat::entity::data::character::{CharacterDataTrait};

#[derive(Debug, Clone)]
pub enum NPCData {
	Crabdra,
	Trent,
	Wolfhydra,
	BellPlant,
}

// Q: Why bundle all the NPC data into one enum impl?
// R: It makes it easier to compare their stats which helps with balancing.
impl CharacterDataTrait for NPCData {
	fn stamina_max(&self, level: usize, rng: Option<&mut StdRng>) -> BoundUSize<1, 500> {
		let base = match self {
			NPCData::Crabdra   => 16 + (level * 20) / 10,
			NPCData::Trent     => 18 + (level * 25) / 10,
			NPCData::Wolfhydra => 30 + (level * 15) / 10,
			NPCData::BellPlant => 12 + (level * 12) / 10,
		};

		if rng.is_none() {
			return base.into();
		}

		let amplitude = match self {
			NPCData::Crabdra   => (2 * (100 + level * 15)) / 100,
			NPCData::Trent     => (3 * (100 + level * 15)) / 100,
			NPCData::Wolfhydra => (3 * (100 + level * 15)) / 100,
			NPCData::BellPlant => (1 * (100 + level * 15)) / 100,
		} as isize;

		let rng = rng.unwrap();
		let amplitude = rng.gen_range(-amplitude..=amplitude);
		return (base as isize + amplitude).into();
	}

	fn dmg(&self, level: usize) -> RangeInclusive<usize> {
		let lower = match self {
			NPCData::Crabdra   =>  3 * (100 + level * 14),
			NPCData::Trent     =>  5 * (100 + level * 10),
			NPCData::Wolfhydra => 10 * (100 + level * 10),
			NPCData::BellPlant =>  1 * (100 + level * 15),
		} / 100;

		let upper = ((100 + level * 10) *  match self {
			NPCData::Crabdra   =>  6 * (100 + level * 14),
			NPCData::Trent     => 10 * (100 + level *  8),
			NPCData::Wolfhydra =>  4 * (100 + level * 12),
			NPCData::BellPlant =>  3 * (100 + level * 15),
		}) / 100;

		return lower..=upper;
	}

	fn spd(&self, level: usize) -> BoundUSize<20, 300> {
		return match self {
			NPCData::Crabdra   => 100 + (level * 13) / 10,
			NPCData::Trent     => 100 + (level *  9) / 10,
			NPCData::Wolfhydra => 100 + (level * 12) / 10,
			NPCData::BellPlant => 100 + (level *  8) / 10,
		}.into();
	}

	fn acc(&self, level: usize) -> BoundISize<-300, 300> {
		return match self {
			NPCData::Crabdra   => 0 + (level * 35) / 10,
			NPCData::Trent     => 0 + (level * 32) / 10,
			NPCData::Wolfhydra => 0 + (level * 30) / 10,
			NPCData::BellPlant => 0 + (level * 35) / 10,
		}.into();
	}

	fn crit(&self, level: usize) -> BoundISize<-300, 300> {
		return match self {
			NPCData::Crabdra   => 0 + (level * 10) / 10,
			NPCData::Trent     => 0 + (level * 15) / 10,
			NPCData::Wolfhydra => 0 + (level * 12) / 10,
			NPCData::BellPlant => 0 + (level *  8) / 10,
		}.into();
	}

	fn dodge(&self, level: usize) -> BoundISize<-300, 300> {
		let level = level as isize;
		return match self {
			NPCData::Crabdra   =>   5 + (level * 35) / 10,
			NPCData::Trent     => -10 + (level * 25) / 10,
			NPCData::Wolfhydra =>  25 + (level * 35) / 10,
			NPCData::BellPlant => -20 + (level * 20) / 10,
		}.into();
	}

	fn toughness(&self, level: usize) -> BoundISize<-100, 100> {
		return match self {
			NPCData::Crabdra   => 10 + (level * 18) / 10,
			NPCData::Trent     => 25 + (level * 16) / 10,
			NPCData::Wolfhydra =>  0 + (level * 10) / 10,
			NPCData::BellPlant =>  0 + (level *  7) / 10,
		}.into();
	}

	fn stun_def(&self, level: usize) -> BoundISize<-100, 300> {
		let level = level as isize;
		return match self {
			NPCData::Crabdra   =>  20 + (level * 60) / 10,
			NPCData::Trent     =>  40 + (level * 70) / 10,
			NPCData::Wolfhydra =>  25 + (level * 50) / 10,
			NPCData::BellPlant => -20 + (level * 30) / 10,
		}.into();
	}

	fn debuff_res(&self, level: usize) -> BoundISize<-300, 300> {
		return match self {
			NPCData::Crabdra   => 20 + (level * 50) / 10,
			NPCData::Trent     => 30 + (level * 70) / 10,
			NPCData::Wolfhydra => 15 + (level * 60) / 10,
			NPCData::BellPlant =>  0 + (level * 40) / 10,
		}.into();
	}

	fn debuff_rate(&self, level: usize) -> BoundISize<-300, 300> {
		return match self {
			NPCData::Crabdra   => 0 + (level * 50) / 10,
			NPCData::Trent     => 0 + (level * 60) / 10,
			NPCData::Wolfhydra => 0 + (level * 65) / 10,
			NPCData::BellPlant => 0 + (level * 50) / 10,
		}.into();
	}

	fn move_res(&self, level: usize) -> BoundISize<-300, 300> {
		return match self {
			NPCData::Crabdra   =>  25 + (level * 60) / 10,
			NPCData::Trent     => 100 + (level * 70) / 10,
			NPCData::Wolfhydra =>   0 + (level * 50) / 10,
			NPCData::BellPlant =>  50 + (level * 70) / 10,
		}.into();
	}

	fn move_rate(&self, level: usize) -> BoundISize<-300, 300> {
		return match self {
			NPCData::Crabdra   => 0 + (level * 50) / 10,
			NPCData::Trent     => 0 + (level * 50) / 10,
			NPCData::Wolfhydra => 0 + (level * 50) / 10,
			NPCData::BellPlant => 0 + (level * 50) / 10,
		}.into();
	}

	fn poison_res(&self, level: usize) -> BoundISize<-300, 300> {
		return match self {
			NPCData::Crabdra   =>  0 + (level * 40) / 10,
			NPCData::Trent     => 15 + (level * 55) / 10,
			NPCData::Wolfhydra =>  0 + (level * 50) / 10,
			NPCData::BellPlant => 20 + (level * 70) / 10,
		}.into();
	}

	fn poison_rate(&self, level: usize) -> BoundISize<-300, 300> {
		return match self {
			NPCData::Crabdra   => 0 + (level * 50) / 10,
			NPCData::Trent     => 0 + (level * 60) / 10,
			NPCData::Wolfhydra => 0 + (level * 50) / 10,
			NPCData::BellPlant => 0 + (level * 50) / 10,
		}.into();
	}
}