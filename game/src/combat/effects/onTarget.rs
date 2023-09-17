use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use fyrox::rand::Rng;
use fyrox::rand::rngs::StdRng;
use crate::combat::{Character, Manager, Side};
use crate::combat::effects::{MoveDirection};
use crate::combat::effects::persistent::PersistentEffect as PersistentEffect;
use crate::combat::effects::persistent as Persistent;
use crate::combat::ModifiableStat;
use crate::combat::ModifiableStat::{DEBUFF_RATE, DEBUFF_RES, MOVE_RATE, MOVE_RES, STUN_DEF};

#[derive(Debug)]
pub enum TargetApplier {
	Arouse {
		duration_ms: isize,
		lust_per_sec: isize,
	},
	Buff {
		duration_ms: isize,
		stat: ModifiableStat,
		modifier: isize,
		apply_chance: Option<isize>,
	},
	MakeSelfGuardTarget { 
		duration_ms: isize, 
	},
	MakeTargetGuardSelf { 
		duration_ms: isize,
	},
	Heal { 
		base_multiplier: isize,
	},
	Lust {
		min: isize,
		max: isize,
	},
	Mark { 
		duration_ms: isize,
	},
	Move {
		direction: MoveDirection,
		apply_chance: Option<isize>,
	},
	PersistentHeal {
		duration_ms: isize,
		heal_per_sec: isize,
	},
	Poison {
		duration_ms: isize,
		dmg_per_sec: isize,
	},
	MakeTargetRiposte {
		duration_ms: isize,
		dmg_multiplier: isize,
		acc: isize,
	},
	Stun {
		force: isize,
	},
	Tempt { 
		intensity: isize,
	},
}

impl TargetApplier {
	pub fn apply(&self, caster_rc: &Rc<RefCell<Character>>, caster_side: Side, target_rc: &Rc<RefCell<Character>>, target_side: &mut Side, seed: &mut StdRng, manager: &Manager) {
		let caster: RefMut<Character> = match caster_rc.try_borrow_mut() {
			Ok(some) => { some }
			Err(err) => {
				eprintln!("Trying to apply effects but caster is already borrowed: {:?}", err);
				return;
			}
		};

		let target: RefMut<Character> = match target_rc.try_borrow_mut() {
			Ok(some) => { some }
			Err(err) => {
				eprintln!("Trying to apply effects but target is already borrowed: {:?}", err);
				return;
			}
		};

		match self {
			TargetApplier::Arouse { duration_ms, lust_per_sec } => {
				target.persistent_effects.push(PersistentEffect::new_arousal(*duration_ms, *lust_per_sec));
			}
			TargetApplier::Buff{ duration_ms, stat, modifier, apply_chance } => {
				match (apply_chance, Side::same_side(&caster_side, target_side)) {
					(Some(chance), false) => {
						let final_chance = chance + caster.stat(DEBUFF_RATE) - target.stat(DEBUFF_RES);
						if seed.gen_range(0..100) > final_chance {
							return;
						}
					}
					_ => {}
				}
				
				target.persistent_effects.push(PersistentEffect::new_buff(*duration_ms, *stat, *modifier));
			}
			TargetApplier::Heal{ base_multiplier } => {
				let max: isize = caster.damage.max.max(0);
				let min: isize = caster.damage.min.clamp(0, max);

				let healAmount: isize;

				if max <= 0 {
					return;
				} else if max == min {
					healAmount = max;
				} else {
					healAmount = (seed.gen_range(min..=max) * base_multiplier) / 100;
				}

				target.stamina_cur = (target.stamina_cur + healAmount).clamp(0, target.stamina_max);
			}
			TargetApplier::Lust{ min, max } => {
				match &mut target.girl {
					None => {
						return;
					}
					Some(girl) => {
						
						let actual_min: isize = *min.min(&(max - 1));
						let lustAmount: isize = seed.gen_range(*min..=*max);
						girl.lust += lustAmount;
					}
				}
			}
			TargetApplier::Mark{ duration_ms } => {
				target.persistent_effects.push(PersistentEffect::new_marked(*duration_ms));
			}
			TargetApplier::Move{ direction, apply_chance } => {
				match (apply_chance, Side::same_side(&caster_side, target_side)) {
					(Some(chance), false) => {
						let final_chance = chance + caster.stat(MOVE_RATE) - target.stat(MOVE_RES);
						if seed.gen_range(0..100) > final_chance {
							return;
						}
					}
					_ => {}
				}

				let direction: isize = match direction {
					MoveDirection::ToCenter(amount) => { -1 * amount.abs() }
					MoveDirection::ToEdge(amount) => { amount.abs() }
				};

				let (index_current, allies): (&mut usize, &Vec<Character>) = match target_side {
					Side::Left(pos) => (pos, &manager.left_characters),
					Side::Right(pos) => (pos, &manager.right_characters),
				};

				*index_current = (((*index_current as isize) + direction) as usize).clamp(0, allies.len());
			}
			TargetApplier::PersistentHeal{ duration_ms, heal_per_sec } => {
				target.persistent_effects.push(PersistentEffect::new_heal(*duration_ms, *heal_per_sec));
			}
			TargetApplier::Poison{ duration_ms, dmg_per_sec } => {
				target.persistent_effects.push(PersistentEffect::new_poison(*duration_ms, *dmg_per_sec, Rc::downgrade(caster_rc)));
			}
			TargetApplier::MakeTargetRiposte{ duration_ms, dmg_multiplier, acc } => {
				target.persistent_effects.push(PersistentEffect::new_riposte(*duration_ms, *dmg_multiplier, *acc));
			}
			TargetApplier::Stun{ force } => {
				let force = *force as f64;
				let def = target.stat(STUN_DEF) as f64;

				let dividend = force + (force * force / 500.0) - def - (def * def / 500.0);
				let divisor = 125.0 + (force * 0.25) + (def * 0.25) + (force * def * 0.0005);

				let bonus_redundancy_ms = ((dividend / divisor) * 4000.0) as i64;

				if bonus_redundancy_ms > 0 {
					match &mut target.stun_redundancy_ms {
						None => { target.stun_redundancy_ms = Some(bonus_redundancy_ms); }
						Some(remaining) => { *remaining += bonus_redundancy_ms; }
					};
				}
			}
			TargetApplier::MakeSelfGuardTarget { duration_ms } => {
				target.persistent_effects.push(PersistentEffect::new_guarded(*duration_ms, Rc::downgrade(caster_rc)));
			}
			TargetApplier::MakeTargetGuardSelf { duration_ms } => {
				caster.persistent_effects.push(PersistentEffect::new_guarded(*duration_ms, Rc::downgrade(target_rc)));
			}
			TargetApplier::Tempt{ intensity } => {}//todo!
		}
	}
}