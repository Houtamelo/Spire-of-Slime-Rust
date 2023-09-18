use std::cell::{BorrowError, BorrowMutError, RefCell, RefMut};
use std::rc::Rc;
use fyrox::core::algebra::clamp;
use fyrox::event::VirtualKeyCode::Mute;
use fyrox::rand::Rng;
use fyrox::rand::rngs::StdRng;
use fyrox::scene::base::PropertyValue::String;
use crate::combat::effects::persistent;
use crate::combat::ModifiableStat::{MOVE_RATE, MOVE_RES};
use crate::combat::effects::onSelf::SelfApplier as SelfEffect;
use crate::combat::entity::{Character, CharacterState, Entity};
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

		let all_characters : Vec<&mut Rc<RefCell<Character>>> = self.left_characters.iter_mut().filter_map(|entity| match entity{
			Entity::Character(character) => { Some(character) }
			Entity::Corpse => { None }
		}).collect();
		
		for character in all_characters {
			let mut_char: &mut Character = character.get_mut();
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
	
	fn apply_effect_self(&mut self, effect: SelfEffect, caster_rc: &mut Rc<RefCell<Character>>) {
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
	fn guid_side(&self, guid: usize) -> Result<Side, std::string::String> {
		{
			let try_error = self.left_characters.iter().try_for_each(|rc| {
				let result = rc.try_borrow();
				if result.is_err() {
					return Err(result.unwrap_err());
				}
				Ok(())
			});

			if try_error.is_err() {
				return Err(try_error.unwrap_err().to_string());
			}
			
			let pos = self.left_characters.iter().position(|c| c.borrow().guid == guid);
			match pos{
				Some(index) => { return Ok(Side::Left(index)); }
				None => {}
			};
		}

		{
			let try_error = self.right_characters.iter().try_for_each(|rc| {
				let result = rc.try_borrow();
				if result.is_err() {
					return Err(result.unwrap_err());
				}
				Ok(())
			});
			
			if try_error.is_err() {
				return Err(try_error.unwrap_err().to_string());
			}
			
			let pos = self.right_characters.iter().position(|c| c.borrow().guid == guid);
			match pos{
				Some(index) => { return Ok(Side::Right(index)); }
				None => {}
			};
		}
		
		return Err(format!("Character with guid {} not found in combat manager", guid));
	}
	
	fn character_side(&self, character_rc: &Rc<RefCell<Character>>) -> Result<Side, std::string::String> {
		self.left_characters.iter().position(|c| Rc::ptr_eq(c, character_rc)).map_or
		(match self.right_characters.iter().position(|c| Rc::ptr_eq(c, character_rc)) {
			Some(index) => Ok(Side::Right(index)),
			None => Err(format!("Character {:?} not found in combat manager", character_rc)),
		}, |index| Ok(Side::Left(index)))
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


