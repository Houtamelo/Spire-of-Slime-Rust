use std::fmt::Debug;
#[allow(unused_imports)]
use crate::*;
use crate::combat::graphics::ui::{get_ref_or_bail, get_tref_or_bail};
use crate::combat::skill_types::SkillData;

pub trait SkillButtonSprites: Debug + SkillData {
	fn background(&self) -> Ref<Texture>;
	fn base(&self) -> Ref<Texture>;
	fn base_fx(&self) -> Option<Ref<Texture>>;
	fn hover(&self) -> Ref<Texture>;
	fn hover_fx(&self) -> Option<Ref<Texture>>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
	Base,
	Hover,
	Selected,
	Disabled,
}

pub const SIGNAL_CLICKED: &str = "clicked";
pub const SIGNAL_ENTERED: &str = "entered";
pub const SIGNAL_EXITED: &str = "exited";

#[derive(Debug, NativeClass)]
#[inherit(Reference)]
#[no_constructor]
#[register_with(Self::register)]
#[user_data(GoodCellData<SkillButton>)]
pub struct SkillButton {
	owner_ref: Ref<Control>,
	background: Ref<TextureRect>,
	frame: Ref<TextureRect>,
	base: Ref<TextureRect>,
	base_idx: i64,
	base_fx: Ref<TextureRect>,
	base_fx_idx: i64,
	hover: Ref<TextureRect>,
	hover_idx: i64,
	hover_fx: Ref<TextureRect>,
	hover_fx_idx: i64,
	selected_indicator: Ref<TextureRect>,
	skill: Box<dyn SkillButtonSprites>,
	state: State,
	anim_seq: Option<SequenceID>,
}

#[methods]
impl SkillButton {
	fn register(builder: &ClassBuilder<SkillButton>) {
		builder.signal(SIGNAL_CLICKED).done();
		builder.signal(SIGNAL_ENTERED).done();
		builder.signal(SIGNAL_EXITED).done();
	}
	
	pub fn build_in(owner: &Control, skill: impl SkillButtonSprites + 'static) -> Result<Instance<Self>> {
		let owner_ref = unsafe { owner.assume_shared() };
		
		let background = get_ref_or_bail!(owner, "background", TextureRect)?;
		let frame = get_ref_or_bail!(owner, "frame", TextureRect)?;
		
		let (base, base_idx) = {
			let node = get_tref_or_bail!(owner, "base", TextureRect)?;
			(unsafe { node.assume_shared() }, node.get_index())
		};
		
		let (base_fx, base_fx_idx) = {
			let node = get_tref_or_bail!(owner, "base_fx", TextureRect)?;
			(unsafe { node.assume_shared() }, node.get_index())
		};
		
		let (hover, hover_idx) = {
			let node = get_tref_or_bail!(owner, "hover", TextureRect)?;
			(unsafe { node.assume_shared() }, node.get_index())
		};
		
		let (hover_fx, hover_fx_idx) = {
			let node = get_tref_or_bail!(owner, "hover_fx", TextureRect)?;
			(unsafe { node.assume_shared() }, node.get_index())
		};
		
		let selected_indicator = get_ref_or_bail!(owner, "selected_indicator", TextureRect)?;
		let skill = Box::new(skill);
		
		let _self = Self {
			owner_ref,
			background,
			frame,
			base,
			base_idx,
			base_fx,
			base_fx_idx,
			hover,
			hover_idx,
			hover_fx,
			hover_fx_idx,
			selected_indicator,
			skill,
			state: State::Base,
			anim_seq: None,
		}.emplace();

		owner.set_script(_self);

		let _self =
			owner.get_script()
			     .ok_or_else(|| anyhow!("Failed to set `{}` script for {}", type_name::<Self>(), owner.name()))
			     .map(|script| {
				     script.cast_instance()
				           .ok_or_else(|| anyhow!("Failed to cast `{}` script for {}", type_name::<Self>(), owner.name()))
			     }).flatten()?;
		
		_self.touch_assert_safe_mut(|s: &mut SkillButton, _| s.set_sprites());

		owner.connect("mouse_entered", _self.clone(), fn_name(&Self::on_mouse_entered),
		              VariantArray::new_shared(), Object::CONNECT_DEFERRED)?;

		owner.connect("mouse_exited", _self.clone(), fn_name(&Self::on_mouse_exited),
					  VariantArray::new_shared(), Object::CONNECT_DEFERRED)?;
		
		Ok(_self)
	}
	
