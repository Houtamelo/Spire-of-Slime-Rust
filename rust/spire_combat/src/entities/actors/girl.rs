use slotmap::SlotMap;

use super::*;

#[derive(Serialize, Deserialize)]
pub struct Girl {
	stats: BaseGirlStats, // private because this is an implementation detail
	raw_stats: RawGirlStats,
	pub perks: HashMap<GirlPerkID, GirlPerk>,
	pub data: GirlDataEnum,
	pub statuses: HashMap<Id, GirlStatus>,
}

impl Girl {
	pub fn base_stat<Stat: GetGirlCommon + Clone>(&self) -> Stat {
		self.stats.get::<Stat>().clone()
	}

	pub fn base_stat_mut<Stat: GetGirlCommon>(&mut self) -> &mut Stat {
		self.stats.get_mut::<Stat>()
	}

	pub fn raw_stat<Stat: GetGirlRawCommon + Clone>(&self) -> Stat {
		self.raw_stats.get::<Stat>().clone()
	}

	pub fn raw_stat_mut<Stat: GetGirlRawCommon>(&mut self) -> &mut Stat {
		self.raw_stats.get_mut::<Stat>()
	}

	pub fn get_status<SE: IGirlStatusEffect + FromEnumRef<GirlStatus>>(&self) -> Option<&SE> {
		self.statuses
			.values()
			.find_map(|eff| eff.as_variant_ref::<SE>())
	}

	pub fn get_status_mut<SE: IGirlStatusEffect + FromEnumMut<GirlStatus>>(
		&mut self,
	) -> Option<&mut SE> {
		self.statuses
			.values_mut()
			.find_map(|eff| eff.as_variant_mut::<SE>())
	}

	pub fn add_status(&mut self, status: impl Into<GirlStatus>) {
		self.statuses.insert(Id::new(), status.into());
	}

	pub fn get_perk<P>(&self) -> Option<&P>
	where P: IGirlPerk + FromEnumRef<GirlPerk> {
		self.perks
			.values()
			.find_map(|perk| perk.as_variant_ref::<P>())
	}

	pub fn get_perk_mut<P>(&mut self) -> Option<&mut P>
	where P: IGirlPerk + FromEnumMut<GirlPerk> {
		self.perks
			.values_mut()
			.find_map(|perk| perk.as_variant_mut::<P>())
	}

	pub fn iter_perks(&self) -> impl Iterator<Item = &GirlPerk> { self.perks.values() }

	pub fn iter_perks_mut(&mut self) -> impl Iterator<Item = &mut GirlPerk> {
		self.perks.values_mut()
	}

	pub fn add_perk<P>(&mut self, perk: P)
	where P: IGirlPerk + Into<GirlPerk> + FromEnumRef<GirlPerk> {
		let perk = perk.into();
		self.perks.insert(perk.id(), perk);
	}
}
