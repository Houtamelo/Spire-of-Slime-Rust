#[allow(unused_imports)]
use crate::*;
use combat::prelude::*;
use crate::save::upgrades::{PrimaryUpgrade, PrimaryUpgradeCount, SecondaryUpgrade, SecondaryUpgradeCount};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenericStats {
	rng: Xoshiro256PlusPlus,
	total_exp: u64,
	dmg: CheckedRange,
	stamina: MaxStamina,
	toughness: Toughness,
	stun_def: StunDef,
	debuff_res: DebuffRes,
	debuff_rate: DebuffRate,
	move_res: MoveRes,
	move_rate: MoveRate,
	poison_res: PoisonRes,
	poison_rate: PoisonRate,
	speed: Speed,
	accuracy: Accuracy,
	crit: CritRate,
	dodge: Dodge,
	lust: Lust,
	temptation: Temptation,
	composure: Composure,
	corruption: Bound_u8<0, 100>,
	orgasm_limit: OrgasmLimit,
	orgasm_count: OrgasmCount,
	primary_upgrades: PrimaryUpgradeCount,
	secondary_upgrades: SecondaryUpgradeCount,
	available_points_primary: u8,
	available_points_secondary: u8,
	next_upgrade_options_primary: Option<Vec<PrimaryUpgrade>>,
	next_upgrade_options_secondary: Option<Vec<SecondaryUpgrade>>,
	available_points_perk: u8,
	sexual_exp: HashMap<NPCVariant, u16>,
}

impl GenericStats {
	pub fn from_data(rng: Xoshiro256PlusPlus, data: &(impl CharacterData + GirlData)) -> GenericStats {
		return GenericStats {
			rng,
			total_exp: 0,
			dmg: data.dmg(0),
			stamina: data.max_stamina(0),
			toughness: data.toughness(0),
			stun_def: data.stun_def(0),
			debuff_res: data.debuff_res(0),
			debuff_rate: data.debuff_rate(0),
			move_res: data.move_res(0),
			move_rate: data.move_rate(0),
			poison_res: data.poison_res(0),
			poison_rate: data.poison_rate(0),
			speed: data.spd(0),
			accuracy: data.acc(0),
			crit: data.crit(0),
			dodge: data.dodge(0),
			lust: Lust::new(0),
			temptation: Temptation::new(0),
			composure: data.composure(),
			corruption: 0.into(),
			orgasm_limit: data.orgasm_limit(),
			orgasm_count: OrgasmCount::new(0),
			primary_upgrades: PrimaryUpgradeCount::default(),
			secondary_upgrades: SecondaryUpgradeCount::default(),
			available_points_primary: 0,
			available_points_secondary: 0,
			next_upgrade_options_primary: None,
			next_upgrade_options_secondary: None,
			available_points_perk: 0,
			sexual_exp: HashMap::new(),
		};
	}
}