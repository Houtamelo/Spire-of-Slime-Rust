use super::*;

mod action_animation;
mod entity_anim;
mod stages;
mod ui;

pub use action_animation::*;
pub use entity_anim::*;
pub use stages::*;
pub use ui::*;

#[derive(GodotClass)]
#[class(no_init, base = Node2D)]
pub struct CombatScene {
	animation_nodes: Gd<AnimationNodes>,
	bg: Gd<Node2D>,
	bg_serial: SerializedBG,
	character_stats_left: Gd<ActorStatsUI>,
	character_stats_right: Gd<ActorStatsUI>,
	targeting_tooltip: Gd<TargetingTooltip>,
	speed_buttons: Gd<SpeedButtons>,
}

#[godot_api]
impl CombatScene {
	pub fn load(parent: &mut Node, stage: CombatBG, rng: &mut impl Rng) -> Result<Gd<Self>> {
		let mut animation_nodes =
			spawn_prefab_as::<AnimationNodes>("res://core/combat/scene_combat.tscn")?;

		let (bg, bg_serial) = stage.spawn_randomized(&mut animation_nodes, rng)?;

		let mut character_stats_left = animation_nodes
			.get_child::<ActorStatsUI>("canvas-layer_default/ui/actors-stats/left-side")?;
		character_stats_left.hide();

		let mut character_stats_right = animation_nodes
			.get_child::<ActorStatsUI>("canvas-layer_default/ui/actors-stats/right-side")?;
		character_stats_right.hide();

		let mut targeting_tooltip = animation_nodes
			.get_child::<TargetingTooltip>("canvas-layer_default/ui/targeting_tooltip")?;
		targeting_tooltip.hide();

		let speed_buttons =
			animation_nodes.get_child::<SpeedButtons>("canvas-layer_default/ui/speed-buttons")?;

		let this = Gd::from_object(Self {
			animation_nodes: animation_nodes.clone(),
			bg,
			bg_serial,
			character_stats_left,
			character_stats_right,
			targeting_tooltip,
			speed_buttons,
		});

		parent.add_child(&this);

		Ok(this)
	}
}
