use crate::*;
use super::randomizer::*;

pub const GROVE_NODE: BGTree =
	BGTree::new(
		None, &[
			("SW_03", BGTree::new(mode!(SW{50}), &[
				("ME_04", BGTree::end(mode!(ME))),
				("SW_11_lights", BGTree::end(mode!(SW{50})))
			])),
			("SW_06_close-hills", BGTree::end(mode!(SW{50}))),
			("SW_07", BGTree::new(mode!(SW{50}), &[
				("SW_09", BGTree::new(mode!(SW{50}), &[
					("ME_10", BGTree::end(mode!(ME)))
				]))
			])),
			("SW_14", BGTree::new(mode!(SW{50}), &[
				("SW_15_old-bridge", BGTree::end(mode!(SW{50})))
			])),
			("PP_18", BGTree::new(mode!(PP{60}), &[
				("bush_0", children!(&[
					"SW_roses", mode!(SW{50})
				])),
				("bush_1", children!(&[
					"SW_roses", mode!(SW{50})
				])),
				("bush_2", children!(&[
					"SW_roses", mode!(SW{50})
				])),
				("bush_3", children!(&[
					"SW_roses", mode!(SW{50})
				])),
				("tree-middle", children!(&[
					"SW_19_leaves-middle", mode!(SW{50})
				])),
			])),
			("PP_20", BGTree::new(mode!(PP{60}), &[
				("tree-middle", children!(&[
					"SW_21_vine_1", mode!(SW{50}),
					"SW_21_vine_2", mode!(SW{50}),
				])),
				("tree-right", children!(&[
					"SW_21_vine_3", mode!(SW{50}),
					"SW_21_vine_4", mode!(SW{50}),
				])),
				("ME_21A", BGTree::new(mode!(ME), &[
					("bush_5", children!(&[
						"SW_roses", mode!(SW{50})
					]))
				])),
				("ME_21B", BGTree::new(mode!(ME), &[
					("dead-tree_middle", children!(&[
						"SW_web", mode!(SW{50})
					]))
				])),
				("ME_21C", BGTree::new(mode!(ME), &[
					("bush_4", children!(&[
						"SW_roses", mode!(SW{50})
					]))
				])),
				("ME_21C", BGTree::end(mode!(ME)))
			])),
			("SW_25", BGTree::end(mode!(SW{50}))),
			("PP_26", BGTree::new(mode!(PP{60}), &[
				("bush_1", children!(&[
					"SW_roses", mode!(SW{50})
				])),
				("bush_2", children!(&[
					"SW_roses", mode!(SW{50})
				])),
				("bush_3", children!(&[
					"SW_roses", mode!(SW{50})
				]))
			])),
			("PP_27", BGTree::new(mode!(PP{60}), &[
				("tree-left", children!(&[
					"SW_leaves", mode!(SW{50}),
					"SW_slime", mode!(SW{50}),
					"SW_web", mode!(SW{50}),
					"SW_vine_1", mode!(SW{50}),
					"SW_vine_2", mode!(SW{50}),
				])),
				("tree-right", children!(&[
					"SW_leaves", mode!(SW{50}),
					"SW_vine_3", mode!(SW{50}),
					"SW_vine_4", mode!(SW{50}),
				]))
			])),
			("PP_28", BGTree::new(mode!(PP{60}), &[
				("ME_right-plants", BGTree::end(mode!(ME))),
				("plant_1", children!(&[
					"SW_slime", mode!(SW{50}),
				])),
				("plant_3", children!(&[
					"SW_slime", mode!(SW{50}),
				])),
			])),
		]);