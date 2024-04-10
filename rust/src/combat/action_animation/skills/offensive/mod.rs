use enum_dispatch::enum_dispatch;

#[allow(unused_imports)]
use crate::*;
use crate::combat::action_animation::skills::anim_utils::*;
use crate::combat::shared::*;

pub mod girls;
pub mod npcs;

pub trait OffensiveSkillAnim {
	fn animate(caster_ref: Instance<CharacterNode>, enemies: Vec<(Instance<CharacterNode>, AttackResult)>) -> Sequence;
}

#[enum_dispatch]
pub trait AttackedAnim {
	fn anim_hitted(&self, target_ref: Instance<CharacterNode>, _attacker_ref: Instance<CharacterNode>) -> Sequence { 
		anim_std_hitted(target_ref)
	}
	
	fn anim_killed(&self, target_ref: Instance<CharacterNode>, _attacker_ref: Instance<CharacterNode>) -> Sequence { 
		anim_std_killed(target_ref)
	}
	
	fn anim_dodged(&self, target_ref: Instance<CharacterNode>, _attacker_ref: Instance<CharacterNode>) -> Sequence {
		anim_std_dodged(target_ref)
	}

	fn anim_std_full_counter(&self,
	                         target_ref: Instance<CharacterNode>,
	                         attacker_ref: Instance<CharacterNode>,
	                         before_counter: BeforeCounter,
	                         counter_result: CounterResult)
	                         -> Sequence { 
		anim_std_full_counter(target_ref, attacker_ref, before_counter, counter_result)
	}

	fn anim_counter_only(&self,
	                     target_ref: Instance<CharacterNode>,
	                     attacker_ref: Instance<CharacterNode>,
	                     result: CounterResult)
	                     -> Sequence {
		anim_std_counter_only(target_ref, attacker_ref, result)
	}
	
	fn anim_by_result(&self, target_ref: Instance<CharacterNode>, attacker_ref: Instance<CharacterNode>, result: AttackResult) -> Sequence {
		match result {
			AttackResult::Hitted => self.anim_hitted(target_ref, attacker_ref),
			AttackResult::Killed => self.anim_killed(target_ref, attacker_ref),
			AttackResult::Dodged => self.anim_dodged(target_ref, attacker_ref),
			AttackResult::Counter(before_counter, counter_result) => 
				self.anim_std_full_counter(target_ref, attacker_ref, before_counter, counter_result),
		}
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

pub fn anim_std_hitted(target_ref: Instance<CharacterNode>) -> Sequence {
	let mut seq = Sequence::new().bound_to(&target_ref);
	
	seq.append_call(move || {
		target_ref.touch_assert_safe(|_, node| {
			node_hide(node, "anims/idle");
			node_show(node, "anims/hitted");
			node_play_sound(node, "anims/hitted/sound");
			node_maybe_emit_particles(node, "anims/hitted/particles");
		});
	});
	
	seq
}

pub fn anim_std_killed(target_ref: Instance<CharacterNode>) -> Sequence {
	let mut seq = Sequence::new().bound_to(&target_ref);
	
	let target_ref1= target_ref.clone();
	
	seq.append_call(move || {
		target_ref.touch_assert_safe(|_, node| {
			node_hide(node, "anims/idle");
			node_show(node, "anims/killed");
			node_play_sound(node, "anims/killed/sound");
			node_maybe_emit_particles(node, "anims/killed/particles");
		});
	});
	
	seq.join(target_ref1.do_color(Color::from_rgba(0., 0., 0., 0.), 0.).as_speed_based(0.1));

	seq
}

pub fn anim_std_dodged(target_ref: Instance<CharacterNode>) -> Sequence {
	let mut seq = Sequence::new().bound_to(&target_ref);
	
	seq.append_call(move || {
		target_ref.touch_assert_safe(|_, node| {
			node_hide(node, "anims/idle");
			node_show(node, "anims/dodged");
			node_play_sound(node, "anims/dodged/sound");
			node_maybe_emit_particles(node, "anims/dodged/particles");
		});
	});

	seq
}

pub fn anim_std_full_counter(target_ref: Instance<CharacterNode>,
                             attacker_ref: Instance<CharacterNode>,
                             before_counter: BeforeCounter,
                             counter_result: CounterResult)
                             -> Sequence {
	let mut seq = Sequence::new().bound_to(&target_ref);
	
	let target_ref_c = target_ref.clone();
	seq.append_call(move || {
		target_ref_c.touch_assert_safe(|_, node| {
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
		target_ref.touch_assert_safe(|target, _| {
			target.character()
				  .anim_counter_only(target_ref.clone(), attacker_ref.clone(), counter_result)
				  .register()
				  .log_if_err();
		});
	});

	seq
}

pub fn anim_std_counter_only(target_ref: Instance<CharacterNode>, attacker_ref: Instance<CharacterNode>, result: CounterResult) -> Sequence {
	let mut seq = Sequence::new();
	
	seq.append_call(move || {
		target_ref.touch_assert_safe(|_, node| {
			node_hide(node, "anims/idle");
			node_show(node, "anims/counter");
			node_play_sound(node, "anims/counter/sound");
			node_maybe_emit_particles(node, "anims/counter/particles");
		});
		
		attacker_ref.touch_assert_safe(|atkr, _| {
			atkr.character()
				.anim_by_result(attacker_ref.clone(), target_ref.clone(), result.as_attack_result())
				.register()
				.log_if_err();
		});
	});
	
	seq
}
