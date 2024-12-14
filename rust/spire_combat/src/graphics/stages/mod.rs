use super::*;

mod cave;
mod forest;
mod grove;
mod randomizer;
mod serialization;

use cave::*;
use forest::*;
use grove::*;
pub use randomizer::*;
pub use serialization::*;

#[derive(GodotConvert)]
#[godot(via = GString)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CombatBG {
	#[default]
	Grove,
	Forest,
	Cave,
}

impl Var for CombatBG {
	fn get_property(&self) -> Self::Via { self.to_godot() }

	fn set_property(&mut self, value: Self::Via) {
		if let Ok(new) = Self::try_from_godot(value.clone()) {
			*self = new;
		} else {
			godot_error!("Invalid CombatBG value: {value:?}");
		}
	}
}

impl Export for CombatBG {}

impl CombatBG {
	pub fn padding(&self) -> StagePadding {
		match self {
			CombatBG::Grove => default(),
			CombatBG::Forest => default(),
			CombatBG::Cave => default(),
		}
	}

	fn path(&self) -> &'static str {
		match self {
			CombatBG::Grove => "res://Core/Combat/Backgrounds/Grove/grove.tscn",
			CombatBG::Forest => "res://Core/Combat/Backgrounds/Forest/forest.tscn",
			CombatBG::Cave => "res://Core/Combat/Backgrounds/Cave/cave.tscn",
		}
	}

	fn randomize(&self, rng: &mut impl Rng, parent: &Node2D, name: &str) -> Result<SerializedBG> {
		let bg_tree = match self {
			CombatBG::Grove => GROVE_NODE,
			CombatBG::Forest => FOREST_NODE,
			CombatBG::Cave => CAVE_NODE,
		};

		let tree = bg_tree.randomize_recursive(rng, name, parent)?;
		Ok(SerializedBG { stage: *self, tree })
	}

	fn spawn(&self, parent: &mut Node2D) -> Result<Gd<Node2D>> {
		let bg = spawn_prefab_as::<Node2D>(self.path())?;
		parent.add_child(&bg);
		Ok(bg)
	}

	pub fn spawn_randomized(
		&self,
		parent: &mut Node2D,
		rng: &mut impl Rng,
	) -> Result<(Gd<Node2D>, SerializedBG)> {
		let bg = spawn_prefab_as::<Node2D>(self.path())?;
		parent.add_child(&bg);

		let serial = self.randomize(rng, parent, &bg.get_name().to_string())?;

		Ok((bg, serial))
	}

	pub fn deserialize(serial: SerializedBG, parent: &mut Node2D) -> Result<Gd<Node2D>> {
		serial.deserialize(parent)
	}
}

#[derive(Debug, Copy, Clone)]
pub struct StagePadding {
	center_to_left: f64,
	center_to_right: f64,
	entity_y: f64,
}

fn default() -> StagePadding {
	StagePadding {
		center_to_left: 150.,
		center_to_right: 150.,
		entity_y: 120.,
	}
}

impl StagePadding {
	pub fn center_to_left(&self) -> f64 { self.center_to_left }
	pub fn center_to_right(&self) -> f64 { self.center_to_right }
	pub fn entity_y(&self) -> f64 { self.entity_y }
}
