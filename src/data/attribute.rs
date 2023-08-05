use crate::bin_io::read::BinRead;
use crate::bin_io::write::BinWrite;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Error, ErrorKind, Read, Write};

const INDEX_CODE: u32 = 0;
const TYPE_CODE: u32 = 1;
const SORT_CODE: u32 = 2;
const TECH_LEVEL_CODE: u32 = 3;
const MINIMUM_PRICE_SYSTEM_CODE: u32 = 4;
const MAXIMUM_PRICE_SYSTEM_CODE: u32 = 5;
const OCCURRENCE_CODE: u32 = 6;
const MINIMUM_PRICE_CODE: u32 = 7;
const MAXIMUM_PRICE_CODE: u32 = 8;
const DAMAGE_CODE: u32 = 9;
const EMP_DAMAGE_CODE: u32 = 10;
const LOADING_SPEED_CODE: u32 = 11;
const SPEED_FACTOR_CODE: u32 = 13;
const SHIELD_CAPACITY_CODE: u32 = 18;
const SHIELD_LOADING_SPEED_CODE: u32 = 19;
const ARMOUR_CODE: u32 = 20;
const COMPRESSOR_EFFECT_CODE: u32 = 22;
const TRACTOR_BEAM_AUTOMATIC_CODE: u32 = 23;
const TRACTOR_BEAM_LOCK_TIME_CODE: u32 = 24;
const BOOST_EFFECT_CODE: u32 = 25;
const BOOST_LOADING_SPEED_CODE: u32 = 26;
const BOOST_DURATION_CODE: u32 = 27;
const THRUSTER_EFFECT_CODE: u32 = 28;
const SCANNER_LOCK_TIME_CODE: u32 = 29;
const SCANNER_SHOW_CLASS_A_ASTEROIDS_CODE: u32 = 30;
const SCANNER_SHOW_CARGO_CODE: u32 = 31;
const MINING_HANDLING_CODE: u32 = 32;
const MINING_YIELD_CODE: u32 = 33;
const CABIN_SIZE_CODE: u32 = 34;
const CLOAK_EFFECT_CODE: u32 = 35;
const CLOAK_LOADING_SPEED_CODE: u32 = 36;
const CLOAK_ENERGY_CONSUMPTION_CODE: u32 = 38;
const TIME_EXTENDER_EFFECT_CODE: u32 = 42;
const TIME_EXTENDER_LOADING_SPEED_CODE: u32 = 43;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Attribute {
    Index,
    Type,
    Sort,
    TechLevel,
    MinimumPriceSystem,
    MaximumPriceSystem,
    Occurrence,
    MinimumPrice,
    MaximumPrice,
    Damage,
    EmpDamage,
    LoadingSpeed,
    SpeedFactor,
    ShieldCapacity,
    ShieldLoadingSpeed,
    Armour,
    CompressorEffect,
    TractorBeamAutomatic,
    TractorBeamLockTime,
    BoostEffect,
    BoostLoadingSpeed,
    BoostDuration,
    ThrusterEffect,
    ScannerLockTime,
    ScannerShowClassAAsteroids,
    ScannerShowCargo,
    MiningHandling,
    MiningYield,
    CabinSize,
    CloakEffect,
    CloakLoadingSpeed,
    CloakEnergyConsumption,
    TimeExtenderEffect,
    TimeExtenderLoadingSpeed,
}

impl TryFrom<u32> for Attribute {
    type Error = Error;

