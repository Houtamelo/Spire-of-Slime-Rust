use crate::*;
use crate::graphics::action_animation::skills::anim_utils::TryGetNode;

macro_rules! children {
    (&[$($child_path: literal, $child_mode: expr),*$(,)?]) => {
	    BGTree { 
		    rng_mode: None,
		    randomized_children: &[
			    $(($child_path, BGTree::end($child_mode))),*
	        ]
	    }
    };
}

pub(in super) use children;

macro_rules! mode {
    (SW { $chance: literal }) => {
	    Some(RngMode::Switch { on_chance: PercentageU8::new($chance) })
    };
	(PP { $max_percent: literal }) => {
	    Some(RngMode::Props { max_percent: PercentageU8::new($max_percent) })
	};
	(ME) => {
	    Some(RngMode::MutuallyExclusive)
	};
}

pub(in super) use mode;
use crate::graphics::stages::serialization::{SerializedBGTree, SerializedRngMode};

impl BGTree {
	pub(in super) const fn new(mode: Option<RngMode>, children: &'static[(&'static str, BGTree)]) -> Self {
		Self { rng_mode: mode, randomized_children: children }
	}

	pub(in super) const fn end(mode: Option<RngMode>) -> Self {
		Self { rng_mode: mode, randomized_children: &[] }
	}
}

pub(in super) enum RngMode {
	Switch { on_chance: PercentageU8 },
	Props { max_percent: PercentageU8 },
	MutuallyExclusive,
}

pub(in super) struct BGTree {
	pub rng_mode: Option<RngMode>,
	pub randomized_children: &'static[(&'static str, BGTree)],
}

impl BGTree {
	pub(in super) unsafe fn randomize_recursive(&self, rng: &mut impl Rng, name: &str, parent: &Node2D) -> Result<SerializedBGTree> {
		let self_node = parent.try_get_node::<Node2D>(name)?;
		
		let Some(mode) = &self.rng_mode
			else {
				let randomized_children =
					self.randomized_children
					    .iter()
					    .try_fold(vec![], |mut sum, (child_name, bg_node)| {
						    let child_result = bg_node.randomize_recursive(rng, child_name, &self_node)?;
						    sum.push((child_name.to_string(), child_result));
						    Result::<_>::Ok(sum)
					    })?;
				
				return Ok(SerializedBGTree { rng_mode: None, randomized_children })
			};
		
		let result = mode.randomize(rng, &self_node)?;
		match result {
			SerializedRngMode::Switch { on } => {
				let randomized_children =
					if on {
						self.randomized_children
							.iter()
							.try_fold(vec![], |mut sum, (child_name, bg_node)| {
								let child_result= bg_node.randomize_recursive(rng, child_name, &self_node)?;
								sum.push((child_name.to_string(), child_result));
								Result::<_>::Ok(sum)
							})?
					} else {
						vec![]
					};
				
				Ok(SerializedBGTree { rng_mode: Some(SerializedRngMode::Switch { on }), randomized_children })
			}
			SerializedRngMode::Props { chosens } => {
				let randomized_children =
					self.randomized_children
					    .iter()
					    .filter(|(child_name, _)| {
						    chosens.iter().any(|name| name.as_str() == *child_name)
					    })
						.try_fold(vec![], |mut sum, (child_name, bg_node)| {
							let child_result = bg_node.randomize_recursive(rng, child_name, &self_node)?;
							sum.push((child_name.to_string(), child_result));
							Result::<_>::Ok(sum)
						})?;
				
				Ok(SerializedBGTree { rng_mode: Some(SerializedRngMode::Props { chosens }), randomized_children })
			}
			SerializedRngMode::MutuallyExclusive { chosen } => {
				if let Some((child_name, bg_node)) = 
					self.randomized_children
						.iter()
						.find(|(child_name, _)| *child_name == chosen.as_str()) {
					let randomized_children = vec![(child_name.to_string(), bg_node.randomize_recursive(rng, child_name, &self_node)?)];
					Ok(SerializedBGTree { rng_mode: Some(SerializedRngMode::MutuallyExclusive { chosen }), randomized_children })
				} else {
					Ok(SerializedBGTree { rng_mode: Some(SerializedRngMode::MutuallyExclusive { chosen }), randomized_children: vec![] })
				}
			}
		}
	}
}

impl RngMode {
	unsafe fn randomize(&self, rng: &mut impl Rng, node: &Node2D) -> Result<SerializedRngMode> {
		match self {
			RngMode::Switch { on_chance } => {
				let on = rng.gen_ratio(on_chance.get() as u32, 100);
				node.set_visible(on);
				Ok(SerializedRngMode::Switch { on })
			}
			RngMode::Props { max_percent } => {
				let percent_0_100 = rng.gen_range(0..max_percent.get());
				let child_count = node.get_child_count() as usize;
				let visible_count = (child_count * percent_0_100 as usize) / 100;
				
				let candidates = (0..child_count).collect::<Vec<_>>();
				let chosens_idx = (0..visible_count).scan(candidates, |candidates, _| {
					candidates.take_random(rng)
				}).collect::<Vec<_>>();
				
				let chosens =
					node.get_children()
					    .iter()
					    .enumerate()
					    .filter_map(|(idx, child)| {
						    child.try_to_object::<Node2D>()
						         .map_err(|err| anyhow!(
									 "Failed to cast child at index {idx} to Node2D.\
									  Error: {err}"))
						         .map(|child_node| {
							         let t_ref = child_node.assume_safe();
							         if chosens_idx.contains(&idx) {
								         t_ref.show();
								         Some(t_ref.name().to_string())
							         } else {
								         None
							         }
						         }).transpose()
					    }).try_collect::<Vec<_>>()?;
				
				Ok(SerializedRngMode::Props { chosens })
			}
			RngMode::MutuallyExclusive => {
				let child_count = node.get_child_count() as usize;
				let chosen_idx = rng.gen_range(0..child_count);
				
				let chosen = 
					node.get_child(chosen_idx as i64)
						.and_then(|child| child.assume_safe().cast::<Node2D>())
						.ok_or_else(|| anyhow!("Failed to cast child at index {chosen_idx} to Node2D."))?;
				
				chosen.show();
				
				Ok(SerializedRngMode::MutuallyExclusive { chosen: chosen.name().to_string() })
			}
		}
	}
}