use super::*;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct ActorStatsUI {
	base: Base<Control>,

	#[init(node = "name")]
	name_label: OnReady<Gd<Label>>,
	#[init(node = "portrait")]
	portrait:   OnReady<Gd<TextureRect>>,

	#[init(node = "stamina/fill")]
	stamina_fill:  OnReady<Gd<Range>>,
	#[init(node = "stamina/value")]
	stamina_label: OnReady<Gd<Label>>,

	#[init(node = "grid-stats/damage/value")]
	dmg_label: OnReady<Gd<Label>>,
	#[init(node = "grid-stats/power/value")]
	power_label: OnReady<Gd<Label>>,
	#[init(node = "grid-stats/accuracy/value")]
	acc_label: OnReady<Gd<Label>>,
	#[init(node = "grid-stats/crit-rate/value")]
	crit_rate_label: OnReady<Gd<Label>>,
	#[init(node = "grid-stats/speed/value")]
	speed_label: OnReady<Gd<Label>>,
	#[init(node = "grid-stats/dodge/value")]
	dodge_label: OnReady<Gd<Label>>,

	#[init(node = "grid-stats/toughness/value")]
	toughness_label: OnReady<Gd<Label>>,
	#[init(node = "grid-stats/stun-mitigation/value")]
	stun_def_label:  OnReady<Gd<Label>>,

	#[init(node = "grid-stats/debuff-resistance/value")]
	debuff_res_label:  OnReady<Gd<Label>>,
	#[init(node = "grid-stats/debuff-rate/value")]
	debuff_rate_label: OnReady<Gd<Label>>,

	#[init(node = "grid-stats/move-resistance/value")]
	move_res_label:  OnReady<Gd<Label>>,
	#[init(node = "grid-stats/move-rate/value")]
	move_rate_label: OnReady<Gd<Label>>,

	#[init(node = "grid-stats/poison-resistance/value")]
	poison_res_label:  OnReady<Gd<Label>>,
	#[init(node = "grid-stats/poison-rate/value")]
	poison_rate_label: OnReady<Gd<Label>>,

	#[init(node = "temptation/fill")]
	temptation_fill: OnReady<Gd<Range>>,
	#[init(node = "temptation/value")]
	temptation_label: OnReady<Gd<Label>>,
	#[init(node = "lust/low_fill")]
	lust_low_fill: OnReady<Gd<Range>>,
	#[init(node = "lust/high_fill")]
	lust_high_fill: OnReady<Gd<Range>>,
	#[init(node = "lust/value")]
	lust_label: OnReady<Gd<Label>>,
	#[init(node = "grid-stats/composure/value")]
	composure_label: OnReady<Gd<Label>>,

	#[init(node = "orgasm-counter")]
	orgasm_parent:  OnReady<Gd<Control>>,
	#[export]
	orgasm_toggles: Array<Gd<TextureRect>>,

	#[init(default = OnReady::new(|| load_resource_as::<Texture2D>(
		"res://Core/Combat/UI/Character Stats/Orgasm Counter/orgasm_off.png"
	).unwrap()))]
	orgasm_texture_off: OnReady<Gd<Texture2D>>,
	#[init(default = OnReady::new(|| load_resource_as::<Texture2D>(
		"res://Core/Combat/UI/Character Stats/Orgasm Counter/orgasm_on.png"
	).unwrap()))]
	orgasm_texture_on:  OnReady<Gd<Texture2D>>,
}

