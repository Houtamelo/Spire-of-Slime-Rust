use super::*;

pub trait SkillButtonSprites: Debug + SkillData {
	fn background(&self) -> Gd<Texture2D>;
	fn idle(&self) -> Gd<Texture2D>;
	fn idle_fx(&self) -> Option<Gd<Texture2D>>;
	fn hover(&self) -> Gd<Texture2D>;
	fn hover_fx(&self) -> Option<Gd<Texture2D>>;
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
enum State {
	#[default]
	ActorBase,
	Hover,
	Selected,
	Disabled,
}

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct SkillButton {
	base: Base<Control>,
	#[init(node = "background")]
	background: OnReady<Gd<TextureRect>>,
	#[init(node = "frame")]
	frame: OnReady<Gd<TextureRect>>,
	#[init(node = "idle")]
	idle: OnReady<Gd<TextureRect>>,
	idle_idx: i32,
	#[init(node = "idle_fx")]
	idle_fx: OnReady<Gd<TextureRect>>,
	idle_fx_idx: i32,
	#[init(node = "hover")]
	hover: OnReady<Gd<TextureRect>>,
	hover_idx: i32,
	#[init(node = "hover_fx")]
	hover_fx: OnReady<Gd<TextureRect>>,
	hover_fx_idx: i32,
	#[init(node = "selected_indicator")]
	selected_indicator: OnReady<Gd<TextureRect>>,
	skill: Option<Box<dyn SkillButtonSprites>>,
	state: State,
	anim_seq: Option<SpireHandle<Sequence>>,
}

#[godot_api]
impl IControl for SkillButton {
	fn gui_input(&mut self, event: Gd<InputEvent>) {
		let Ok(mouse_event) = event.try_cast::<InputEventMouseButton>()
		else { return };

		if mouse_event.is_echo()
			|| !mouse_event.is_pressed()
			|| mouse_event.get_button_index() != godot::global::MouseButton::LEFT
		{
			return;
		}

		match self.state {
			| State::ActorBase | State::Hover | State::Selected => {
				self.base_mut().emit_signal(Self::SIGNAL_CLICKED, &[]);
			}
			State::Disabled => {}
		}
	}

	fn ready(&mut self) {
		self.idle_idx = self.idle.get_index();
		self.idle_fx_idx = self.idle_fx.get_index();
		self.hover_idx = self.hover.get_index();
		self.hover_fx_idx = self.hover_fx.get_index();

		self.connect_with_deferred(&self.to_gd(), "mouse_entered", |this, _| {
			match this.state {
				State::ActorBase => {
					this.change_state(State::Hover);
				}
				| State::Hover | State::Selected | State::Disabled => {}
			}

			this.base_mut().emit_signal(Self::SIGNAL_ENTERED, &[]);
		});

		self.connect_with_deferred(&self.to_gd(), "mouse_exited", |this, _| {
			match this.state {
				| State::Hover => {
					this.change_state(State::ActorBase);
				}
				| State::ActorBase | State::Selected | State::Disabled => {}
			}

			this.base_mut().emit_signal(Self::SIGNAL_EXITED, &[]);
		});
	}
}

#[allow(unused)]
#[godot_api]
impl SkillButton {
	const SIGNAL_CLICKED: &'static str = "clicked";
	const SIGNAL_ENTERED: &'static str = "entered";
	const SIGNAL_EXITED: &'static str = "exited";

	#[signal]
	fn clicked() {}
	#[signal]
	fn entered() {}
	#[signal]
	fn exited() {}

	pub fn set_skill(&mut self, skill: impl SkillButtonSprites + 'static) {
		self.set_sprites(&skill);
		self.skill = Some(Box::new(skill));
	}

	fn set_sprites(&mut self, skill: &impl SkillButtonSprites) {
		self.background.set_texture(&skill.background());

		self.idle.set_texture(&skill.idle());
		self.hover.set_texture(&skill.hover());

		if let Some(texture) = skill.idle_fx() {
			self.idle_fx.set_texture(&texture);
			self.idle_fx.show();
		} else {
			self.idle_fx.hide();
		}

		if let Some(texture) = skill.hover_fx() {
			self.hover_fx.set_texture(&texture);
			self.hover_fx.show();
		} else {
			self.hover_fx.hide();
		}

		self.change_state(State::ActorBase);
	}

