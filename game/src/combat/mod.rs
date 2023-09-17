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

mod effects;
mod skills;

include!("character.rs");
include!("stat.rs");

pub struct Manager {
	left_characters: Vec<Rc<RefCell<Character>>>,
	right_characters: Vec<Rc<RefCell<Character>>>,
	seed: StdRng,
	elapsed_ms: i64,
}

impl Manager {
	fn apply_effect_self(&mut self, effect: SelfEffect, caster_rc: &Rc<RefCell<Character>>) {
		let mut side = match self.character_side(caster_rc){
			Ok(ok) => {ok}
			Err(err) => {
				eprintln!("Trying to apply effects but caster is not in combat manager: {:?}", err);
				return;
			}
		};
		
		let caster = match caster_rc.try_borrow_mut() {
			Ok(ok) => { ok } 
			Err(err) => {
				eprintln!("Trying to apply effects but caster is already borrowed: {:?}", err);
				return;
			}
		};
		
		match side {
			Side::Left (index) => { self.left_characters.remove(index); } 
			Side::Right(index) => { self.right_characters.remove(index); }
		};
		
		effect.apply(caster_rc, &mut side, &mut self.seed, self);
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