    fn try_from(value: u32) -> io::Result<Self> {
        match value {
            INDEX_CODE => Ok(Self::Index),
            TYPE_CODE => Ok(Self::Type),
            SORT_CODE => Ok(Self::Sort),
            TECH_LEVEL_CODE => Ok(Self::TechLevel),
            MINIMUM_PRICE_SYSTEM_CODE => Ok(Self::MinimumPriceSystem),
            MAXIMUM_PRICE_SYSTEM_CODE => Ok(Self::MaximumPriceSystem),
            OCCURRENCE_CODE => Ok(Self::Occurrence),
            MINIMUM_PRICE_CODE => Ok(Self::MinimumPrice),
            MAXIMUM_PRICE_CODE => Ok(Self::MaximumPrice),
            DAMAGE_CODE => Ok(Self::Damage),
            EMP_DAMAGE_CODE => Ok(Self::EmpDamage),
            LOADING_SPEED_CODE => Ok(Self::LoadingSpeed),
            SPEED_FACTOR_CODE => Ok(Self::SpeedFactor),
            SHIELD_CAPACITY_CODE => Ok(Self::ShieldCapacity),
            SHIELD_LOADING_SPEED_CODE => Ok(Self::ShieldLoadingSpeed),
            ARMOUR_CODE => Ok(Self::Armour),
            COMPRESSOR_EFFECT_CODE => Ok(Self::CompressorEffect),
            TRACTOR_BEAM_AUTOMATIC_CODE => Ok(Self::TractorBeamAutomatic),
            TRACTOR_BEAM_LOCK_TIME_CODE => Ok(Self::TractorBeamLockTime),
            BOOST_EFFECT_CODE => Ok(Self::BoostEffect),
            BOOST_LOADING_SPEED_CODE => Ok(Self::BoostLoadingSpeed),
            BOOST_DURATION_CODE => Ok(Self::BoostDuration),
            THRUSTER_EFFECT_CODE => Ok(Self::ThrusterEffect),
            SCANNER_LOCK_TIME_CODE => Ok(Self::ScannerLockTime),
            SCANNER_SHOW_CLASS_A_ASTEROIDS_CODE => Ok(Self::ScannerShowClassAAsteroids),
            SCANNER_SHOW_CARGO_CODE => Ok(Self::ScannerShowCargo),
            MINING_HANDLING_CODE => Ok(Self::MiningHandling),
            MINING_YIELD_CODE => Ok(Self::MiningYield),
            CABIN_SIZE_CODE => Ok(Self::CabinSize),
            CLOAK_EFFECT_CODE => Ok(Self::CloakEffect),
            CLOAK_LOADING_SPEED_CODE => Ok(Self::CloakLoadingSpeed),
            CLOAK_ENERGY_CONSUMPTION_CODE => Ok(Self::CloakEnergyConsumption),
            TIME_EXTENDER_EFFECT_CODE => Ok(Self::TimeExtenderEffect),
            TIME_EXTENDER_LOADING_SPEED_CODE => Ok(Self::TimeExtenderLoadingSpeed),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid attribute")),
        }
    }
}

impl From<Attribute> for u32 {
    fn from(value: Attribute) -> Self {
        match value {
            Attribute::Index => INDEX_CODE,
            Attribute::Type => TYPE_CODE,
            Attribute::Sort => SORT_CODE,
            Attribute::TechLevel => TECH_LEVEL_CODE,
            Attribute::MinimumPriceSystem => MINIMUM_PRICE_SYSTEM_CODE,
            Attribute::MaximumPriceSystem => MAXIMUM_PRICE_SYSTEM_CODE,
            Attribute::Occurrence => OCCURRENCE_CODE,
            Attribute::MinimumPrice => MINIMUM_PRICE_CODE,
            Attribute::MaximumPrice => MAXIMUM_PRICE_CODE,
            Attribute::Damage => DAMAGE_CODE,
            Attribute::EmpDamage => EMP_DAMAGE_CODE,
            Attribute::LoadingSpeed => LOADING_SPEED_CODE,
            Attribute::SpeedFactor => SPEED_FACTOR_CODE,
            Attribute::ShieldCapacity => SHIELD_CAPACITY_CODE,
            Attribute::ShieldLoadingSpeed => SHIELD_LOADING_SPEED_CODE,
            Attribute::Armour => ARMOUR_CODE,
            Attribute::CompressorEffect => COMPRESSOR_EFFECT_CODE,
            Attribute::TractorBeamAutomatic => TRACTOR_BEAM_AUTOMATIC_CODE,
            Attribute::TractorBeamLockTime => TRACTOR_BEAM_LOCK_TIME_CODE,
            Attribute::BoostEffect => BOOST_EFFECT_CODE,
            Attribute::BoostLoadingSpeed => BOOST_LOADING_SPEED_CODE,
            Attribute::BoostDuration => BOOST_DURATION_CODE,
            Attribute::ThrusterEffect => THRUSTER_EFFECT_CODE,
            Attribute::ScannerLockTime => SCANNER_LOCK_TIME_CODE,
            Attribute::ScannerShowClassAAsteroids => SCANNER_SHOW_CLASS_A_ASTEROIDS_CODE,
            Attribute::ScannerShowCargo => SCANNER_SHOW_CARGO_CODE,
            Attribute::MiningHandling => MINING_HANDLING_CODE,
            Attribute::MiningYield => MINING_YIELD_CODE,
            Attribute::CabinSize => CABIN_SIZE_CODE,
            Attribute::CloakEffect => CLOAK_EFFECT_CODE,
            Attribute::CloakLoadingSpeed => CLOAK_LOADING_SPEED_CODE,
            Attribute::CloakEnergyConsumption => CLOAK_ENERGY_CONSUMPTION_CODE,
            Attribute::TimeExtenderEffect => TIME_EXTENDER_EFFECT_CODE,
            Attribute::TimeExtenderLoadingSpeed => TIME_EXTENDER_LOADING_SPEED_CODE,
        }
    }
}

impl BinRead for Attribute {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> io::Result<Self> {
        source.read_u32::<O>()?.try_into()
    }
}

impl BinWrite for Attribute {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> io::Result<()> {
        destination.write_u32::<O>((*self).into())
    }
}
