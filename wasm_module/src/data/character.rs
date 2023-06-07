#![allow(non_snake_case)]
use super::{
    affix::Affix,
    attribute::{Attribute, ATTRIBUTE_COUNT},
    settings::Settings,
};
use serde::Serialize;

pub trait AttributesArray {
    fn get_a(&self, attr: Attribute) -> f32;
    fn set_a(&mut self, attr: Attribute, value: f32);
    fn add_a(&mut self, attr: Attribute, value: f32);
}

pub type Attributes = [f32; ATTRIBUTE_COUNT];
impl AttributesArray for Attributes {
    #[inline(always)]
    fn get_a(&self, attr: Attribute) -> f32 {
        self[attr as usize]
    }

    #[inline(always)]
    fn set_a(&mut self, attr: Attribute, value: f32) {
        self[attr as usize] = value;
    }

    #[inline(always)]
    fn add_a(&mut self, attr: Attribute, value: f32) {
        self[attr as usize] += value;
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
#[repr(align(64))] // cache line alignment
pub struct Character {
    // attributes indexed by Attribute enum
    // array instead of struct since this is muuuuuuuuch faster than matching with enum
    #[serde(with = "serde_arrays")]
    pub base_attributes: Attributes,
    #[serde(with = "serde_arrays")]
    pub attributes: Attributes,
    pub rankby: Attribute,
    pub gear: [Affix; 14],
    pub combination_id: u32,
}
mod serde_arrays {
    use serde::Serialize;

    use super::ATTRIBUTE_COUNT;

    pub fn serialize<S, T>(array: &[T; ATTRIBUTE_COUNT], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: Serialize,
    {
        array[..].serialize(serializer)
    }
}

impl Character {
    pub fn new(rankby: Attribute) -> Self {
        Character {
            base_attributes: [0.0; ATTRIBUTE_COUNT],
            attributes: [0.0; ATTRIBUTE_COUNT],
            rankby,
            gear: [Affix::None; 14],
            combination_id: std::u32::MAX,
        }
    }

    pub fn clear(&mut self) {
        self.base_attributes.iter_mut().for_each(|attr| {
            *attr = 0.0;
        });

        self.attributes.iter_mut().for_each(|attr| {
            *attr = 0.0;
        });
    }

    pub fn score(&self) -> f32 {
        return self.attributes.get_a(self.rankby);
    }

    pub fn is_invalid(&self, settings: &Settings) -> bool {
        let invalid = (settings.minBoonDuration.is_some()
            && self.attributes.get_a(Attribute::BoonDuration)
                < settings.minBoonDuration.unwrap() / 100.0)
            || (settings.minQuicknessDuration.is_some()
                && self.attributes.get_a(Attribute::BoonDuration)
                    + self.attributes.get_a(Attribute::QuicknessDuration)
                    < settings.minQuicknessDuration.unwrap() / 100.0)
            || (settings.minHealingPower.is_some()
                && self.attributes.get_a(Attribute::HealingPower)
                    < settings.minHealingPower.unwrap())
            || (settings.minToughness.is_some()
                && self.attributes.get_a(Attribute::Toughness) < settings.minToughness.unwrap())
            || (settings.maxToughness.is_some()
                && self.attributes.get_a(Attribute::Toughness) > settings.maxToughness.unwrap())
            || (settings.minHealth.is_some()
                && self.attributes.get_a(Attribute::Health) < settings.minHealth.unwrap())
            || (settings.minCritChance.is_some()
                && self.attributes.get_a(Attribute::CriticalChance)
                    < settings.minCritChance.unwrap() / 100.0)
            || (settings.minOutgoingHealing.is_some()
                && self.attributes.get_a(Attribute::OutgoingHealing)
                    < settings.minOutgoingHealing.unwrap() / 100.0);

        invalid
    }
}

#[derive(Debug, Clone, Serialize)]
#[repr(align(64))] // cache line alignment
pub struct ResultCharacter {
    // attributes indexed by Attribute enum
    // array instead of struct since this is muuuuuuuuch faster than matching with enum
    #[serde(with = "serde_arrays")]
    pub base_attributes: Attributes,
    #[serde(with = "serde_arrays")]
    pub attributes: Attributes,

    pub gear: [Affix; 14],
    pub gear_stats: [f32; 10],
    pub combination_id: u32,

    pub results: Results,
}
impl ResultCharacter {
    pub fn to_character(&self) -> Character {
        Character {
            base_attributes: self.base_attributes,
            attributes: self.attributes,
            rankby: Attribute::Power,
            gear: self.gear,
            combination_id: self.combination_id,
        }
    }
    pub fn from(character: &Character) -> Self {
        ResultCharacter {
            base_attributes: character.base_attributes,
            attributes: character.attributes,
            gear: character.gear,
            gear_stats: [0.0; 10],
            combination_id: character.combination_id,
            results: Results {
                value: character.score(),
                coefficientHelper: CoefficientHelper {
                    Bleeding: CoefficientDetails {
                        slope: 0.0,
                        intercept: 0.0,
                    },
                    Burning: CoefficientDetails {
                        slope: 0.0,
                        intercept: 0.0,
                    },
                    Confusion: CoefficientDetails {
                        slope: 0.0,
                        intercept: 0.0,
                    },
                    Poison: CoefficientDetails {
                        slope: 0.0,
                        intercept: 0.0,
                    },
                    Power: CoefficientDetails {
                        slope: 0.0,
                        intercept: 0.0,
                    },
                    Power2: CoefficientDetails {
                        slope: 0.0,
                        intercept: 0.0,
                    },
                    Torment: CoefficientDetails {
                        slope: 0.0,
                        intercept: 0.0,
                    },
                },
                damageBreakdown: [0.0; 7],
                effectiveDamageDistribution: [0.0; 7],
                effectiveNegativeValues: [0.0; 5],
                effectivePositiveValues: [0.0; 5],
            },
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Results {
    pub coefficientHelper: CoefficientHelper,
    pub damageBreakdown: [f32; 7],
    pub effectiveDamageDistribution: [f32; 7],
    pub effectiveNegativeValues: [f32; 5],
    pub effectivePositiveValues: [f32; 5],
    pub value: f32,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CoefficientHelper {
    pub Bleeding: CoefficientDetails,
    pub Burning: CoefficientDetails,
    pub Confusion: CoefficientDetails,
    pub Poison: CoefficientDetails,
    pub Power: CoefficientDetails,
    pub Power2: CoefficientDetails,
    pub Torment: CoefficientDetails,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CoefficientDetails {
    pub slope: f32,
    pub intercept: f32,
}
