#![feature(let_chains)]
#![feature(macro_metavar_expr)]
#![feature(result_flattening)]
#![feature(iter_from_coroutine)]
#![feature(coroutines)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(inline_const_pat)]
#![feature(const_option)]
#![feature(iterator_try_collect)]
#![feature(is_none_or)]
#![feature(trait_upcasting)]
#![feature(if_let_guard)]
#![feature(type_changing_struct_update)]
#![feature(min_specialization)]
#![feature(rustc_attrs)]
#![feature(arbitrary_self_types)]
#![feature(trait_alias)]
#![feature(option_get_or_insert_default)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(macro_metavar_expr_concat)]
#![feature(step_trait)]
#![feature(never_type)]
#![feature(try_blocks)]
#![feature(box_patterns)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(warnings)]
#![allow(clippy::absurd_extreme_comparisons)]
#![allow(clippy::derived_hash_with_manual_eq)]
#![allow(clippy::derive_ord_xor_partial_ord)]

mod entities;
mod graphics;
mod id;
mod macros;
mod skill_types;
mod state;
mod timeline;
mod util;

use internal_prelude::*;

pub mod prelude {
	pub use crate::internal_prelude::*;
}

#[allow(unused)]
mod internal_prelude {
	pub use std::{
		any::Any,
		borrow::Cow,
		cell::UnsafeCell,
		fmt::{Debug, Display, Formatter},
		iter,
		str::FromStr,
		sync::LazyLock,
	};

	pub use comfy_bounded_ints::prelude::*;
	pub use declarative_type_state::*;
	pub use derived_deref::{Deref, DerefMut};
	pub use discrimenum::{Hash as DiscriminantHash, PartialEq as DiscriminantPartialEq};
	pub use godot::{classes::*, prelude::*};
	pub use houtamelo_utils::prelude::*;
	pub use houtamelo_utils_gdext::prelude::*;
	pub use perfect_derive::perfect_derive;
	pub use rand::{
		distributions::uniform::SampleRange,
		prelude::{IteratorRandom, Rng, RngCore},
	};
	pub use rand_xoshiro::Xoshiro256PlusPlus;
	pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
	pub use shared::prelude::*;
	pub use slotmap::{HopSlotMap, SecondaryMap};
	pub use spire_tween::prelude::*;
	pub use strum_macros::{EnumCount, EnumString, FromRepr, VariantNames};
	pub use uuid::Uuid;

	pub(crate) use crate::macros::positions;
	pub use crate::{
		entities::*,
		graphics::*,
		id::*,
		skill_types::*,
		state::*,
		timeline::*,
		util::*,
	};
}
