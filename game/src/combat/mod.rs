use std::iter::FilterMap;
use std::slice::{Iter, IterMut};
use fyrox::rand::rngs::StdRng;
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::entity::{CombatCharacter, CharacterState, Entity, Position};
use crate::combat::timeline::{TimelineEvent};

mod effects;
mod skills;
mod timeline;
mod entity;

include!("stat.rs");

pub struct CombatState {
	characters: Vec<Entity>,
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

		let all_characters : Vec<&mut CombatCharacter> = self.all_characters_mut().collect();
		
		for character in all_characters {
			match &character.state {
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
		self.all_characters().for_each(|character| TimelineEvent::register_character(character, &mut all_events));
		all_events.sort_by(|a, b| a.time_frame_ms.cmp(&b.time_frame_ms));
		return all_events;
	}
	
	fn apply_effect_self(&mut self, effect: SelfApplier, caster: &mut CombatCharacter) {
		effect.apply(caster,self);
	}

	pub fn left_characters(&self) -> FilterMap<Iter<Entity>, fn(&Entity) -> Option<&CombatCharacter>> {
		return self.characters.iter().filter_map(|entity| match entity{
			Entity::Character(character) => {
				if let Position::Left { .. } = character.position {
					Some(character)
				}
				else {
					None
				}
			}
			Entity::Corpse(_) => { None }
		});
	}
		
	pub fn left_characters_mut(&mut self) -> FilterMap<IterMut<Entity>, fn(&mut Entity) -> Option<&mut CombatCharacter>> {
		return self.characters.iter_mut().filter_map(|entity| match entity{
			Entity::Character(character) => {
				if let Position::Left { .. } = character.position {
					Some(character)
				}
				else {
					None
				}
			}
			Entity::Corpse(_) => { None }
		});
	}
	
	pub fn left_entities(&self) -> FilterMap<Iter<Entity>, fn(&Entity) -> Option<&Entity>> {
		return self.characters.iter()
				.filter_map(|entity|
						if let Position::Left { .. } = entity.position() {
							Some(entity)
						} else { None });
	}

	pub fn left_entities_mut(&mut self) -> FilterMap<IterMut<Entity>, fn(&mut Entity) -> Option<&mut Entity>> {
		return self.characters.iter_mut()
				.filter_map(|entity|
						if let Position::Left { .. } = entity.position() {
							Some(entity)
						} else { None });
	}
	
	pub fn right_characters(&self) -> FilterMap<Iter<Entity>, fn(&Entity) -> Option<&CombatCharacter>> {
		return self.characters.iter().filter_map(|entity| match entity{
			Entity::Character(character) => { 
				if let Position::Right { .. } = character.position {
					Some(character)
				}
				else { None }
			}
			Entity::Corpse(_) => { None }
		});
	}
	
	pub fn right_characters_mut(&mut self) -> FilterMap<IterMut<Entity>, fn(&mut Entity) -> Option<&mut CombatCharacter>> {
		return self.characters.iter_mut().filter_map(|entity| match entity{
			Entity::Character(character) => { 
				if let Position::Right { .. } = character.position {
					Some(character)
				}
				else { None }
			}
			Entity::Corpse(_) => { None }
		});
	}
	
	pub fn right_entities(&self) -> FilterMap<Iter<Entity>, fn(&Entity) -> Option<&Entity>> {
		return self.characters.iter()
				.filter_map(|entity| 
						if let Position::Right { .. } = entity.position() {
							Some(entity) 
						} 
						else { None });
	}
	
	pub fn right_entities_mut(&mut self) -> FilterMap<IterMut<Entity>, fn(&mut Entity) -> Option<&mut Entity>> {
		return self.characters.iter_mut().filter_map(|entity|
				if let Position::Right { .. } = entity.position() {
					Some(entity)
				}
				else { None });
	}
	
	pub fn all_characters(&self) -> FilterMap<Iter<Entity>, fn(&Entity) -> Option<&CombatCharacter>> {
		return self.characters.iter().filter_map(|entity| match entity{
			Entity::Character(character) => { Some(character) }
			Entity::Corpse(_) => { None }
		});
	}
	
	pub fn all_characters_mut(&mut self) -> FilterMap<IterMut<Entity>, fn(&mut Entity) -> Option<&mut CombatCharacter>> {
		return self.characters.iter_mut().filter_map(|entity| match entity{
			Entity::Character(character) => { Some(character) }
			Entity::Corpse(_) => { None }
		});
	}
}


