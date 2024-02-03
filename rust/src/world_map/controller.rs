use std::collections::HashMap;
use gdnative::api::Line2D;

use gdnative::prelude::*;
use gdnative_export_node_as_path::extends;
use houta_utils_gdnative::prelude::*;
use crate::util;

use super::location::WorldLocation;
use super::{
	SIGNAL_OPEN_SETTINGS_MENU,
	SIGNAL_OPEN_CHARACTER_MENU,
	SIGNAL_LOCATION_CLICKED,
};

#[extends(Node)]
#[register_with(Self::register)]
#[derive(Debug)]
pub struct WorldMapController {
	#[export_path] button_settings_menu: Option<Ref<Button>>,
	#[export_path] button_character_menu: Option<Ref<Button>>,
	
	#[export_path] button_chapel: Option<Ref<Button>>,
	#[export_path] button_grove: Option<Ref<Button>>,
	#[export_path] button_cave: Option<Ref<Button>>,
	#[export_path] button_forest: Option<Ref<Button>>,
	
	#[export_path] line_chapel_grove: Option<Ref<Line2D>>,
	#[export_path] line_grove_forest: Option<Ref<Line2D>>,
	#[export_path] line_forest_cave: Option<Ref<Line2D>>,
	#[export_path] line_cave_chapel: Option<Ref<Line2D>>,
	
	mapped_lines: HashMap<(WorldLocation, WorldLocation), Ref<Line2D>>,
}

#[methods]
impl WorldMapController {
	fn register(builder: &ClassBuilder<Self>) {
		builder.signal(SIGNAL_OPEN_SETTINGS_MENU).done();
		builder.signal(SIGNAL_OPEN_CHARACTER_MENU).done();
		builder.signal(SIGNAL_LOCATION_CLICKED)
			.with_param("location", VariantType::Object)
			.done();
	}
	
	#[method]
	fn _ready(&mut self, #[base] owner: &Node) {
		self.grab_nodes_by_path(owner);
		let owner_ref = unsafe { owner.assume_shared() };
		
		self.mapped_lines.insert((WorldLocation::Chapel, WorldLocation::Grove), self.line_chapel_grove.unwrap());
		self.mapped_lines.insert((WorldLocation::Grove, WorldLocation::Forest), self.line_grove_forest.unwrap());
		self.mapped_lines.insert((WorldLocation::Forest, WorldLocation::Cave), self.line_forest_cave.unwrap());
		self.mapped_lines.insert((WorldLocation::Cave, WorldLocation::Chapel), self.line_cave_chapel.unwrap());
		
		self.button_settings_menu.unwrap_manual()
			.connect("pressed", owner_ref, util::fn_name(&Self::_button_pressed_settings_menu), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
	}
	
	fn location_button(&self, location: WorldLocation) -> Ref<Button> {
		return match location {
			WorldLocation::Chapel => self.button_chapel,
			WorldLocation::Grove => self.button_grove,
			WorldLocation::Forest => self.button_forest,
			WorldLocation::Cave => self.button_cave,
		}.unwrap();
	}
	
	#[method]
	fn _button_pressed_settings_menu(&self, #[base] owner: &Node) {
		owner.emit_signal(SIGNAL_OPEN_SETTINGS_MENU, &[]);
	}
	
	#[method]
	fn _button_pressed_character_menu(&self, #[base] owner: &Node) {
		owner.emit_signal(SIGNAL_OPEN_CHARACTER_MENU, &[]);
	}
}