use std::str::FromStr;
use gdnative::export::Export;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;
#[allow(unused_imports)]
use crate::*;
use crate::combat::entity::data::girls::ethel::skills::Pierce;
use crate::combat::entity::data::skill_name::EthelSkill;
use crate::combat::shared::{CharacterName, CharacterNode, SkillName};
use crate::combat::graphics::action_animation::skills::offensive::*;

#[extends(Node)]
pub struct AnimTester {
	#[export_path] button_play_offensive: Option<Ref<Button>>,
	#[export_path] button_play_defensive: Option<Ref<Button>>,
	#[export_path] button_play_lewd: Option<Ref<Button>>,
	#[export_path] caster: Option<Ref<Node2D>>,
	#[export_path] targets: Vec<Ref<Node2D>>,
	#[property] skill: SkillWrapper,
}

#[repr(transparent)]
#[derive(Copy, Clone)]
struct SkillWrapper(pub SkillName);

impl Default for SkillWrapper {
	fn default() -> Self {
		SkillWrapper(SkillName::FromEthel(EthelSkill::Safeguard))
	}
}

impl FromVariant for SkillWrapper {
	fn from_variant(variant: &Variant) -> std::result::Result<Self, FromVariantError> {
		Ok(SkillWrapper(SkillName::from_variant(variant)?))
	}
}

impl ToVariant for SkillWrapper {
	fn to_variant(&self) -> Variant {
		self.0.to_variant()
	}
}

impl Export for SkillWrapper {
	type Hint = <SkillName as Export>::Hint;

	fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
		SkillName::export_info(_hint)
	}
}

unsafe fn node_to_character(node: Ref<Node2D>) -> CharacterNode {
	let node = node.unwrap_manual();
	let character = CharacterName::from_str(&node.name().to_string()).unwrap();
	
	CharacterNode::new(node.assume_shared(), character, Uuid::nil())
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
	unsafe fn _play_offensive(&self) {
		let caster = node_to_character(self.caster.unwrap());
		let mut rng = Xoshiro256StarStar::from_entropy();
		let targets = 
			self.targets
				.iter()
				.map(|node| {
					(node_to_character(*node), 
						match rng.gen_range(0..=2) {
							0 => AttackResult::Hitted,
							1 => AttackResult::Dodged,
							_ => AttackResult::Killed,
						})
				}).collect::<Vec<_>>();
		
		let skill: Box<dyn OffensiveAnim> = Box::new(
			match self.skill.0 {
				SkillName::FromEthel(EthelSkill::Pierce) => Pierce,
				_ => return,
			});
		
		skill.offensive_anim(caster, targets)
			 .call_when_finished(move || {
				 skill.reset(caster)
					  .log_if_err();
			 }).register()
			 .log_if_err();
	}
	
	#[method]
	fn _play_defensive(&self) {
		
	}
	
	#[method]
	fn _play_lewd(&self) {
		
	}
}