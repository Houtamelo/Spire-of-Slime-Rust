use super::*;

mod actors;
mod context;
mod data;
mod position;
mod skill_intention;
mod stat;

pub use actors::*;
pub use context::*;
pub use data::*;
pub use position::*;
pub use skill_intention::*;
pub use stat::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Race {
	Human,
	Plant,
	Mutation,
}
