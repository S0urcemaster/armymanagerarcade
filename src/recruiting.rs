#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoldierType {
    Infantry,
    Archer,
    Rider,
}

impl SoldierType {
    pub fn label(self) -> &'static str {
        match self {
            Self::Infantry => "Infantry",
            Self::Archer => "Archer",
            Self::Rider => "Rider",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Composition {
    Infantry,
    Archers,
    Riders,
    Mixed,
}

impl Composition {
    pub fn label(self) -> &'static str {
        match self {
            Self::Infantry => "Infantry",
            Self::Archers => "Archers",
            Self::Riders => "Riders",
            Self::Mixed => "Mixed",
        }
    }

    fn pure(soldier_type: SoldierType) -> Self {
        match soldier_type {
            SoldierType::Infantry => Self::Infantry,
            SoldierType::Archer => Self::Archers,
            SoldierType::Rider => Self::Riders,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmourQuality {
    Improvised,
    Standard,
    Reinforced,
    Veteran,
    Royal,
}

impl ArmourQuality {
    pub fn from_level(level: u8) -> Self {
        match level {
            1 => Self::Improvised,
            2 => Self::Standard,
            3 => Self::Reinforced,
            4 => Self::Veteran,
            _ => Self::Royal,
        }
    }

    pub fn level(self) -> u8 {
        self as u8 + 1
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Improvised => "Improvised",
            Self::Standard => "Standard",
            Self::Reinforced => "Reinforced",
            Self::Veteran => "Veteran",
            Self::Royal => "Royal",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OriginProfession {
    Farmer,
    Hunter,
    StableHand,
    Smith,
    Guard,
    Sailor,
    Trader,
    Shepherd,
}

impl OriginProfession {
    pub fn label(self) -> &'static str {
        match self {
            Self::Farmer => "Farmer",
            Self::Hunter => "Hunter",
            Self::StableHand => "Stable hand",
            Self::Smith => "Smith",
            Self::Guard => "Town guard",
            Self::Sailor => "Sailor",
            Self::Trader => "Trader",
            Self::Shepherd => "Shepherd",
        }
    }

    fn affinity(self) -> SoldierType {
        match self {
            Self::Hunter | Self::Shepherd => SoldierType::Archer,
            Self::StableHand | Self::Trader => SoldierType::Rider,
            Self::Farmer | Self::Smith | Self::Guard | Self::Sailor => SoldierType::Infantry,
        }
    }

    fn modifiers(self) -> (i16, i16, i16, u8, &'static str, &'static str) {
        match self {
            Self::Farmer => (8, 4, 0, 0, "Enduring", "Untrained"),
            Self::Hunter => (5, 3, 4, 2, "Keen eye", "Light armour"),
            Self::StableHand => (6, 2, 2, 1, "Horsewise", "Poor formation"),
            Self::Smith => (7, 0, 1, 0, "Equipment care", "Slow march"),
            Self::Guard => (4, 6, 1, 1, "Formation drill", "Rigid"),
            Self::Sailor => (7, 3, 2, 1, "Sure-footed", "No cavalry sense"),
            Self::Trader => (1, 2, 8, 3, "Well connected", "Low endurance"),
            Self::Shepherd => (6, 1, 3, 2, "Patient scout", "Light build"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecruitOffer {
    pub leader_name: String,
    pub profession: OriginProfession,
    pub preferred_type: SoldierType,
    pub composition: Composition,
    pub armour_type: SoldierType,
    pub group_size: u32,
    pub talent: u8,
    pub fitness: u8,
    pub belonging: u8,
    pub armour_quality: ArmourQuality,
    pub armour_durability: u8,
    pub intel: u8,
    pub personal_gold: u32,
    pub hire_cost: u32,
    pub bonus: &'static str,
    pub malus: &'static str,
}

impl RecruitOffer {
    pub fn is_group(&self) -> bool {
        self.group_size > 1
    }
}

const FIRST_NAMES: &[&str] = &[
    "Aelius", "Amon", "Cassian", "Darius", "Drusus", "Hanno", "Ilyas", "Leontes", "Lucan", "Mago",
    "Nikanor", "Orestes", "Sabin", "Samir", "Tarek", "Varro",
];

const FAMILY_NAMES: &[&str] = &[
    "Afer", "Barca", "Corvin", "Damon", "Elian", "Faro", "Galen", "Hasdrin", "Kasson", "Marcell",
    "Nerva", "Orontes", "Rufus", "Severin", "Timon", "Zeno",
];

const PROFESSIONS: &[OriginProfession] = &[
    OriginProfession::Farmer,
    OriginProfession::Hunter,
    OriginProfession::StableHand,
    OriginProfession::Smith,
    OriginProfession::Guard,
    OriginProfession::Sailor,
    OriginProfession::Trader,
    OriginProfession::Shepherd,
];

const SOLDIER_TYPES: &[SoldierType] = &[
    SoldierType::Infantry,
    SoldierType::Archer,
    SoldierType::Rider,
];

struct Roller(u64);

impl Roller {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D049BB133111EB);
        (value ^ (value >> 31)) as u32
    }

    fn range(&mut self, min: u32, max: u32) -> u32 {
        min + self.next() % (max - min + 1)
    }
}

fn group_size(stage: usize, roller: &mut Roller) -> u32 {
    match stage {
        0..=3 => 1,
        4..=8 => roller.range(2, 8),
        9..=15 => roller.range(10, 40),
        16..=25 => roller.range(50, 200),
        26..=40 => roller.range(250, 1_000),
        41..=60 => roller.range(1_500, 5_000),
        _ => roller.range(10_000, 50_000),
    }
}

fn bounded(base: u32, modifier: i16) -> u8 {
    (base as i16 + modifier).clamp(1, 100) as u8
}

pub fn generate_offer(campaign_seed: u64, stage: usize, slot: usize) -> RecruitOffer {
    let seed = campaign_seed
        ^ (stage as u64).wrapping_mul(0xD6E8FEB86659FD93)
        ^ (slot as u64).wrapping_mul(0xA5A3564E27F8862D);
    let mut roller = Roller::new(seed);
    let first = FIRST_NAMES[roller.range(0, FIRST_NAMES.len() as u32 - 1) as usize];
    let family = FAMILY_NAMES[roller.range(0, FAMILY_NAMES.len() as u32 - 1) as usize];
    let profession = PROFESSIONS[roller.range(0, PROFESSIONS.len() as u32 - 1) as usize];
    let (fitness_mod, belonging_mod, gold_mod, intel_mod, bonus, malus) = profession.modifiers();
    let group_size = group_size(stage, &mut roller);
    let talent = roller.range(25, 90) as u8;
    let fitness = bounded(roller.range(38, 78), fitness_mod);
    let belonging = bounded(
        roller.range(35, 75),
        belonging_mod + (group_size > 1) as i16 * 8,
    );
    let armour_level = (roller.range(1, 4)
        + matches!(
            profession,
            OriginProfession::Smith | OriginProfession::Guard
        ) as u32)
        .min(5) as u8;
    let armour_quality = ArmourQuality::from_level(armour_level);
    let armour_durability = roller.range(45, 100) as u8;
    let armour_type = SOLDIER_TYPES[roller.range(0, SOLDIER_TYPES.len() as u32 - 1) as usize];
    let preferred_type = profession.affinity();
    let composition = if group_size == 1 || roller.range(0, 9) >= 8 {
        Composition::pure(preferred_type)
    } else {
        Composition::Mixed
    };
    let intel = (roller.range(0, 2) as u8 + intel_mod).min(5);
    let personal_gold_per_member = (roller.range(1, 12) as i16 + gold_mod).max(0) as u32;
    let personal_gold = personal_gold_per_member * group_size;
    let individual_cost = 18 + talent as u32 / 4 + armour_level as u32 * 9 + intel as u32 * 4;
    let hire_cost = individual_cost * group_size;

    RecruitOffer {
        leader_name: format!("{first} {family}"),
        profession,
        preferred_type,
        composition,
        armour_type,
        group_size,
        talent,
        fitness,
        belonging,
        armour_quality,
        armour_durability,
        intel,
        personal_gold,
        hire_cost,
        bonus,
        malus,
    }
}

pub fn generate_offers(campaign_seed: u64, stage: usize, count: usize) -> Vec<RecruitOffer> {
    (0..count)
        .map(|slot| generate_offer(campaign_seed, stage, slot))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offers_are_stable_for_a_campaign_stage_and_slot() {
        assert_eq!(generate_offer(17, 4, 2), generate_offer(17, 4, 2));
        assert_ne!(generate_offer(17, 4, 2), generate_offer(17, 4, 3));
    }

    #[test]
    fn recruitment_scales_from_people_to_large_groups() {
        assert_eq!(generate_offer(17, 1, 0).group_size, 1);
        assert!(generate_offer(17, 12, 0).group_size >= 10);
        assert!(generate_offer(17, 65, 0).group_size >= 10_000);
    }

    #[test]
    fn armour_has_five_levels_and_is_independent_from_aptitude() {
        let offers = generate_offers(17, 6, 32);
        assert!(
            offers
                .iter()
                .all(|offer| (1..=5).contains(&offer.armour_quality.level()))
        );
        assert!(
            offers
                .iter()
                .any(|offer| offer.armour_type != offer.preferred_type)
        );
    }

    #[test]
    fn groups_are_usually_mixed_and_cost_the_sum_of_members() {
        let offers = generate_offers(19, 6, 100);
        let mixed = offers
            .iter()
            .filter(|offer| offer.composition == Composition::Mixed)
            .count();
        assert!(mixed >= 70);
        for offer in offers {
            assert_eq!(offer.hire_cost % offer.group_size, 0);
        }
    }
}
