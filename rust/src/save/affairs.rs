#[allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};

pub trait Affair { 
	type State;
	fn get(map: &AffairMap) -> Self::State;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffairMap {
	RescuedByMistressTender: RescuedByMistressTender,
}

impl AffairMap {
	pub fn get<T: Affair>(&self) -> T::State {
		return T::get(self);
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

		impl Affair for $name {
			type State = $state;

			fn get(map: &AffairMap) -> Self::State { return map.$name.state; }
		}
    };
}

affair_type!(struct RescuedByMistressTender, Happened);
