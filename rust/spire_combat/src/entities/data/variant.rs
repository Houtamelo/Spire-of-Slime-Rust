use super::*;

delegated_enum! {
	ENUM_OUT: {
		#[derive(Debug, Copy, Clone)]
		#[derive(Serialize, Deserialize)]
		pub enum CharacterVariant {
			Girl(GirlName),
			NPC(NpcName),
		}
	}

	DELEGATES: {
		impl trait EntityAnim {
			[fn prefab_path(&self) -> &'static str]
			[fn required_height(&self) -> f64]
			[fn required_width(&self) -> f64]
			[fn position_size(&self) -> Int]
		}

		impl trait AttackedAnim {
			[fn anim_hitted(&self, target: &ActorNode, attacker: &ActorNode) -> SpireTween<Sequence>]

			[fn anim_killed(&self, target: &ActorNode, attacker: &ActorNode) -> SpireTween<Sequence>]

			[fn anim_dodged(&self, target: &ActorNode, attacker: &ActorNode) -> SpireTween<Sequence>]

			[fn anim_std_full_counter(&self,
				target: &ActorNode,
				attacker: &ActorNode,
				attack: AttackResult,
				counter: AttackResult,
			) -> SpireTween<Sequence>]

			[fn anim_counter_only(
				&self,
				target: &ActorNode,
				attacker: &ActorNode,
				counter: AttackResult,
			) -> SpireTween<Sequence>]

			[fn anim_by_result(
				&self,
				target: &ActorNode,
				attacker: &ActorNode,
				result: OffensiveResult,
			) -> SpireTween<Sequence>]
		}
	}
}
