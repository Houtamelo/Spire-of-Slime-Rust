use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MapLocation {
	Chapel,
	Grove,
	Forest,
	Cave,
}