impl ActorStatsUI {
	pub fn set_actor<T>(&mut self, actor: &Ptr<Actor>, ctx: &ActorContext) {
		self.name_label.set_text(actor.name.display_name());
		self.portrait.set_texture(
			&actor
				.name
				.combat_portrait()
				.map_err(|e| godot_warn!("{e:?}"))
				.unwrap_or_default(),
		);

		let stamina_cur = actor.raw_stat::<CurrentStamina>();
		let stamina_max = actor.eval_dyn_stat::<MaxStamina>(ctx);
		let stamina_percent = (*stamina_cur as f64 / *stamina_max as f64) * 100.0;
		self.stamina_fill.set_value(stamina_percent);
		self.stamina_label
			.set_text(&format!("{} / {}", stamina_cur, stamina_max));

		let dmg_range = actor.base_stat::<Damage>();
		self.dmg_label
			.set_text(&format!("{}~{}", dmg_range.lower(), dmg_range.upper()));

		self.power_label
			.set_text(&format!("{}", actor.eval_dyn_stat::<Power>(ctx)));

		self.acc_label
			.set_text(&actor.eval_dyn_stat::<Accuracy>(ctx).display_sign());
		self.crit_rate_label
			.set_text(&actor.eval_dyn_stat::<CritRate>(ctx).display_sign());

		self.speed_label
			.set_text(&format!("{}", actor.eval_dyn_stat::<Speed>(ctx)));
		self.dodge_label
			.set_text(&format!("{}", actor.eval_dyn_stat::<Dodge>(ctx)));
		self.toughness_label
			.set_text(&format!("{}", actor.eval_dyn_stat::<Toughness>(ctx)));
		self.stun_def_label
			.set_text(&format!("{}", actor.eval_dyn_stat::<StunDef>(ctx)));
		self.debuff_res_label
			.set_text(&format!("{}", actor.eval_dyn_stat::<DebuffRes>(ctx)));
		self.debuff_rate_label
			.set_text(&format!("{}", actor.eval_dyn_stat::<DebuffRate>(ctx)));
		self.move_res_label
			.set_text(&format!("{}", actor.eval_dyn_stat::<MoveRes>(ctx)));
		self.move_rate_label
			.set_text(&format!("{}", actor.eval_dyn_stat::<MoveRate>(ctx)));
		self.poison_res_label
			.set_text(&format!("{}", actor.eval_dyn_stat::<PoisonRes>(ctx)));
		self.poison_rate_label
			.set_text(&format!("{}", actor.eval_dyn_stat::<PoisonRate>(ctx)));

		if let Some(girl) = &actor.girl {
			let temptation = girl.raw_stat::<Temptation>();
			let temptation_percent = (temptation / 100.0) * 100.0;
			self.temptation_fill.set_value(temptation_percent);
			self.temptation_label
				.set_text(&format!("{} / 100", temptation));

			let lust_cur = girl.raw_stat::<Lust>();
			match *lust_cur {
				..=100 => {
					let low_fill_percent = (lust_cur / 100.0) * 100.0;
					self.lust_low_fill.set_value(low_fill_percent);
					self.lust_high_fill.set_value(0.0);
					self.lust_label.set_text(&format!("{} / 100", lust_cur));
				}
				101.. => {
					let high_fill_percent = ((lust_cur - 100.0) / 100.0) * 100.0;
					self.lust_low_fill.set_value(100.0);
					self.lust_high_fill.set_value(high_fill_percent);
					self.lust_label.set_text(&format!("{} / 100", lust_cur));
				}
			}

			let composure = actor.eval_dyn_girl_stat::<Composure>(girl, ctx);
			self.composure_label.set_text(&format!("{}", composure));

			let orgasm_count = *girl.raw_stat::<OrgasmCount>();

			for i in 0..orgasm_count {
				let idx = cram::<usize>(i);

				if let Some(mut toggle) = self.orgasm_toggles.get(idx) {
					toggle.set_texture(&*self.orgasm_texture_on);
					toggle.set_visible(true);
				} else {
					godot_warn!("Insufficient orgasm toggles! Expected at least {idx}");
				}
			}

			let orgasm_limit = *actor.eval_dyn_girl_stat::<OrgasmLimit>(girl, ctx);
			for i in orgasm_count..orgasm_limit {
				let idx = cram::<usize>(i);

				if let Some(mut toggle) = self.orgasm_toggles.get(idx) {
					toggle.set_texture(&*self.orgasm_texture_off);
					toggle.set_visible(true);
				} else {
					godot_warn!("Insufficient orgasm toggles! Expected at least {idx}");
				}
			}

			for i in orgasm_limit..self.orgasm_toggles.len() as i64 {
				let idx = cram::<usize>(i);

				if let Some(mut toggle) = self.orgasm_toggles.get(idx) {
					toggle.set_visible(false);
				} else {
					godot_warn!(
						"Array::get(i) returned none despite index being lower than `len`. Index: {idx}"
					);
				}
			}
		}

		let is_girl = actor.girl.is_some();

		macro_rules! set_parent_visible {
			($var:ident) => {
				self.$var
					.get_parent()
					.ok_or_else(|| anyhow!("{} has no parent", stringify!($var)))
					.and_then(|node| {
						node.try_cast::<CanvasItem>().map_err(|_| {
							anyhow!("{}'s parent does not inherit CanvasItem.", stringify!($var))
						})
					})
					.map(|mut node| node.set_visible(is_girl))
					.log_if_err();
			};
		}

		set_parent_visible!(temptation_label);
		set_parent_visible!(lust_low_fill);
		set_parent_visible!(composure_label);
		set_parent_visible!(orgasm_parent);
	}
}