	#[method]
	fn on_mouse_entered(&mut self) {
		match self.state {
			State::Base => {
				self.change_state(State::Hover);
			}
			| State::Hover
			| State::Selected
			| State::Disabled => {}
		}
		
		self.owner_ref.touch_assert_sane(|owner| {
			owner.emit_signal(SIGNAL_ENTERED, &[]);
		});
	}
	
	#[method]
	fn on_mouse_exited(&mut self) {
		match self.state {
			| State::Hover => {
				self.change_state(State::Base);
			}
			| State::Base
			| State::Selected
			| State::Disabled => {}
		}
		
		self.owner_ref.touch_assert_sane(|owner| {
			owner.emit_signal(SIGNAL_EXITED, &[]);
		});
	}
	
	#[method]
	fn on_gui_input(&mut self, event: Ref<InputEvent>) {
		let Some(mouse_event) = (unsafe { event.assume_safe().cast::<InputEventMouseButton>() })
			else { return };
		
		if mouse_event.is_echo()
		|| !mouse_event.is_pressed()
		|| mouse_event.button_index() != GlobalConstants::BUTTON_LEFT {
			return;
		}
		
		match self.state {
			| State::Base
			| State::Hover
			| State::Selected => {
				self.owner_ref.touch_assert_sane(|owner| { 
					owner.emit_signal(SIGNAL_CLICKED, &[]);
				});
			}
			State::Disabled => {}
		}
	}
	
	fn set_sprites(&mut self) {
		self.background.touch_assert_sane(|bg| {
			bg.set_texture(self.skill.background());
		});

		self.base.touch_assert_sane(|base| {
			base.set_texture(self.skill.base());
		});

		self.base_fx.touch_assert_sane(|base_fx| {
			if let Some(texture) = self.skill.base_fx() {
				base_fx.set_texture(texture);
				base_fx.show();
			} else {
				base_fx.hide();
			}
		});
		
		self.hover.touch_assert_sane(|hover| {
			hover.set_texture(self.skill.hover());
		});
		
		self.hover_fx.touch_assert_sane(|hover_fx| {
			if let Some(texture) = self.skill.hover_fx() {
				hover_fx.set_texture(texture);
				hover_fx.show();
			} else {
				hover_fx.hide();
			}
		});
		
		self.change_state(State::Base);
	}
	
