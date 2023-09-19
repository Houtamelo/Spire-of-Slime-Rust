include!("skill_intention.rs");
include!("character.rs");

pub enum Entity {
	Character(Rc<RefCell<CombatCharacter>>),
	Corpse,
}
