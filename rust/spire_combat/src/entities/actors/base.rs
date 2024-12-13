use slotmap::SlotMap;

use super::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct ActorBase {
	stats: BaseStats,
	raw_stats: RawStats,
	pub perks: HashMap<PerkID, Perk>,
	pub statuses: HashMap<Id, StatusEffect>,
	pub id: Id,
	pub name: ActorName,
	pub last_damager: Option<Id>,
	pub stun_redundancy_ms: Option<Int>,
	pub skill_use_counters: HashMap<SkillIdent, u16>,
	pub on_zero_stamina: OnZeroStamina,
	pub team: Team,
}

impl ActorBase {
	pub fn base_stat<Stat: GetBaseCommon + Clone>(&self) -> Stat {
		self.stats.get::<Stat>().clone()
	}

	pub fn base_stat_mut<Stat: GetBaseCommon>(&mut self) -> &mut Stat { self.stats.get_mut() }

	pub fn raw_stat<Stat: GetRawCommon + Clone>(&self) -> Stat {
		self.raw_stats.get::<Stat>().clone()
	}

	pub fn raw_stat_mut<Stat: GetRawCommon>(&mut self) -> &mut Stat { self.raw_stats.get_mut() }

	pub fn get_status<SE: IStatusEffect + FromEnumRef<StatusEffect>>(&self) -> Option<&SE> {
		self.statuses
			.values()
			.find_map(|eff| eff.as_variant_ref::<SE>())
	}

	pub fn get_status_mut<SE: IStatusEffect + FromEnumMut<StatusEffect>>(
		&mut self,
	) -> Option<&mut SE> {
		self.statuses
			.values_mut()
			.find_map(|eff| eff.as_variant_mut::<SE>())
	}

	pub fn add_status(&mut self, status: impl Into<StatusEffect>) {
		self.statuses.insert(Id::new(), status.into());
	}

	pub fn get_perk<P: IPerk + FromEnumRef<Perk>>(&self) -> Option<&P> {
		self.perks
			.values()
			.find_map(|perk| perk.as_variant_ref::<P>())
	}

	pub fn get_perk_mut<P: IPerk + FromEnumMut<Perk>>(&mut self) -> Option<&mut P> {
		self.perks
			.values_mut()
			.find_map(|perk| perk.as_variant_mut::<P>())
	}

	pub fn iter_perks(&self) -> impl Iterator<Item = &Perk> { self.perks.values() }
	pub fn iter_perks_mut(&mut self) -> impl Iterator<Item = &mut Perk> { self.perks.values_mut() }

	pub fn add_perk<P: IPerk + Into<Perk> + FromEnumRef<Perk>>(&mut self, perk: P) {
		let perk = perk.into();
		self.perks.insert(perk.id(), perk);
	}

	pub fn is_affected_by(&self, additive: PoisonAdditive) -> bool {
		self.statuses.values().any(|eff| {
			eff.as_variant_ref::<Poison>()
				.is_some_and(|p| p.additives.contains(&additive))
		})
	}
}
