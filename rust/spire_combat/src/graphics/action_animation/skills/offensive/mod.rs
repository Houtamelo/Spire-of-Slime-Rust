use super::*;

pub trait AttackedAnim {
	fn anim_hitted(&self, target: &ActorNode, _attacker: &ActorNode) -> SpireTween<Sequence> {
		anim_std_hitted(target)
	}

	fn anim_killed(&self, target: &ActorNode, _attacker: &ActorNode) -> SpireTween<Sequence> {
		anim_std_killed(target)
	}

	fn anim_dodged(&self, target: &ActorNode, _attacker: &ActorNode) -> SpireTween<Sequence> {
		anim_std_dodged(target)
	}

	fn anim_std_full_counter(
		&self,
		target: &ActorNode,
		attacker: &ActorNode,
		attack: AttackResult,
		counter: AttackResult,
	) -> SpireTween<Sequence> {
		anim_std_full_counter(target, attacker, attack, counter)
	}

	fn anim_counter_only(
		&self,
		target: &ActorNode,
		attacker: &ActorNode,
		counter: AttackResult,
	) -> SpireTween<Sequence> {
		anim_std_counter_only(target, attacker, counter)
	}

	fn anim_by_result(
		&self,
		target: &ActorNode,
		attacker: &ActorNode,
		result: OffensiveResult,
	) -> SpireTween<Sequence> {
		use AttackResult::*;
		match (result.attack, result.counter) {
			(Hit { lethal: true }, None) => self.anim_killed(target, attacker),
			(Hit { lethal: false }, None) => self.anim_hitted(target, attacker),
			(Miss, None) => self.anim_dodged(target, attacker),
			(HitGrappler { .. }, _) => todo!(),
			(attack @ Hit { .. }, Some(counter)) => {
				self.anim_std_full_counter(target, attacker, attack, counter)
			}
			(Miss, Some(counter)) => self.anim_counter_only(target, attacker, counter),
		}
	}
}

pub trait OffensiveAnim {
	fn offensive_anim(
		&self,
		caster: &mut ActorNode,
		enemies: &mut [(ActorNode, OffensiveResult)],
	) -> SpireTween<Sequence>;

	fn reset(&self, caster: &mut ActorNode);
	fn padding(&self) -> OffensivePadding;
	fn caster_movement(&self) -> CharacterMovement;
	fn enemies_movement(&self) -> CharacterMovement;
}

#[allow(unused)]
pub struct OffensiveStruct {
	anim: fn(ActorNode, Vec<(ActorNode, OffensiveResult)>) -> Sequence,
	reset: fn(ActorNode),
	padding: OffensivePadding,
	caster_movement: CharacterMovement,
	enemies_movement: CharacterMovement,
}

impl AttackedAnim for NpcName {}
impl AttackedAnim for GirlName {}

pub fn play_attackeds_anim(attacker: &ActorNode, targets: &[(ActorNode, OffensiveResult)]) {
	for (actr, result) in targets {
		actr.ident()
			.anim_by_result(actr, attacker, *result)
			.register();
	}
}

pub fn anim_std_hitted(target: &ActorNode) -> SpireTween<Sequence> {
	let mut seq = SpireSequence::new().bound_to(&target.node());

	seq.append_call({
		let targ = target.clone();
		move || {
			let node = &mut targ.node();
			node_hide(node, "anims/idle");
			node_show(node, "anims/hitted");
			node_play_sound(node, "anims/hitted/sound");
			node_maybe_particles(node, "anims/hitted/particles");
		}
	});

	seq
}

pub fn anim_std_killed(target: &ActorNode) -> SpireTween<Sequence> {
	let mut seq = SpireSequence::new().bound_to(&target.node());

	seq.append_call({
		let targ = target.clone();
		move || {
			let node = &mut targ.node();
			node_hide(node, "anims/idle");
			node_show(node, "anims/killed");
			node_play_sound(node, "anims/killed/sound");
			node_maybe_particles(node, "anims/killed/particles");
		}
	});

	seq.join(
		target
			.node()
			.do_color(Color::from_rgba(0., 0., 0., 0.), 0.)
			.as_speed_based(0.1),
	);

	seq
}

pub fn anim_std_dodged(target: &ActorNode) -> SpireTween<Sequence> {
	let mut seq = SpireSequence::new().bound_to(&target.node());

	seq.append_call({
		let targ = target.clone();
		move || {
			let node = &mut targ.node();
			node_hide(node, "anims/idle");
			node_show(node, "anims/dodged");
			node_play_sound(node, "anims/dodged/sound");
			node_maybe_particles(node, "anims/dodged/particles");
		}
	});

	seq
}

pub fn anim_std_full_counter(
	target: &ActorNode,
	attacker: &ActorNode,
	attack: AttackResult,
	counter: AttackResult,
) -> SpireTween<Sequence> {
	use AttackResult::*;

	let mut seq = SpireSequence::new().bound_to(&target.node());

	seq.append_call({
		let targ = target.clone();
		move || {
			let targ_node = &mut targ.node();
			node_hide(targ_node, "anims/idle");

			match attack {
				Hit { lethal: true } => {
					node_show(targ_node, "anims/killed");
					node_play_sound(targ_node, "anims/killed/sound");
					node_maybe_particles(targ_node, "anims/killed/particles");
				}
				Hit { lethal: false } => {
					node_show(targ_node, "anims/hitted");
					node_play_sound(targ_node, "anims/hitted/sound");
					node_maybe_particles(targ_node, "anims/hitted/particles");
				}
				Miss => {
					node_show(targ_node, "anims/dodged");
					node_play_sound(targ_node, "anims/dodged/sound");
					node_maybe_particles(targ_node, "anims/dodged/particles");
				}
				HitGrappler { .. } => todo!(),
			}
		}
	});

	seq.append_interval(0.5);

	seq.append_call({
		let targ = target.clone();
		let atta = attacker.clone();
		move || {
			targ.ident()
				.anim_counter_only(&targ, &atta, counter)
				.register();
		}
	});

	seq
}

pub fn anim_std_counter_only(
	target: &ActorNode,
	attacker: &ActorNode,
	counter: AttackResult,
) -> SpireTween<Sequence> {
	let mut seq = SpireSequence::new();

	seq.append_call({
		let targ = target.clone();
		let atta = attacker.clone();
		move || {
			let targ_node = &mut targ.node();
			node_hide(targ_node, "anims/idle");
			node_show(targ_node, "anims/counter");
			node_play_sound(targ_node, "anims/counter/sound");
			node_maybe_particles(targ_node, "anims/counter/particles");

			let result = OffensiveResult {
				attack:  counter,
				counter: None,
			};
			atta.ident().anim_by_result(&atta, &targ, result).register();
		}
	});

	seq
}
