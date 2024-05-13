#[allow(unused_imports)]
use crate::prelude::*;
use anyhow::bail;
use super::get_ref_or_bail;

pub struct CharacterStatsUI {
	root: Ref<Control>,
	
	name_label: Ref<Label>,
	portrait: Ref<TextureRect>,
	
	stamina_fill: Ref<Range>,
	stamina_label: Ref<Label>,
	
	dmg_label: Ref<Label>,

	power_label: Ref<Label>,
	acc_label: Ref<Label>,
	crit_rate_label: Ref<Label>,
	speed_label: Ref<Label>,
	dodge_label: Ref<Label>,

	toughness_label: Ref<Label>,
	stun_def_label: Ref<Label>,
	
	debuff_res_label: Ref<Label>,
	debuff_rate_label: Ref<Label>,
	
	move_res_label: Ref<Label>,
	move_rate_label: Ref<Label>,
	
	poison_res_label: Ref<Label>,
	poison_rate_label: Ref<Label>,
	
	temptation_fill: Ref<Range>,
	temptation_label: Ref<Label>,
	lust_low_fill: Ref<Range>,
	lust_high_fill: Ref<Range>,
	lust_label: Ref<Label>,
	composure_label: Ref<Label>,

	orgasm_parent: Ref<Control>,
	orgasm_toggles: [Ref<TextureRect>; 8],
	orgasm_texture_off: Ref<Texture>,
	orgasm_texture_on: Ref<Texture>,
}

impl CharacterStatsUI {
	pub fn new(root: &Control) -> Result<Self> {
		let name_label = get_ref_or_bail!(root, "name", Label)?;
		let portrait = get_ref_or_bail!(root, "portrait", TextureRect)?;
		
		let stamina_fill = get_ref_or_bail!(root, "stamina/fill", Range)?;
		let stamina_label = get_ref_or_bail!(root, "stamina/value", Label)?;
		
		let temptation_fill = get_ref_or_bail!(root, "temptation/fill", Range)?;
		let temptation_label = get_ref_or_bail!(root, "temptation/value", Label)?;
		
		let lust_low_fill = get_ref_or_bail!(root, "lust/low_fill", Range)?;
		let lust_high_fill = get_ref_or_bail!(root, "lust/high_fill", Range)?;
		let lust_label = get_ref_or_bail!(root, "lust/value", Label)?;
		
		let dmg_label = get_ref_or_bail!(root, "grid-stats/damage/value", Label)?;
		let power_label = get_ref_or_bail!(root, "grid-stats/power/value", Label)?;
		let acc_label = get_ref_or_bail!(root, "grid-stats/accuracy/value", Label)?;
		let crit_rate_label = get_ref_or_bail!(root, "grid-stats/crit-rate/value", Label)?;
		let speed_label = get_ref_or_bail!(root, "grid-stats/speed/value", Label)?;
		let dodge_label = get_ref_or_bail!(root, "grid-stats/dodge/value", Label)?;
		
		let toughness_label = get_ref_or_bail!(root, "grid-stats/toughness/value", Label)?;
		let composure_label = get_ref_or_bail!(root, "grid-stats/composure/value", Label)?;
		let stun_def_label = get_ref_or_bail!(root, "grid-stats/stun-mitigation/value", Label)?;
		
		let debuff_res_label = get_ref_or_bail!(root, "grid-stats/debuff-resistance/value", Label)?;
		let debuff_rate_label = get_ref_or_bail!(root, "grid-stats/debuff-rate/value", Label)?;
		
		let move_res_label = get_ref_or_bail!(root, "grid-stats/move-resistance/value", Label)?;
		let move_rate_label = get_ref_or_bail!(root, "grid-stats/move-rate/value", Label)?;
		
		let poison_res_label = get_ref_or_bail!(root, "grid-stats/poison-resistance/value", Label)?;
		let poison_rate_label = get_ref_or_bail!(root, "grid-stats/poison-rate/value", Label)?;

		let orgasm_parent_tref = unsafe {
			root.get_node_as::<Control>("orgasm-counter")
			    .ok_or_else(|| anyhow!("Failed to orgasm-counter from {}", root.name()))
		}?;
		
		let orgasm_parent = unsafe { orgasm_parent_tref.assume_shared() };

		let orgasm_toggles = {
			let children = orgasm_parent_tref.get_children();
			if children.len() != 8 {
				bail!("Expected 8 children in orgasm-counter, found {}", children.len());
			}
			
			[
				children.get(0).try_to_object()?,
				children.get(1).try_to_object()?,
				children.get(2).try_to_object()?,
				children.get(3).try_to_object()?,
				children.get(4).try_to_object()?,
				children.get(5).try_to_object()?,
				children.get(6).try_to_object()?,
				children.get(7).try_to_object()?,
			]
		};
		
		let orgasm_texture_off = load_resource_as::<Texture>(
			"res://Core/Combat/UI/Character Stats/Orgasm Counter/orgasm_off.png")?;
		
		let orgasm_texture_on = load_resource_as::<Texture>(
			"res://Core/Combat/UI/Character Stats/Orgasm Counter/orgasm_on.png")?;
		
		let root = unsafe { root.assume_shared() };
		
		let _self = Self {
			root,
			
			name_label,
			portrait,
			
			stamina_fill,
			stamina_label,
			
			temptation_fill,
			temptation_label,
			
			lust_low_fill,
			lust_high_fill,
			lust_label,

			dmg_label,
			power_label,
			acc_label,
			crit_rate_label,
			speed_label,
			dodge_label,
			
			toughness_label,
			composure_label,
			stun_def_label,
			
			debuff_res_label,
			debuff_rate_label,
			
			move_res_label,
			move_rate_label,
			
			poison_res_label,
			poison_rate_label,

			orgasm_parent,
			orgasm_toggles,
			orgasm_texture_off,
			orgasm_texture_on,
		};
		
		return Ok(_self);
	}
	
