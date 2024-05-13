#[allow(unused_imports)]
use crate::prelude::*;
use crate::graphics::action_animation::character_movement::CharacterMovement;
use crate::graphics::action_animation::character_position::OffensivePadding;
use crate::graphics::action_animation::skills::anim_utils::*;

#[enum_delegate::delegate(for(CharacterVariant))]
pub trait AttackedAnim { // DO NOT RENAME `_target` TO `target`, IT BREAKS THE ENUM_DELEGATE MACRO
	fn anim_hitted(&self, _target: CharacterNode, _attacker: CharacterNode) -> Sequence {
		anim_std_hitted(_target)
	}

	fn anim_killed(&self, _target: CharacterNode, _attacker: CharacterNode) -> Sequence {
		anim_std_killed(_target)
	}

	fn anim_dodged(&self, _target: CharacterNode, _attacker: CharacterNode) -> Sequence {
		anim_std_dodged(_target)
	}

	fn anim_std_full_counter(
		&self, 
		_target: CharacterNode, 
		attacker: CharacterNode, 
		before_counter: BeforeCounter, 
		counter_result: CounterResult
	) -> Sequence {
		anim_std_full_counter(_target, attacker, before_counter, counter_result)
	}

	fn anim_counter_only(
		&self,
		_target: CharacterNode, 
		attacker: CharacterNode, 
		result: CounterResult
	) -> Sequence {
		anim_std_counter_only(_target, attacker, result)
	}

	fn anim_by_result(&self, _target: CharacterNode, attacker: CharacterNode, result: AttackResult) -> Sequence {
		match result {
			AttackResult::Hitted => self.anim_hitted(_target, attacker),
			AttackResult::Killed => self.anim_killed(_target, attacker),
			AttackResult::Dodged => self.anim_dodged(_target, attacker),
			AttackResult::Counter(before_counter, counter_result) =>
				self.anim_std_full_counter(_target, attacker, before_counter, counter_result),
		}
	}
}

pub trait OffensiveAnim {
	fn offensive_anim(&self, caster: CharacterNode, enemies: Vec<(CharacterNode, AttackResult)>) -> Sequence;
	fn reset(&self, caster: CharacterNode);
	fn padding(&self) -> OffensivePadding;
	fn caster_movement(&self) -> CharacterMovement;
	fn enemies_movement(&self) -> CharacterMovement;
}

#[allow(unused)]
pub struct OffensiveStruct {
	anim: fn(CharacterNode, Vec<(CharacterNode, AttackResult)>) -> Sequence,
	reset: fn(CharacterNode),
	padding: OffensivePadding,
	caster_movement: CharacterMovement,
	enemies_movement: CharacterMovement,
}


impl AttackedAnim for NPCVariant {}
impl AttackedAnim for GirlVariant {}

pub fn play_attackeds_anim(attacker: CharacterNode, targets: &[(CharacterNode, AttackResult)]) {
	for (target, result) in targets {
		target.name()
		      .anim_by_result(*target, attacker, *result)
		      .register()
		      .log_if_err();
	}
}

#[derive(Debug, Copy, Clone)]
pub enum AttackResult {
	Hitted,
	Killed,
	Dodged,
	Counter(BeforeCounter, CounterResult),
}

#[derive(Debug, Copy, Clone)]
pub enum BeforeCounter {
	Hitted,
	Dodged,
}

#[derive(Debug, Copy, Clone)]
pub enum CounterResult {
	Hitted,
	Dodged,
	Killed,
}

impl CounterResult {
	pub fn as_attack_result(&self) -> AttackResult {
		match self {
			CounterResult::Hitted => AttackResult::Hitted,
			CounterResult::Dodged => AttackResult::Dodged,
			CounterResult::Killed => AttackResult::Killed,
		}
	}
}

pub fn anim_std_hitted(target: CharacterNode) -> Sequence {
	let mut seq = Sequence::new().bound_to(&target.node());
	
	seq.append_call(move || {
		target.node().touch_assert_sane(|node| {
			node_hide(node, "anims/idle");
			node_show(node, "anims/hitted");
			node_play_sound(node, "anims/hitted/sound");
			node_maybe_emit_particles(node, "anims/hitted/particles");
		});
	});
	
	seq
}

pub fn anim_std_killed(target: CharacterNode) -> Sequence {
	let mut seq = Sequence::new().bound_to(&target.node());
	
	seq.append_call(move || {
		target.node().touch_assert_sane(|node| {
			node_hide(node, "anims/idle");
			node_show(node, "anims/killed");
			node_play_sound(node, "anims/killed/sound");
			node_maybe_emit_particles(node, "anims/killed/particles");
		});
	});
	
	seq.join(target.node().do_color(Color::from_rgba(0., 0., 0., 0.), 0.).as_speed_based(0.1));

	seq
}

pub fn anim_std_dodged(target: CharacterNode) -> Sequence {
	let mut seq = Sequence::new().bound_to(&target.node());
	
	seq.append_call(move || {
		target.node().touch_assert_sane(|node| {
			node_hide(node, "anims/idle");
			node_show(node, "anims/dodged");
			node_play_sound(node, "anims/dodged/sound");
			node_maybe_emit_particles(node, "anims/dodged/particles");
		});
	});

	seq
}

pub fn anim_std_full_counter(target: CharacterNode,
                             attacker: CharacterNode,
                             before_counter: BeforeCounter,
                             counter_result: CounterResult)
                             -> Sequence {
	let mut seq = Sequence::new().bound_to(&target.node());
	
	seq.append_call(move || {
		target.node().touch_assert_sane(|node| {
			node_hide(node, "anims/idle");
			
			match before_counter {
				BeforeCounter::Hitted => {
					node_show(node, "anims/hitted");
					node_play_sound(node, "anims/hitted/sound");
					node_maybe_emit_particles(node, "anims/hitted/particles");
				}
				BeforeCounter::Dodged => {
					node_show(node, "anims/dodged");
					node_play_sound(node, "anims/dodged/sound");
					node_maybe_emit_particles(node, "anims/dodged/particles");
				}
			}
		});
	});

	seq.append_interval(0.5);
	
	seq.append_call(move || {
		target.name()
			  .anim_counter_only(target.clone(), attacker.clone(), counter_result)
			  .register()
			  .log_if_err();
	});

	seq
}

pub fn anim_std_counter_only(target: CharacterNode, attacker: CharacterNode, result: CounterResult) -> Sequence {
	let mut seq = Sequence::new();
	
	seq.append_call(move || {
		target.node().touch_assert_sane(|node| {
			node_hide(node, "anims/idle");
			node_show(node, "anims/counter");
			node_play_sound(node, "anims/counter/sound");
			node_maybe_emit_particles(node, "anims/counter/particles");
		});
		
		attacker.name()
				.anim_by_result(attacker.clone(), target.clone(), result.as_attack_result())
				.register()
				.log_if_err();
	});
	
	seq
}
