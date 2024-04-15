#[allow(unused_imports)]
use crate::*;
use crate::combat::entity::data::girls::ethel::skills::Pierce;
use crate::combat::entity::data::skill_name::EthelSkill;
use crate::combat::shared::*;
use crate::combat::graphics::action_animation::skills::offensive::*;
use crate::combat::graphics::action_animation::test::exported_character::NameWrapper;
use crate::combat::graphics::entity_anim;
use crate::combat::graphics::entity_anim::EntityAnim;
use crate::combat::graphics::stages::CombatStage;

use std::iter;
use gdnative::export::Export;
use gdnative::export::hint::{ArrayHint, EnumHint, IntHint};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;
use strum::VariantNames;
use entity_anim::default_position::calc_default_positions;
use exported_skill::SkillWrapper;

mod exported_skill;
mod exported_character;

#[derive(Default, FromVariant, ToVariant)]
struct Targets(pub Vec<NameWrapper>);

impl Export for Targets {
	type Hint = ArrayHint;

	fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
		let values =
			GirlName::VARIANTS.iter()
			                  .chain(NPCName::VARIANTS)
			                  .map(|v| v.to_string())
			                  .collect::<Vec<_>>();
		
		ArrayHint::with_element_hint::<CharacterName>(<IntHint<u16>>::Enum(EnumHint::new(values))).export_info()
	}
}


#[extends(Node)]
pub struct AnimTester {
	#[export_path] button_play_offensive: Option<Ref<Button>>,
	#[export_path] button_play_defensive: Option<Ref<Button>>,
	#[export_path] button_play_lewd: Option<Ref<Button>>,
	#[property] caster: NameWrapper,
	#[property] targets: Targets,
	#[property] skill: SkillWrapper,
	loaded_characters: Vec<CharacterNode>,
	current_sequence: Option<SequenceID>,
}

unsafe fn load_character(parent: &Node, name: CharacterName) -> CharacterNode {
	CharacterNode::load_spawn(parent, name, Uuid::nil()).unwrap()
}

fn unload_characters(parent: &Node, character: impl Iterator<Item = &CharacterNode>) {
	for character in character {
		parent.remove_child(character.node());
		character.node().touch_assert_sane(|node| node.queue_free());
	}
}

#[methods]
impl AnimTester {
	#[method]
	unsafe fn _ready(&self, #[base] owner: &Node) {
		self.button_play_offensive.unwrap().connect_fn("pressed", owner, fn_name(&Self::_play_offensive));
		self.button_play_defensive.unwrap().connect_fn("pressed", owner, fn_name(&Self::_play_defensive));
		self.button_play_lewd.unwrap().connect_fn("pressed", owner, fn_name(&Self::_play_lewd));
	}
	
	#[method]
	unsafe fn _play_offensive(&mut self, #[base] owner: &Node) {
		unload_characters(owner, self.loaded_characters.iter());
		self.current_sequence
			.take()
			.map(|id| { 
				id.kill()
			});
		
		let caster = load_character(owner, self.caster.0);
		let mut rng = Xoshiro256StarStar::from_entropy();
		let targets = 
			self.targets.0
				.iter()
				.map(|node| {
					(load_character(owner, node.0),
					 match rng.gen_range(0..=2) {
							0 => AttackResult::Hitted,
							1 => AttackResult::Dodged,
							_ => AttackResult::Killed,
						})
				}).collect::<Vec<_>>();

		let all_characters =
			iter::once(caster)
				.chain(targets.iter().map(pluck!(.0)))
				.collect::<Vec<_>>();
		
		self.loaded_characters = all_characters.clone();
		
		let skill: Box<dyn OffensiveAnim> = Box::new(
			match self.skill.0 {
				SkillName::FromEthel(EthelSkill::Pierce) => Pierce,
				_ => return,
			});
		
		let targets_clone = targets.clone();
		
		let mut seq = Sequence::new();
		seq.append_interval(0.1);
		seq.append_call(move || {
			let with_positions =
				iter::once((caster, Position { order: 0.into(), size: caster.name().position_size(), side: Side::Left }))
					.chain(targets.iter().enumerate().map(|(i, (target, _))| {
						(target.clone(), Position { order: i.into(), size: target.name().position_size(), side: Side::Right })
					})).collect::<Vec<_>>();
			
			calc_default_positions(CombatStage::Grove.padding(), with_positions.into_iter())
				.into_iter()
				.for_each(|(character, pos)| {
					character
						.node()
						.touch_assert_sane(|ch| {
							ch.set_position(pos);
						});
				});
		});
		
		seq.append_sequence(skill.offensive_anim(caster, targets_clone));
		self.current_sequence = Some(seq.register().unwrap());
	}
	
	#[method]
	fn _play_defensive(&self) {
		
	}
	
	#[method]
	fn _play_lewd(&self) {
		
	}
}
