use super::*;

// Get -----------------------------------------------------------------------------------------------------------------
pub trait GetStatus<Enum> {
	fn get_status<SE: ValidStatus<Enum>>(&self) -> Option<&SE>;
	fn get_status_mut<SE: ValidStatus<Enum>>(&mut self) -> Option<&mut SE>;
}

impl<Enum> GetStatus<Enum> for Actor {
	fn get_status<SE: ValidStatus<Enum>>(&self) -> Option<&SE> { SE::find_in(self) }
	fn get_status_mut<SE: ValidStatus<Enum>>(&mut self) -> Option<&mut SE> { SE::find_in_mut(self) }
}

// Map -----------------------------------------------------------------------------------------------------------------
pub trait MapStatus<Ret, Enum> {
	fn map_status<SE: ValidStatus<Enum>>(&self, f: impl FnOnce(&SE) -> Ret) -> Option<Ret>;
	fn map_status_mut<SE: ValidStatus<Enum>>(
		&mut self,
		f: impl FnOnce(&mut SE) -> Ret,
	) -> Option<Ret>;
}

impl<Ret, Enum> MapStatus<Ret, Enum> for Actor {
	fn map_status<SE: ValidStatus<Enum>>(&self, f: impl FnOnce(&SE) -> Ret) -> Option<Ret> {
		SE::find_in(self).map(f)
	}
	fn map_status_mut<SE: ValidStatus<Enum>>(
		&mut self,
		f: impl FnOnce(&mut SE) -> Ret,
	) -> Option<Ret> {
		SE::find_in_mut(self).map(f)
	}
}

// Has -----------------------------------------------------------------------------------------------------------------
pub trait HasStatus<Enum> {
	fn has_status<SE: ValidStatus<Enum>>(&self) -> bool;
}

impl<Enum> HasStatus<Enum> for Actor {
	fn has_status<SE: ValidStatus<Enum>>(&self) -> bool { SE::find_in(self).is_some() }
}

// Impl ----------------------------------------------------------------------------------------------------------------
trait ValidStatus<Enum> {
	fn find_in(actor: &Actor) -> Option<&Self>;
	fn find_in_mut(actor: &mut Actor) -> Option<&mut Self>;
}

impl<SE: IStatusEffect + FromEnumRef<StatusEffect> + FromEnumMut<StatusEffect>>
	ValidStatus<StatusEffect> for SE
{
	fn find_in(actor: &Actor) -> Option<&Self> { actor.base.get_status() }

	fn find_in_mut(actor: &mut Actor) -> Option<&mut Self> { actor.base.get_status_mut() }
}

impl<SE: IGirlStatusEffect + FromEnumRef<GirlStatus> + FromEnumMut<GirlStatus>>
	ValidStatus<GirlStatus> for SE
{
	fn find_in(actor: &Actor) -> Option<&Self> {
		actor.girl.as_ref().and_then(|girl| girl.get_status())
	}

	fn find_in_mut(actor: &mut Actor) -> Option<&mut Self> {
		actor.girl.as_mut().and_then(|girl| girl.get_status_mut())
	}
}
