use super::*;

newtype_table! {
	ENUM: {
		#[vars(derive(Default, Clone, Serialize, Deserialize, PartialEq, Eq))]
		pub enum Affair {
			RescuedByMistressTender(Happened),
		}
	}

	TABLE: {
		#[derive(Default, Clone, Serialize, Deserialize)]
		pub struct AffairMap;
	}
}

impl AffairMap {
	#[allow(unused)] //todo!
	pub fn is_fulfilled<T: MemberOf<AffairMap, MemberType: PartialEq + Eq>>(
		&self,
		condition: &T::MemberType,
	) -> bool {
		self.get::<T>() == condition
	}
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Happened {
	#[default]
	No,
	Yes,
}
