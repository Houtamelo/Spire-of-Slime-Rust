use super::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum SkillIdent {
	Nema(NemaSkill),
	Ethel(EthelSkill),
	Crabdra(CrabdraSkill),
	BellPlant(BellPlantSkill),
}

impl FromStr for SkillIdent {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		if let Ok(ok) = NemaSkill::from_str(s) {
			return Ok(Self::Nema(ok));
		}

		if let Ok(ok) = EthelSkill::from_str(s) {
			return Ok(Self::Ethel(ok));
		}

		if let Ok(ok) = CrabdraSkill::from_str(s) {
			return Ok(Self::Crabdra(ok));
		}

		if let Ok(ok) = BellPlantSkill::from_str(s) {
			return Ok(Self::BellPlant(ok));
		}

		Err(anyhow!("Invalid SkillName: {s}"))
	}
}
