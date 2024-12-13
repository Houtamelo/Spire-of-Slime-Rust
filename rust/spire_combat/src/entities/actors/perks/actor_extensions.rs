use super::*;

// Get -----------------------------------------------------------------------------------------------------------------
pub trait GetPerk<Enum> {
	fn get_perk<Perk: ValidPerk<Enum>>(&self) -> Option<&Perk>;
	fn get_perk_mut<Perk: ValidPerk<Enum>>(&mut self) -> Option<&mut Perk>;
}

impl<Enum> GetPerk<Enum> for Actor {
	fn get_perk<Perk: ValidPerk<Enum>>(&self) -> Option<&Perk> { Perk::find_in(self) }
	fn get_perk_mut<Perk: ValidPerk<Enum>>(&mut self) -> Option<&mut Perk> {
		Perk::find_in_mut(self)
	}
}

// Map -----------------------------------------------------------------------------------------------------------------
pub trait MapPerk<Ret, Enum> {
	fn map_perk<Perk: ValidPerk<Enum>>(&self, f: impl FnOnce(&Perk) -> Ret) -> Option<Ret>;
	fn map_perk_mut<Perk: ValidPerk<Enum>>(
		&mut self,
		f: impl FnOnce(&mut Perk) -> Ret,
	) -> Option<Ret>;
}

impl<Ret, Enum> MapPerk<Ret, Enum> for Actor {
	fn map_perk<Perk: ValidPerk<Enum>>(&self, f: impl FnOnce(&Perk) -> Ret) -> Option<Ret> {
		Perk::find_in(self).map(f)
	}
	fn map_perk_mut<Perk: ValidPerk<Enum>>(
		&mut self,
		f: impl FnOnce(&mut Perk) -> Ret,
	) -> Option<Ret> {
		Perk::find_in_mut(self).map(f)
	}
}

// Has -----------------------------------------------------------------------------------------------------------------
pub trait HasPerk<Enum> {
	fn has_perk<Perk: ValidPerk<Enum>>(&self) -> bool;
}

impl<Enum> HasPerk<Enum> for Actor {
	fn has_perk<Perk: ValidPerk<Enum>>(&self) -> bool { Perk::find_in(self).is_some() }
}

// Impl ----------------------------------------------------------------------------------------------------------------
trait ValidPerk<Enum> {
	fn find_in(actor: &Actor) -> Option<&Self>;
	fn find_in_mut(actor: &mut Actor) -> Option<&mut Self>;
}

impl<P: IPerk + FromEnumRef<Perk> + FromEnumMut<Perk>> ValidPerk<Perk> for P {
	fn find_in(actor: &Actor) -> Option<&Self> { actor.base.get_perk() }
	fn find_in_mut(actor: &mut Actor) -> Option<&mut Self> { actor.base.get_perk_mut() }
}

impl<P: IGirlPerk + FromEnumRef<GirlPerk> + FromEnumMut<GirlPerk>> ValidPerk<GirlPerk> for P {
	fn find_in(actor: &Actor) -> Option<&Self> {
		actor.girl.as_ref().and_then(|girl| girl.get_perk())
	}
	fn find_in_mut(actor: &mut Actor) -> Option<&mut Self> {
		actor.girl.as_mut().and_then(|girl| girl.get_perk_mut())
	}
}
