#[allow(unused_imports)]
use crate::prelude::*;
pub use dynamic::*;
pub use range::*;
pub use rigid::*;

mod dynamic;
mod range;
mod rigid;

macro_rules! dynamic_stat {
    (struct $name: tt, $field: ty, $num: ty, $enum_variant: path) => {
	    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
	    pub struct $name {
		    inner: $field
	    }
	    
	    impl $name {
		    pub const fn new(value: $num) -> Self {
			    return Self { inner: <$field>::new(value) };
		    }
	    }
	    
	    impl Deref for $name {
		    type Target = $field;
		    
		    fn deref(&self) -> &Self::Target {
			    return &self.inner;
		    }
	    }
	    
	    impl DerefMut for $name {
		    fn deref_mut(&mut self) -> &mut Self::Target {
			    return &mut self.inner;
		    }
	    }
	    
	    impl DynamicStatTrait for $name {
		    type Inner = $field;
		    
		    fn stat_enum() -> DynamicStat {
			    return $enum_variant;
		    }
		    
		    fn from_i64(value: i64) -> Self {
			    return Self::new(value.squeeze_to());
		    }
	    }
	    
	    impl From<$num> for $name {
		    fn from(value: $num) -> Self {
			    return Self::new(value);
		    }
	    }
	    
	    impl From<isize> for $name {
		    fn from(value: isize) -> Self {
			    return Self::new(value.squeeze_to());
		    }
	    }
    };
}

use dynamic_stat;

macro_rules! rigid_stat {
    (struct $name: tt, $field: ty, $num: ty, $enum_variant: path) => {
	    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
		#[serde(transparent)]
	    #[repr(transparent)]
	    pub struct $name {
		    inner: $field
	    }
	    
	    impl $name {
		    pub const fn new(value: $num) -> Self {
			    return Self { inner: <$field>::new(value) };
		    }
	    }
	    
	    impl Deref for $name {
		    type Target = $field;
		    
		    fn deref(&self) -> &Self::Target {
			    return &self.inner;
		    }
	    }
	    
	    impl DerefMut for $name {
		    fn deref_mut(&mut self) -> &mut Self::Target {
			    return &mut self.inner;
		    }
	    }
	    
	    impl RigidStatTrait for $name {
		    type Inner = $field;
		    
		    fn stat_enum() -> RigidStat {
			    return $enum_variant;
		    }
	    }
    };
}

use rigid_stat;