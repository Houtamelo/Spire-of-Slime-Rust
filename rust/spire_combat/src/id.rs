use super::*;

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Id(Uuid);

impl Id {
	pub fn new() -> Self { Self(Uuid::new_v4()) }
}

impl From<&Actor> for Id {
	fn from(actor: &Actor) -> Self { actor.id }
}

impl From<&Ptr<Actor>> for Id {
	fn from(actor: &Ptr<Actor>) -> Self { actor.id }
}

impl Display for Id {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}
