use super::*;

declarative_type_state::type_table! {
	ENUM_OUT: {
		#[vars(derive(Default, Clone, Serialize, Deserialize, PartialEq, Eq))]
		pub enum Affair {
			RescuedByMistressTender(bool),
		}
	}

	TABLE: {
		#[derive(Default, Clone, Serialize, Deserialize)]
		pub struct AffairMap;
	}
}

impl AffairMap {
	#[allow(unused)] //todo!
	pub fn is_fulfilled<T: GetInTable + PartialEq + Eq>(&self, condition: &T) -> bool {
		self.get::<T>() == condition
	}
}
