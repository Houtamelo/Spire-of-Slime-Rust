use super::*;

mod base;
mod effects;
mod girl;
mod name;
mod perks;
mod ptr;
mod state;
mod stats;

pub use base::*;
pub use effects::*;
pub use girl::*;
pub use name::*;
pub use perks::*;
pub use ptr::*;
pub use state::*;
pub use stats::*;

#[derive(Serialize, Deserialize, Deref, DerefMut, Clone)]
pub struct Actor {
	#[target]
	pub base: ActorBase,
	#[serde(
		serialize_with = "serialize_girl",
		deserialize_with = "deserialize_girl"
	)]
	pub girl: Option<Ptr<Girl>>,
}

fn serialize_girl<S: Serializer>(
	girl_opt: &Option<Ptr<Girl>>,
	serializer: S,
) -> std::result::Result<S::Ok, S::Error> {
	match girl_opt {
		Some(girl_ptr) => {
			let girl: &Girl = &**girl_ptr;
			serializer.serialize_some(girl)
		}
		None => serializer.serialize_none(),
	}
}

fn deserialize_girl<'de, D: Deserializer<'de>>(
	deserializer: D,
) -> std::result::Result<Option<Ptr<Girl>>, D::Error> {
	Option::<Girl>::deserialize(deserializer).map(|girl_opt| girl_opt.map(Ptr::new))
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum OnZeroStamina {
	Vanish,
	Corpse,
	Downed,
}
