#![allow(clippy::useless_format)]
#![allow(clippy::needless_return)]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn insert_combat_character_fields(_item: TokenStream) -> TokenStream {
	let mut output = _item.to_string();
	let fields =
			"	pub(super) size: houta_utils::prelude::BoundUSize<1, 4>,
    pub(super) dmg        : std::ops::RangeInclusive<usize>,
    pub(super) spd        : houta_utils::prelude::comfy_bounded_ints::prelude::Bound_u16<20, 300>,
    pub(super) acc        : houta_utils::prelude::comfy_bounded_ints::prelude::Bound_i16<-300, 300>,
    pub(super) crit       : houta_utils::prelude::comfy_bounded_ints::prelude::Bound_i16<-300, 300>,
    pub(super) dodge      : houta_utils::prelude::comfy_bounded_ints::prelude::Bound_i16<-300, 300>,
    pub(super) stamina_max: houta_utils::prelude::comfy_bounded_ints::prelude::Bound_u16<1, 500>,
    pub(super) toughness  : houta_utils::prelude::comfy_bounded_ints::prelude::Bound_i8<-100, 100>,
    pub(super) stun_def   : houta_utils::prelude::comfy_bounded_ints::prelude::Bound_i16<-100, 300>,
    pub(super) debuff_res : houta_utils::prelude::comfy_bounded_ints::prelude::Bound_i16<-300, 300>,
    pub(super) debuff_rate: houta_utils::prelude::comfy_bounded_ints::prelude::Bound_i16<-300, 300>,
    pub(super) move_res   : houta_utils::prelude::comfy_bounded_ints::prelude::Bound_i16<-300, 300>,
    pub(super) move_rate  : houta_utils::prelude::comfy_bounded_ints::prelude::Bound_i16<-300, 300>,
    pub(super) poison_res : houta_utils::prelude::comfy_bounded_ints::prelude::Bound_i16<-300, 300>,
    pub(super) poison_rate: houta_utils::prelude::comfy_bounded_ints::prelude::Bound_i16<-300, 300>,";

	let left_bracket_index = output.find('{').unwrap();
	output.insert_str(left_bracket_index + 1, fields);
	return output.parse().unwrap();
}


/// usage example: positions!(🛑|✔️|✔️|✔️),
#[proc_macro]
pub fn positions(_item: TokenStream) -> TokenStream {
	let output = _item.to_string().replace('\"', "");
	let bools = output.split('|').collect::<Vec<&str>>();
	let one = bools[0];
	let two = bools[1];
	let tree = bools[2];
	let four = bools[3];

	let one_b = match one.trim() {
		"✔️" => true,
		"🛑" => false,
		_ => panic!("Invalid value for one: {}", one),
	};

	let two_b = match two.trim() {
		"✔️" => true,
		"🛑" => false,
		_ => panic!("Invalid value for two: {}", two),
	};

	let tree_b = match tree.trim() {
		"✔️" => true,
		"🛑" => false,
		_ => panic!("Invalid value for tree: {}", tree),
	};

	let four_b = match four.trim() {
		"✔️" => true,
		"🛑" => false,
		_ => panic!("Invalid value for four: {}", four),
	};

	return format!("crate::combat::skill_types::PositionMatrix {{ positions: [{one_b}, {two_b}, {tree_b}, {four_b}] }}").parse().unwrap();
}

/*
#[proc_macro]
pub fn get_perk(_item: TokenStream) -> TokenStream {
	let string = _item.to_string();
	let inputs = string.split(',').collect::<Vec<&str>>();
	if inputs.len() != 2 {
		panic!("Invalid number of arguments for get_perk! Expected 2, got {}", inputs.len());
	}

	let owner = inputs[0];
	let perk_type = inputs[1];

	return format!("{{
		 'outer: loop {{
			for perk in {owner}.perks.iter() {{
				if let {perk_type} = perk {{
					break 'outer Some(perk);
				}}
			}}

			for effect in {owner}.persistent_effects.iter() {{
				if let crate::combat::effects::persistent::PersistentEffect::TemporaryPerk {{ perk, .. }} = effect {{
					if let {perk_type} = perk {{
						break 'outer Some(perk);
					}}
				}}
			}}

			break None;
		}}
	}}").parse().unwrap();
}

#[proc_macro]
pub fn get_perk_mut(_item: TokenStream) -> TokenStream {
	let string = _item.to_string();
	let inputs = string.split(',').collect::<Vec<&str>>();
	if inputs.len() != 2 {
		panic!("Invalid number of arguments for get_perk! Expected 2, got {}", inputs.len());
	}

	let owner = inputs[0];
	let perk_type = inputs[1];

	return format!("{{
		 'outer: loop {{
			for perk in {owner}.perks.iter_mut() {{
				if let {perk_type} = perk {{
					break 'outer Some(perk);
				}}
			}}

			for effect in {owner}.persistent_effects.iter_mut() {{
				if let crate::combat::effects::persistent::PersistentEffect::TemporaryPerk {{ perk, .. }} = effect {{
					if let {perk_type} = perk {{
						break 'outer Some(perk);
					}}
				}}
			}}

			break None;
		}}
	}}").parse().unwrap();
}
*/

