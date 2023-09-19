use std::cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut};
use std::rc::Rc;
use fyrox::core::algebra::clamp;
use fyrox::event::VirtualKeyCode::Mute;
use fyrox::rand::Rng;
use fyrox::rand::rngs::StdRng;
use fyrox::scene::base::PropertyValue::String;
use crate::combat::effects::persistent;
use crate::combat::ModifiableStat::{MOVE_RATE, MOVE_RES};
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::entity::{CombatCharacter, CharacterState, Entity};
use crate::combat::timeline::{EventType, TimelineEvent};

mod effects;
mod skills;
mod timeline;
mod entity;

include!("stat.rs");

pub struct CombatState {
	left_characters: Vec<Entity>,
	right_characters: Vec<Entity>,
	seed: StdRng,
	elapsed_ms: i64,
}

impl CombatState {
	pub fn run(&mut self) {
		loop {
			let events = self.get_timeline_events();
			if events.len() == 0 {
				break;
			}
			
			let next_event = &events[0];
			let should_break = self.tick(next_event.time_frame_ms);
			if should_break { 
				break;
			}
		}
	}

	fn tick(&mut self, delta_time_ms: i64) -> bool {
		let mut should_break = false;
		self.elapsed_ms += delta_time_ms;

		let all_characters : Vec<&mut Rc<RefCell<CombatCharacter>>> = self.left_characters.iter_mut().filter_map(|entity| match entity{
			Entity::Character(character) => { Some(character) }
			Entity::Corpse => { None }
		}).collect();
		
		
		for character in all_characters {
			let mut_char: &mut CombatCharacter = character.get_mut();
			match &mut_char.state {
				CharacterState::Idle => {
					// todo! run AI here
				} 
				CharacterState::Grappling { victim, lust_per_sec, temptation_per_sec, accumulated_ms } => {
					
				} 
				CharacterState::Downed { .. } => {}
				CharacterState::Stunned { .. } => {}
				CharacterState::Charging { .. } => {} 
				CharacterState::Recovering { .. } => {}
			}
		}
		
		return true;
	}
	
	fn get_timeline_events(&self) -> Vec<TimelineEvent> {
		let mut all_events: Vec<TimelineEvent> = Vec::new();
		self.left_characters .iter().filter_map(|entity| match entity{
			Entity::Character(character) => { Some(character) }
			Entity::Corpse => { None }
		}).for_each(|character_rc| TimelineEvent::register_character(character_rc, &mut all_events));
		
		self.right_characters.iter().filter_map(|entity| match entity{
			Entity::Character(character) => { Some(character) }
			Entity::Corpse => { None }
		}).for_each(|character_rc| TimelineEvent::register_character(character_rc, &mut all_events));
		
		all_events.sort_by(|a, b| a.time_frame_ms.cmp(&b.time_frame_ms));
		return all_events;
	}
	
	fn apply_effect_self(&mut self, effect: SelfApplier, caster_rc: &mut Rc<RefCell<CombatCharacter>>) {
		let mut side = match self.character_side(caster_rc){
			Ok(ok) => {ok}
			Err(err) => {
				eprintln!("Trying to apply effects but caster is not in combat manager: {:?}", err);
				return;
			}
		};

		match side {
			Side::Left (index) => { self.left_characters.remove(index); }
			Side::Right(index) => { self.right_characters.remove(index); }
		};
		
		effect.apply(caster_rc, &mut side, self);
	}
	
	// Returns the side and index of the character with the given guid.
	fn guid_side(&mut self, guid: usize) -> Result<Side, std::string::String> {
		for i in 0..self.left_characters.len() {
			if let Entity::Character(ch) = &mut self.left_characters[i] {
				if ch.get_mut().guid == guid {
					return Ok(Side::Left(i));
				}
			}
		}
		
		for i in 0..self.right_characters.len() {
			if let Entity::Character(ch) = &mut self.right_characters[i] {
				if ch.get_mut().guid == guid {
					return Ok(Side::Right(i));
				}
			}
		}
		
		return Err(format!("Character with guid {} not found in combat manager", guid));
	}
	
	fn character_side(&self, character_rc: &Rc<RefCell<CombatCharacter>>) -> Result<Side, std::string::String> {
		for i in 0..self.left_characters.len() {
			if let Entity::Character(ch) = &self.left_characters[i] {
				if Rc::ptr_eq(ch, character_rc) {
					return Ok(Side::Left(i));
				}
			}
		}
		
		for i in 0..self.right_characters.len() {
			if let Entity::Character(ch) = &self.right_characters[i] {
				if Rc::ptr_eq(ch, character_rc) {
					return Ok(Side::Right(i));
				}
			}
		}
		
		return Err(format!("Character {:?} not found in combat manager", character_rc));
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
	Left(usize),
	Right(usize),
}

impl Side {
	pub fn same_side(a: &Side, b: &Side) -> bool {
		return match (a, b) {
			(Side::Left (_), Side::Left (_)) => true,
			(Side::Right(_), Side::Right(_)) => true,
			_ => false,
		};
	}
}