	fn change_state(&mut self, state: State) {
		const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
		const INACTIVE_A: f64 = 0.15;
		const DISABLED: Color = Color { r: 0.39216, g: 0.39216, b: 0.39216, a: 1.0 };
		const DISABLED_INACTIVE: Color = Color { r: 0.39216, g: 0.39216, b: 0.39216, a: INACTIVE_A as f32 };
		const FADE_DUR: f64 = 0.5;

		macro_rules! set_modulate {
			($node: expr, $color: expr) => {
				$node.touch_assert_sane(|node| {
					node.set_modulate($color);
				});
			};
		}
		
		self.anim_seq.take().map(|id| id.kill());
		self.state = state;
		
		let Some(root) = (unsafe { self.owner_ref.assume_safe_if_sane() })
			else { return godot_error!("{}: owner is not sane", full_fn_name(&Self::change_state)) };
		
		let mut seq = Sequence::new();
		match state {
			State::Base => {
				set_modulate!(self.background, WHITE);
				set_modulate!(self.frame, WHITE);
				
				root.move_child(self.base, self.hover_idx);
				root.move_child(self.base_fx, self.hover_fx_idx);
				root.move_child(self.hover, self.base_idx);
				root.move_child(self.hover_fx, self.base_fx_idx);
				
				self.base.touch_assert_sane(|base| {
					set_white_ignore_a(base);
					seq.join(self.base.do_fade(1.0, FADE_DUR));
				});
				
				self.base_fx.touch_assert_sane(|fx| {
					if fx.is_visible() {
						set_white_ignore_a(fx);
						seq.join(fx.do_fade(1.0, FADE_DUR));
					}
				});
				
				self.hover.touch_assert_sane(|hover| {
					set_white_ignore_a(hover);
					seq.join(hover.do_fade(INACTIVE_A, FADE_DUR));
				});
				
				self.hover_fx.touch_assert_sane(|fx| {
					if fx.is_visible() {
						set_white_ignore_a(fx);
						seq.join(fx.do_fade(INACTIVE_A, FADE_DUR));
					}
				});
				
				self.selected_indicator.touch_assert_sane(|indicator| {
					indicator.hide();
				});
			}
			State::Hover => {
				set_modulate!(self.background, WHITE);
				set_modulate!(self.frame, WHITE);
				
				root.move_child(self.hover, self.hover_idx);
				root.move_child(self.hover_fx, self.hover_fx_idx);
				root.move_child(self.base, self.base_idx);
				root.move_child(self.base_fx, self.base_fx_idx);

				self.base.touch_assert_sane(|base| {
					set_white_ignore_a(base);
					seq.join(self.base.do_fade(INACTIVE_A, FADE_DUR));
				});

				self.base_fx.touch_assert_sane(|fx| {
					if fx.is_visible() {
						set_white_ignore_a(fx);
						seq.join(fx.do_fade(INACTIVE_A, FADE_DUR));
					}
				});

				self.hover.touch_assert_sane(|hover| {
					set_white_ignore_a(hover);
					seq.join(hover.do_fade(1.0, FADE_DUR));
				});

				self.hover_fx.touch_assert_sane(|fx| {
					if fx.is_visible() {
						set_white_ignore_a(fx);
						seq.join(fx.do_fade(1.0, FADE_DUR));
					}
				});

				self.selected_indicator.touch_assert_sane(|indicator| {
					indicator.hide();
				});
			}
			State::Selected => {
				set_modulate!(self.background, WHITE);
				set_modulate!(self.frame, WHITE);
				
				root.move_child(self.hover, self.hover_idx);
				root.move_child(self.hover_fx, self.hover_fx_idx);
				root.move_child(self.base, self.base_idx);
				root.move_child(self.base_fx, self.base_fx_idx);

				self.base.touch_assert_sane(|base| {
					set_white_ignore_a(base);
					seq.join(self.base.do_fade(INACTIVE_A, FADE_DUR));
				});

				self.base_fx.touch_assert_sane(|fx| {
					if fx.is_visible() {
						set_white_ignore_a(fx);
						seq.join(fx.do_fade(INACTIVE_A, FADE_DUR));
					}
				});

				self.hover.touch_assert_sane(|hover| {
					set_white_ignore_a(hover);
					seq.join(hover.do_fade(1.0, FADE_DUR));
				});

				self.hover_fx.touch_assert_sane(|fx| {
					if fx.is_visible() {
						set_white_ignore_a(fx);
						seq.join(fx.do_fade(1.0, FADE_DUR));
					}
				});

				self.selected_indicator.touch_assert_sane(|indicator| {
					indicator.show();
				});
			}
			State::Disabled => {
				set_modulate!(self.background, DISABLED);
				set_modulate!(self.frame, DISABLED);
				set_modulate!(self.base, DISABLED);
				set_modulate!(self.base_fx, DISABLED);
				set_modulate!(self.hover, DISABLED_INACTIVE);
				set_modulate!(self.hover_fx, DISABLED_INACTIVE);

				root.move_child(self.base, self.hover_idx);
				root.move_child(self.base_fx, self.hover_fx_idx);
				root.move_child(self.hover, self.base_idx);
				root.move_child(self.hover_fx, self.base_fx_idx);

				self.selected_indicator.touch_assert_sane(|indicator| {
					indicator.hide();
				});
			}
		}
		
		fn set_white_ignore_a(rect: &TextureRect) {
			let color = Color {
				a: rect.modulate().a,
				..WHITE
			};
			
			rect.set_modulate(color);
		}
	}
	
	pub fn set_skill(&mut self, skill: impl SkillButtonSprites + 'static) {
		self.skill = Box::new(skill);
		self.set_sprites();
	}
	
	pub fn show(&self) {
		self.owner_ref.touch_assert_sane(|node| {
			node.show();
		});
	}
	
	pub fn hide(&self) {
		self.owner_ref.touch_assert_sane(|node| {
			node.hide();
		});
	}
}