	pub fn set_character(&self, character: &CombatCharacter) {
		macro_rules! set_fill {
		    ($var: ident, $fill_expr: expr) => {
			    self.$var.touch_assert_sane(|fill| 
			        fill.set_value($fill_expr));
		    };
		}

		macro_rules! set_label {
		    ($var: ident, $label_expr: expr) => {
			    self.$var.touch_assert_sane(|label| 
			        label.set_text($label_expr));
		    };
		}

		macro_rules! set_parent_visible {
		    ($var: ident, $is_visible: ident) => {
			    self.$var
					.touch_assert_sane(|label|
						label.get_parent_control()
						     .touch_assert_sane(|parent|
							     parent.set_visible($is_visible)));
		    };
		}
		
		self.name_label.touch_assert_sane(|label| {
			label.set_text(character.data.variant().display_name());
		});
		
		self.portrait.touch_assert_sane(|portrait| {
			if let Ok(texture) = character.data.variant().combat_portrait() {
				portrait.set_texture(texture);
			}
		});
		
		//todo! set_name, set_portrait

		let stamina_cur = character.stamina_cur;
		let stamina_max = character.max_stamina();
		let stamina_percent = (stamina_cur.get() as f64 / stamina_max.get() as f64) * 100.0;

		set_fill!(stamina_fill, stamina_percent);
		set_label!(stamina_label, format!("{} / {}", stamina_cur.get(), stamina_max.get()));
		
		let dmg = character.dmg;
		set_label!(dmg_label, format!("{}~{}", dmg.bound_lower(), dmg.bound_upper()));
		
		set_label!(power_label, format!("{}", character.dyn_stat::<Power>().get()));
		
		let acc = character.dyn_stat::<Accuracy>().get();
		if acc > 0 {
			set_label!(acc_label, format!("+{}", acc));
		} else {
			set_label!(acc_label, format!("{}", acc));
		}
		
		let crit = character.dyn_stat::<CritRate>().get();
		if crit > 0 {
			set_label!(crit_rate_label, format!("+{}", crit));
		} else {
			set_label!(crit_rate_label, format!("{}", crit));
		}
		
		set_label!(speed_label, format!("{}", character.dyn_stat::<Speed>().get()));
		
		let dodge = character.dyn_stat::<Dodge>().get();
		if dodge > 0 {
			set_label!(dodge_label, format!("+{}", dodge));
		} else {
			set_label!(dodge_label, format!("{}", dodge));
		}
		
		set_label!(toughness_label, format!("{}", character.dyn_stat::<Toughness>().get()));
		set_label!(stun_def_label, format!("{}", character.dyn_stat::<StunDef>().get()));
		
		set_label!(debuff_res_label, format!("{}", character.dyn_stat::<DebuffRes>().get()));
		set_label!(debuff_rate_label, format!("{}", character.dyn_stat::<DebuffRate>().get()));
		
		set_label!(move_res_label, format!("{}", character.dyn_stat::<MoveRes>().get()));
		set_label!(move_rate_label, format!("{}", character.dyn_stat::<MoveRate>().get()));
		
		set_label!(poison_res_label, format!("{}", character.dyn_stat::<PoisonRes>().get()));
		set_label!(poison_rate_label, format!("{}", character.dyn_stat::<PoisonRate>().get()));

		if let Some(girl_stats) = &character.girl_stats {
			let temptation = girl_stats.temptation;
			let temptation_percent = (temptation.get() as f64 / 100.0) * 100.0;

			set_fill!(temptation_fill, temptation_percent);
			set_label!(temptation_label, format!("{} / 100", temptation.get()));

			let lust_cur = girl_stats.lust;
			match lust_cur.get() {
				0..=100 => {
					let low_fill_percent = (lust_cur.get() as f64 / 100.0) * 100.0;
					set_fill!(lust_low_fill, low_fill_percent);
					set_fill!(lust_high_fill, 0.0);
					set_label!(lust_label, format!("{} / 100", lust_cur.get()));
				}
				101.. => {
					let high_fill_percent = ((lust_cur.get() - 100) as f64 / 100.0) * 100.0;
					set_fill!(lust_low_fill, 100.0);
					set_fill!(lust_high_fill, high_fill_percent);
					set_label!(lust_label, format!("{} / 100", lust_cur.get()));
				}
			}

			let composure = character.dyn_stat::<Composure>();
			set_label!(composure_label, format!("{}", composure.get()));

			let orgasm_limit = girl_stats.orgasm_limit.get() as usize;
			let orgasm_count = girl_stats.orgasm_count.get() as usize;
			
			for i in 0..orgasm_count {
				self.orgasm_toggles[i]
					.touch_assert_sane(|toggle| {
						toggle.set_texture(&self.orgasm_texture_on);
						toggle.set_visible(true);
					});
			}
			
			for i in orgasm_count..orgasm_limit {
				self.orgasm_toggles[i]
					.touch_assert_sane(|toggle| {
						toggle.set_texture(&self.orgasm_texture_off);
						toggle.set_visible(true);
					});
			}
			
			for i in orgasm_limit..8 {
				self.orgasm_toggles[i]
					.touch_assert_sane(|toggle|
						toggle.set_visible(false));
			}
		}
		let is_girl = character.girl_stats.is_some();
		set_parent_visible!(temptation_label, is_girl);
		set_parent_visible!(lust_low_fill, is_girl);
		set_parent_visible!(composure_label, is_girl);
		self.orgasm_parent
		    .touch_assert_sane(|parent|
			    parent.set_visible(is_girl));
	}
	
	pub fn hide(&self) {
		self.root
			.touch_assert_sane(|node| {
				node.hide();
			});
	}
}