	fn change_state(&mut self, state: State) {
		self.anim_seq.take().map(|id| id.kill());
		self.state = state;

		match state {
			State::ActorBase => {
				self.anim_seq = Some(self.downlight());
				self.selected_indicator.hide();
			}
			State::Hover => {
				self.anim_seq = Some(self.highlight());
				self.selected_indicator.hide();
			}
			State::Selected => {
				self.anim_seq = Some(self.highlight());
				self.selected_indicator.show();
			}
			State::Disabled => {
				self.disable_light();
				self.selected_indicator.hide();
			}
		}
	}

	fn disable_light(&mut self) {
		[
			(&mut *self.background, DISABLED),
			(&mut *self.frame, DISABLED),
			(&mut *self.idle, DISABLED),
			(&mut *self.idle_fx, DISABLED),
			(&mut *self.hover, DISABLED_INACTIVE),
			(&mut *self.hover_fx, DISABLED_INACTIVE),
		]
		.into_iter()
		.for_each(|(rect, color)| {
			rect.set_modulate(color);
		});

		{
			let base = &mut self.base.to_gd();
			base.move_child(&*self.idle, self.hover_idx);
			base.move_child(&*self.idle_fx, self.hover_fx_idx);
			base.move_child(&*self.hover, self.idle_idx);
			base.move_child(&*self.hover_fx, self.idle_fx_idx);
		}
	}

	#[must_use]
	fn downlight(&mut self) -> SpireHandle<Sequence> {
		self.background.set_modulate(WHITE);
		self.frame.set_modulate(WHITE);

		{
			let base = &mut self.base.to_gd();
			base.move_child(&*self.idle, self.hover_idx);
			base.move_child(&*self.idle_fx, self.hover_fx_idx);
			base.move_child(&*self.hover, self.idle_idx);
			base.move_child(&*self.hover_fx, self.idle_fx_idx);
		}

		let mut seq = SpireSequence::new();

		set_white_ignore_a(&mut self.idle);
		seq.join(self.idle.do_fade(1.0, FADE_DUR));

		if self.idle_fx.is_visible() {
			set_white_ignore_a(&mut self.idle_fx);
			seq.join(self.idle_fx.do_fade(1.0, FADE_DUR));
		}

		set_white_ignore_a(&mut self.hover);
		seq.join(self.hover.do_fade(INACTIVE_A, FADE_DUR));

		if self.hover_fx.is_visible() {
			set_white_ignore_a(&mut self.hover_fx);
			seq.join(self.hover_fx.do_fade(INACTIVE_A, FADE_DUR));
		}

		seq.register()
	}

	#[must_use]
	fn highlight(&mut self) -> SpireHandle<Sequence> {
		self.background.set_modulate(WHITE);
		self.frame.set_modulate(WHITE);

		{
			let base = &mut self.base.to_gd();
			base.move_child(&*self.hover, self.hover_idx);
			base.move_child(&*self.hover_fx, self.hover_fx_idx);
			base.move_child(&*self.idle, self.idle_idx);
			base.move_child(&*self.idle_fx, self.idle_fx_idx);
		}

		let mut seq = SpireSequence::new();

		set_white_ignore_a(&mut self.idle);
		seq.join(self.idle.do_fade(INACTIVE_A, FADE_DUR));

		if self.idle_fx.is_visible() {
			set_white_ignore_a(&mut self.idle_fx);
			seq.join(self.idle_fx.do_fade(INACTIVE_A, FADE_DUR));
		}

		set_white_ignore_a(&mut self.hover);
		seq.join(self.hover.do_fade(1.0, FADE_DUR));

		if self.hover_fx.is_visible() {
			set_white_ignore_a(&mut self.hover_fx);
			seq.join(self.hover_fx.do_fade(1.0, FADE_DUR));
		}

		seq.register()
	}
}

const WHITE: Color = Color {
	r: 1.0,
	g: 1.0,
	b: 1.0,
	a: 1.0,
};
const INACTIVE_A: f64 = 0.15;
const DISABLED: Color = Color {
	r: 0.39216,
	g: 0.39216,
	b: 0.39216,
	a: 1.0,
};
const DISABLED_INACTIVE: Color = Color {
	r: 0.39216,
	g: 0.39216,
	b: 0.39216,
	a: INACTIVE_A as f32,
};
const FADE_DUR: f64 = 0.5;

fn set_white_ignore_a(rect: &mut Gd<TextureRect>) {
	let color = Color {
		a: rect.get_modulate().a,
		..WHITE
	};

	rect.set_modulate(color);
}
