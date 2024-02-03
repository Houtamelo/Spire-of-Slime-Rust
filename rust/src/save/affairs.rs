use serde::{Deserialize, Serialize};

pub trait AffairTrait { 
	type State;
	fn get_from_map(map: &AffairMap) -> Self::State;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffairMap {
	RescuedByMistressTender: RescuedByMistressTender,
}

impl AffairMap {
	pub fn get<T: AffairTrait>(&self) -> T::State {
		return T::get_from_map(self);
	}
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Happened {
	#[default] No,
	Yes,
}


macro_rules! affair_type {
    (struct $name: ident, $state: ty) => {
	    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
		pub struct $name {
			state: $state,
		}
	    
	    impl $name {
		    pub fn state(&self) -> &$state { return &self.state; }
	    }

		impl AffairTrait for $name {
			type State = $state;

			fn get_from_map(map: &AffairMap) -> Self::State { return map.$name.state; }
		}
    };
}

affair_type!(struct RescuedByMistressTender, Happened);
