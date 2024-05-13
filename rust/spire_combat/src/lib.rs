#![feature(let_chains)]
#![feature(result_flattening)]
#![feature(iter_from_coroutine)]
#![feature(coroutines)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(exclusive_range_pattern)]
#![feature(inline_const_pat)]
#![feature(const_option)]
#![feature(iterator_try_collect)]
#![allow(non_camel_case_types)]

use prelude::*;

mod effects;
mod skill_types;
mod timeline;
mod skill_resolving;
mod perk;
pub mod entity;
pub mod graphics;
pub mod state;

#[allow(unused)]
pub mod prelude {
	pub use util::prelude::*;
	pub use util_gdnative::prelude::*;
	pub use comfy_bounded_ints::prelude::*;
	pub use shared::num::*;
	pub use shared::rand_utils::*;
	pub use serde::{Deserialize, Serialize};
	pub use uuid::Uuid;
	pub use std::num::{NonZeroI8, NonZeroU16, NonZeroU64, NonZeroU8};
	pub use rand::Rng;
	pub use rand_xoshiro::Xoshiro256PlusPlus;
	pub use gdnative_tweener::prelude::*;
	pub use std::borrow::Cow;
	
	pub(crate) use super::perk::{get_perk, get_perk_mut, has_perk, Perk};
	pub use super::skill_types::{ACCMode, CRITMode, DMGMode, PositionMatrix, Skill, SkillData, UseCounter};
	pub use super::skill_types::lewd::LewdSkill;
	pub use super::skill_types::defensive::DefensiveSkill;
	pub use super::skill_types::offensive::{CustomOffensiveModifier, OffensiveSkill};
	pub use super::graphics::entity_anim::character_node::CharacterNode;
	pub use super::entity::{Corpse, Entity};
	pub(crate) use super::entity::{iter_allies_of, iter_enemies_of, iter_mut_allies_of, iter_mut_enemies_of};
	pub use super::entity::character::{CharacterState, CombatCharacter, GrapplingState, StateBeforeStunned};
	pub use super::entity::girl::{AliveGirl_Grappled, DefeatedGirl_Entity, DefeatedGirl_Grappled, GirlState, GrappledGirlEnum};
	pub use super::entity::position::{Direction, Position, Side};
	pub use super::entity::stat::*;
	pub use super::entity::data::{CharacterData, CharacterDataVariant, EntityDataVariant, NPCData};
	pub use super::entity::data::variant::CharacterVariant;
	pub use super::entity::data::asset_folder::AssetFolder;
	pub use super::entity::data::display_name::DisplayName;
	pub use super::entity::data::girls::{GirlData, GirlDataVariant, GirlVariant};
	pub use super::entity::data::girls::nema::NemaData;
	pub use super::entity::data::girls::ethel::EthelData;
	pub use super::entity::data::npc::NPCVariant;
	pub use super::entity::data::skill_variant::SkillVariant;

	#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
	pub struct TrackedTicks {
		pub remaining_ms: SaturatedU64,
		pub initial_ms: SaturatedU64,
	}

	impl TrackedTicks {
		pub fn from_milliseconds(milliseconds: SaturatedU64) -> TrackedTicks {
			return TrackedTicks {
				remaining_ms: milliseconds,
				initial_ms: milliseconds,
			};
		}
	}
}