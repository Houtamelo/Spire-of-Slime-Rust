use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveApplier {
	pub base_apply_chance: Option<Int>,
	pub direction: MoveDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveDirection {
	Front(Int),
	Back(Int),
}

impl IApplyOnAny for MoveApplier {
	fn apply_on_any(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) {
		//apply chance is only used when the caster and target are enemies
		if let Some(base_apply_chance) = self.base_apply_chance
			&& are_enemies(caster, target)
		{
			let vanguard_denies_move = {
				if let Some(Vanguard { cooldown_ms }) = target.get_perk_mut()
					&& cooldown_ms == 0
				{
					cooldown_ms.set(10000);
					true
				} else {
					false
				}
			};

			if vanguard_denies_move {
				return;
			}

			let chance = {
				let mut temp = base_apply_chance;

				temp += caster.eval_dyn_stat::<MoveRate>(ctx);
				temp -= target.eval_dyn_stat::<MoveRes>(ctx);

				if is_crit {
					temp += CRIT_CHANCE_MODIFIER;
				}

				temp
			};

			if !ctx.rng.base100_chance(chance) {
				// roll failed
				return;
			}
		}

		ctx.move_actor(&*target, self.direction);
	}
}
