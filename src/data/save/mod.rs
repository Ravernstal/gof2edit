use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::data::save::agent::SaveAgent;
use crate::data::save::blueprint::SaveBlueprint;
use crate::data::save::inventory_item::SaveInventoryItem;
use crate::data::save::kaamo_status::KaamoStatus;
use crate::data::save::medal::Medal;
use crate::data::save::mission::SaveMission;
use crate::data::save::pending_product::SavePendingProduct;
use crate::data::save::ship::SaveShip;
use crate::data::save::ship_equipment::SaveShipEquipment;
use crate::data::save::station::SaveStation;
use crate::data::save::unknown_structure_1::UnknownStructure1;
use crate::data::save::unknown_structure_2::UnknownStructure2;
use crate::data::save::wanted::SaveWanted;
use crate::result::Result;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io;
use std::io::{Read, Write};

mod agent;
mod blueprint;
mod image_parts;
mod inventory_item;
mod kaamo_status;
mod medal;
mod mission;
mod pending_product;
mod ship;
mod ship_equipment;
mod station;
mod unknown_structure_1;
mod unknown_structure_2;
mod wanted;

const BINARY_HASH_CONSTANT: &[u8] = &[
    0x23, 0x2b, 0xc2, 0xa7, 0x52, 0x30, 0x4c, 0x4c, 0x33, 0x72, 0x28, 0x30, 0x61, 0x53, 0x74, 0x65,
    0x72, 0x26, 0x5f, 0x25, 0x3d, 0x24, 0x2b, 0x23, 0x00,
];
const INJECTED_HASH_CONSTANT: &[u8] = b"9506fe987e36474d";

#[derive(Debug, Deserialize, Serialize)]
pub struct Save {
    pub visited_stations: Vec<bool>,
    pub credits: i32,
    pub rating: i32,
    pub play_time_ms: i64,
    pub kills: i32,
    pub mission_count: i32,
    pub level: i32,
    pub last_xp: i32,
    pub goods_produced: i32,
    pub stations_visited: i32,
    pub current_campaign_mission: i32,
    pub freelance_mission: Option<SaveMission>,
    pub campaign_mission: Option<SaveMission>,
    pub jumpgates_used: i32,
    pub captured_crates: i32,
    pub bought_equipment: i32,
    pub pirate_kills: i32,
    pub unknown_int_1: i32,
    pub unknown_int_2: i32,
    pub last_docked_station: i32,
    pub unknown_int_4: i32,
    pub unknown_bool_list_1: Vec<bool>,
    pub unknown_bool_list_2: Vec<bool>,
    pub unknown_int_5: i32,
    pub ores_mined: i32,
    pub cores_mined: i32,
    pub booze_purchased: i32,
    pub booze_types_obtained: Vec<bool>,
    pub space_junk_destroyed: i32,
    pub visited_systems: Vec<bool>,
    pub passengers_carried: i32,
    pub unknown_long_int_1: i64,
    pub bombs_detonated: i32,
    pub alien_remains_collected: i32,
    pub people_spoken_to: i32,
    pub wingmen_hired: i32,
    pub asteroids_destroyed: i32,
    pub unknown_int_16: i32,
    pub unknown_int_17: i32,
    pub unknown_int_18: i32,
    pub unknown_int_19: i32,
    pub unknown_int_20: i32,
    pub medals: Vec<Medal>,
    pub ship: SaveShip,
    pub ship_equipment: Vec<Option<SaveShipEquipment>>,
    pub ship_cargo: Vec<SaveInventoryItem>,
    pub station_stack: Vec<Option<SaveStation>>,
    pub standings: Vec<i32>,
    pub blueprints: Vec<SaveBlueprint>,
    pub pending_products: Vec<SavePendingProduct>,
    pub unknown_structure_1: Option<UnknownStructure1>,
    pub passengers: i32,
    pub system_visibilities: Vec<bool>,
    pub unknown_int_list_2: Vec<i32>,
    pub unknown_int_list_3: Vec<i32>,
    pub unknown_int_list_4: Vec<i32>,
    pub unknown_int_list_5: Vec<i32>,
    pub unknown_bool_list_6: Vec<bool>,
    pub agents: Vec<SaveAgent>,
    pub unknown_bool_1: bool,
    pub unknown_bool_2: bool,
    pub unknown_bool_3: bool,
    pub unknown_bool_4: bool,
    pub unknown_bool_5: bool,
    pub unknown_bool_6: bool,
    pub unknown_bool_7: bool,
    pub unknown_bool_8: bool,
    pub unknown_bool_9: bool,
    pub unknown_bool_10: bool,
    pub unknown_bool_11: bool,
    pub unknown_bool_12: bool,
    pub unknown_bool_13: bool,
    pub unknown_bool_14: bool,
    pub unknown_bool_15: bool,
    pub unknown_bool_16: bool,
    pub unknown_bool_17: bool,
    pub unknown_bool_18: bool,
    pub unknown_bool_19: bool,
    pub unknown_bool_20: bool,
    pub unknown_bool_21: bool,
    pub unknown_bool_22: bool,
    pub unknown_bool_23: bool,
    pub unknown_bool_24: bool,
    pub unknown_bool_25: bool,
    pub unknown_bool_26: bool,
    pub unknown_bool_27: bool,
    pub unknown_bool_28: bool,
    pub unknown_bool_29: bool,
    pub unknown_float_1: f32,
    pub unknown_long_int_2: i64,
    pub unknown_bool_30: bool,
    pub unknown_bool_31: bool,
    pub unknown_structure_2: Option<UnknownStructure2>,
    pub unknown_int_list_6: Vec<i32>,
    pub unknown_int_22: i32,
    pub unknown_bool_32: bool,
    pub kaamo_status: KaamoStatus,
    pub unknown_bool_33: bool,
    pub station_items: Vec<SaveInventoryItem>,
    pub station_ships: Vec<SaveShip>,
    pub unknown_bool_34: bool,
    pub unknown_bool_35: bool,
    pub unknown_bool_36: bool,
    pub items_obtained: Vec<bool>,
    pub unknown_constant: i32,
    pub unknown_int_list_7: Vec<i32>,
    pub unknown_int_list_8: Vec<i32>,
    pub unknown_int_list_list_1: Vec<Vec<i32>>,
    pub unknown_structure_3: Vec<Vec<Vec<i32>>>,
    pub most_wanted: Vec<SaveWanted>,
    pub collected_bounties: Vec<i32>,
    pub unknown_bool_37: bool,
    pub unknown_int_24: i32,
    pub unknown_bool_38: bool,
    pub unknown_bool_39: bool,
    pub unknown_bool_40: bool,
    pub unknown_bool_41: bool,
    pub unknown_bool_42: bool,
    pub unknown_bool_43: bool,
    pub unknown_bool_44: bool,
    pub unknown_bool_45: bool,
    pub unknown_bool_list_8: Vec<bool>,
    pub unknown_bool_46: bool,
    pub unknown_bool_47: bool,
    pub unknown_bool_48: bool,
    pub unknown_int_25: i32,
    pub unknown_bool_49: bool,
    pub unknown_bool_50: bool,
    pub unknown_bool_51: bool,
    pub unknown_bool_52: bool,
    pub unknown_bool_53: bool,
    pub unknown_bool_54: bool,
    pub unknown_bool_55: bool,
    pub unknown_bool_56: bool,
    pub hash: Vec<u8>,
}

impl BinRead for Save {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let station_stack_count;

        Ok(Self {
            visited_stations: source.read_bin::<O>()?,
            credits: source.read_i32::<O>()?,
            rating: source.read_i32::<O>()?,
            play_time_ms: source.read_i64::<O>()?,
            kills: source.read_i32::<O>()?,
            mission_count: source.read_i32::<O>()?,
            level: source.read_i32::<O>()?,
            last_xp: source.read_i32::<O>()?,
            goods_produced: source.read_i32::<O>()?,
            stations_visited: source.read_i32::<O>()?,
            current_campaign_mission: source.read_i32::<O>()?,
            freelance_mission: source.read_bin::<O>()?,
            campaign_mission: source.read_bin::<O>()?,
            jumpgates_used: source.read_i32::<O>()?,
            captured_crates: source.read_i32::<O>()?,
            bought_equipment: source.read_i32::<O>()?,
            pirate_kills: source.read_i32::<O>()?,
            unknown_int_1: source.read_i32::<O>()?,
            unknown_int_2: source.read_i32::<O>()?,
            last_docked_station: source.read_i32::<O>()?,
            unknown_int_4: source.read_i32::<O>()?,
            unknown_bool_list_1: source.read_bin::<O>()?,
            unknown_bool_list_2: source.read_bin::<O>()?,
            unknown_int_5: source.read_i32::<O>()?,
            ores_mined: source.read_i32::<O>()?,
            cores_mined: source.read_i32::<O>()?,
            booze_purchased: source.read_i32::<O>()?,
            booze_types_obtained: source.read_bin::<O>()?,
            space_junk_destroyed: source.read_i32::<O>()?,
            visited_systems: source.read_bin::<O>()?,
            passengers_carried: source.read_i32::<O>()?,
            unknown_long_int_1: source.read_i64::<O>()?,
            bombs_detonated: source.read_i32::<O>()?,
            alien_remains_collected: source.read_i32::<O>()?,
            people_spoken_to: source.read_i32::<O>()?,
            wingmen_hired: source.read_i32::<O>()?,
            asteroids_destroyed: source.read_i32::<O>()?,
            unknown_int_16: source.read_i32::<O>()?,
            unknown_int_17: source.read_i32::<O>()?,
            unknown_int_18: source.read_i32::<O>()?,
            unknown_int_19: source.read_i32::<O>()?,
            unknown_int_20: source.read_i32::<O>()?,
            medals: source.read_bin::<O>()?,
            ship: source.read_bin::<O>()?,
            ship_equipment: source.read_bin::<O>()?,
            ship_cargo: source.read_bin::<O>()?,
            station_stack: {
                let station_stack: Vec<Option<SaveStation>> = source.read_bin::<O>()?;

                station_stack_count = station_stack.iter().filter(|s| s.is_some()).count();

                station_stack
            },
            standings: source.read_bin::<O>()?,
            blueprints: source.read_bin::<O>()?,
            pending_products: source.read_bin::<O>()?,
            unknown_structure_1: source.read_bin::<O>()?,
            passengers: source.read_i32::<O>()?,
            system_visibilities: source.read_bin::<O>()?,
            unknown_int_list_2: source.read_bin::<O>()?,
            unknown_int_list_3: source.read_bin::<O>()?,
            unknown_int_list_4: source.read_bin::<O>()?,
            unknown_int_list_5: source.read_bin::<O>()?,
            unknown_bool_list_6: source.read_bin::<O>()?,
            agents: source.read_bin::<O>()?,
            unknown_bool_1: source.read_u8()? != 0,
            unknown_bool_2: source.read_u8()? != 0,
            unknown_bool_3: source.read_u8()? != 0,
            unknown_bool_4: source.read_u8()? != 0,
            unknown_bool_5: source.read_u8()? != 0,
            unknown_bool_6: source.read_u8()? != 0,
            unknown_bool_7: source.read_u8()? != 0,
            unknown_bool_8: source.read_u8()? != 0,
            unknown_bool_9: source.read_u8()? != 0,
            unknown_bool_10: source.read_u8()? != 0,
            unknown_bool_11: source.read_u8()? != 0,
            unknown_bool_12: source.read_u8()? != 0,
            unknown_bool_13: source.read_u8()? != 0,
            unknown_bool_14: source.read_u8()? != 0,
            unknown_bool_15: source.read_u8()? != 0,
            unknown_bool_16: source.read_u8()? != 0,
            unknown_bool_17: source.read_u8()? != 0,
            unknown_bool_18: source.read_u8()? != 0,
            unknown_bool_19: source.read_u8()? != 0,
            unknown_bool_20: source.read_u8()? != 0,
            unknown_bool_21: source.read_u8()? != 0,
            unknown_bool_22: source.read_u8()? != 0,
            unknown_bool_23: source.read_u8()? != 0,
            unknown_bool_24: source.read_u8()? != 0,
            unknown_bool_25: source.read_u8()? != 0,
            unknown_bool_26: source.read_u8()? != 0,
            unknown_bool_27: source.read_u8()? != 0,
            unknown_bool_28: source.read_u8()? != 0,
            unknown_bool_29: source.read_u8()? != 0,
            unknown_float_1: source.read_f32::<O>()?,
            unknown_long_int_2: source.read_i64::<O>()?,
            unknown_bool_30: source.read_u8()? != 0,
            unknown_bool_31: source.read_u8()? != 0,
            unknown_structure_2: source.read_bin::<O>()?,
            unknown_int_list_6: source.read_bin::<O>()?,
            unknown_int_22: source.read_i32::<O>()?,
            unknown_bool_32: source.read_u8()? != 0,
            kaamo_status: source.read_bin::<O>()?,
            unknown_bool_33: source.read_u8()? != 0,
            station_items: source.read_bin::<O>()?,
            station_ships: source.read_bin::<O>()?,
            unknown_bool_34: source.read_u8()? != 0,
            unknown_bool_35: source.read_u8()? != 0,
            unknown_bool_36: source.read_u8()? != 0,
            items_obtained: source.read_bin::<O>()?,
            unknown_constant: source.read_i32::<O>()?,
            unknown_int_list_7: source.read_bin::<O>()?,
            unknown_int_list_8: source.read_bin::<O>()?,
            unknown_int_list_list_1: source.read_bin::<O>()?,
            unknown_structure_3: (0..station_stack_count)
                .map(|_| source.read_bin::<O>())
                .collect::<Result<_>>()?,
            most_wanted: source.read_bin::<O>()?,
            collected_bounties: vec![
                source.read_i32::<O>()?,
                source.read_i32::<O>()?,
                source.read_i32::<O>()?,
                source.read_i32::<O>()?,
            ],
            unknown_bool_37: source.read_u8()? != 0,
            unknown_int_24: source.read_i32::<O>()?,
            unknown_bool_38: source.read_u8()? != 0,
            unknown_bool_39: source.read_u8()? != 0,
            unknown_bool_40: source.read_u8()? != 0,
            unknown_bool_41: source.read_u8()? != 0,
            unknown_bool_42: source.read_u8()? != 0,
            unknown_bool_43: source.read_u8()? != 0,
            unknown_bool_44: source.read_u8()? != 0,
            unknown_bool_45: source.read_u8()? != 0,
            unknown_bool_list_8: source.read_bin::<O>()?,
            unknown_bool_46: source.read_u8()? != 0,
            unknown_bool_47: source.read_u8()? != 0,
            unknown_bool_48: source.read_u8()? != 0,
            unknown_int_25: source.read_i32::<O>()?,
            unknown_bool_49: source.read_u8()? != 0,
            unknown_bool_50: source.read_u8()? != 0,
            unknown_bool_51: source.read_u8()? != 0,
            unknown_bool_52: source.read_u8()? != 0,
            unknown_bool_53: source.read_u8()? != 0,
            unknown_bool_54: source.read_u8()? != 0,
            unknown_bool_55: source.read_u8()? != 0,
            unknown_bool_56: source.read_u8()? != 0,
            hash: read_remaining(source)?,
        })
    }
}

impl BinWrite for Save {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let mut bytes = vec![];
        write_save::<O>(&mut bytes, self)?;

        let hash = calculate_save_hash(&bytes);

        destination.write_all(&bytes)?;
        destination.write_all(&hash)?;

        Ok(())
    }
}

fn read_remaining(source: &mut impl Read) -> io::Result<Vec<u8>> {
    let mut buffer = vec![];
    source.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn write_save<O: ByteOrder>(destination: &mut impl Write, save: &Save) -> Result<()> {
    destination.write_bin::<O>(&save.visited_stations)?;
    destination.write_i32::<O>(save.credits)?;
    destination.write_i32::<O>(save.rating)?;
    destination.write_i64::<O>(save.play_time_ms)?;
    destination.write_i32::<O>(save.kills)?;
    destination.write_i32::<O>(save.mission_count)?;
    destination.write_i32::<O>(save.level)?;
    destination.write_i32::<O>(save.last_xp)?;
    destination.write_i32::<O>(save.goods_produced)?;
    destination.write_i32::<O>(save.stations_visited)?;
    destination.write_i32::<O>(save.current_campaign_mission)?;
    destination.write_bin::<O>(&save.freelance_mission)?;
    destination.write_bin::<O>(&save.campaign_mission)?;
    destination.write_i32::<O>(save.jumpgates_used)?;
    destination.write_i32::<O>(save.captured_crates)?;
    destination.write_i32::<O>(save.bought_equipment)?;
    destination.write_i32::<O>(save.pirate_kills)?;
    destination.write_i32::<O>(save.unknown_int_1)?;
    destination.write_i32::<O>(save.unknown_int_2)?;
    destination.write_i32::<O>(save.last_docked_station)?;
    destination.write_i32::<O>(save.unknown_int_4)?;
    destination.write_bin::<O>(&save.unknown_bool_list_1)?;
    destination.write_bin::<O>(&save.unknown_bool_list_2)?;
    destination.write_i32::<O>(save.unknown_int_5)?;
    destination.write_i32::<O>(save.ores_mined)?;
    destination.write_i32::<O>(save.cores_mined)?;
    destination.write_i32::<O>(save.booze_purchased)?;
    destination.write_bin::<O>(&save.booze_types_obtained)?;
    destination.write_i32::<O>(save.space_junk_destroyed)?;
    destination.write_bin::<O>(&save.visited_systems)?;
    destination.write_i32::<O>(save.passengers_carried)?;
    destination.write_i64::<O>(save.unknown_long_int_1)?;
    destination.write_i32::<O>(save.bombs_detonated)?;
    destination.write_i32::<O>(save.alien_remains_collected)?;
    destination.write_i32::<O>(save.people_spoken_to)?;
    destination.write_i32::<O>(save.wingmen_hired)?;
    destination.write_i32::<O>(save.asteroids_destroyed)?;
    destination.write_i32::<O>(save.unknown_int_16)?;
    destination.write_i32::<O>(save.unknown_int_17)?;
    destination.write_i32::<O>(save.unknown_int_18)?;
    destination.write_i32::<O>(save.unknown_int_19)?;
    destination.write_i32::<O>(save.unknown_int_20)?;
    destination.write_bin::<O>(&save.medals)?;
    destination.write_bin::<O>(&save.ship)?;
    destination.write_bin::<O>(&save.ship_equipment)?;
    destination.write_bin::<O>(&save.ship_cargo)?;
    destination.write_bin::<O>(&save.station_stack)?;
    destination.write_bin::<O>(&save.standings)?;
    destination.write_bin::<O>(&save.blueprints)?;
    destination.write_bin::<O>(&save.pending_products)?;
    destination.write_bin::<O>(&save.unknown_structure_1)?;
    destination.write_i32::<O>(save.passengers)?;
    destination.write_bin::<O>(&save.system_visibilities)?;
    destination.write_bin::<O>(&save.unknown_int_list_2)?;
    destination.write_bin::<O>(&save.unknown_int_list_3)?;
    destination.write_bin::<O>(&save.unknown_int_list_4)?;
    destination.write_bin::<O>(&save.unknown_int_list_5)?;
    destination.write_bin::<O>(&save.unknown_bool_list_6)?;
    destination.write_bin::<O>(&save.agents)?;
    destination.write_u8(save.unknown_bool_1.into())?;
    destination.write_u8(save.unknown_bool_2.into())?;
    destination.write_u8(save.unknown_bool_3.into())?;
    destination.write_u8(save.unknown_bool_4.into())?;
    destination.write_u8(save.unknown_bool_5.into())?;
    destination.write_u8(save.unknown_bool_6.into())?;
    destination.write_u8(save.unknown_bool_7.into())?;
    destination.write_u8(save.unknown_bool_8.into())?;
    destination.write_u8(save.unknown_bool_9.into())?;
    destination.write_u8(save.unknown_bool_10.into())?;
    destination.write_u8(save.unknown_bool_11.into())?;
    destination.write_u8(save.unknown_bool_12.into())?;
    destination.write_u8(save.unknown_bool_13.into())?;
    destination.write_u8(save.unknown_bool_14.into())?;
    destination.write_u8(save.unknown_bool_15.into())?;
    destination.write_u8(save.unknown_bool_16.into())?;
    destination.write_u8(save.unknown_bool_17.into())?;
    destination.write_u8(save.unknown_bool_18.into())?;
    destination.write_u8(save.unknown_bool_19.into())?;
    destination.write_u8(save.unknown_bool_20.into())?;
    destination.write_u8(save.unknown_bool_21.into())?;
    destination.write_u8(save.unknown_bool_22.into())?;
    destination.write_u8(save.unknown_bool_23.into())?;
    destination.write_u8(save.unknown_bool_24.into())?;
    destination.write_u8(save.unknown_bool_25.into())?;
    destination.write_u8(save.unknown_bool_26.into())?;
    destination.write_u8(save.unknown_bool_27.into())?;
    destination.write_u8(save.unknown_bool_28.into())?;
    destination.write_u8(save.unknown_bool_29.into())?;
    destination.write_f32::<O>(save.unknown_float_1)?;
    destination.write_i64::<O>(save.unknown_long_int_2)?;
    destination.write_u8(save.unknown_bool_30.into())?;
    destination.write_u8(save.unknown_bool_31.into())?;
    destination.write_bin::<O>(&save.unknown_structure_2)?;
    destination.write_bin::<O>(&save.unknown_int_list_6)?;
    destination.write_i32::<O>(save.unknown_int_22)?;
    destination.write_u8(save.unknown_bool_32.into())?;
    destination.write_bin::<O>(&save.kaamo_status)?;
    destination.write_u8(save.unknown_bool_33.into())?;
    destination.write_bin::<O>(&save.station_items)?;
    destination.write_bin::<O>(&save.station_ships)?;
    destination.write_u8(save.unknown_bool_34.into())?;
    destination.write_u8(save.unknown_bool_35.into())?;
    destination.write_u8(save.unknown_bool_36.into())?;
    destination.write_bin::<O>(&save.items_obtained)?;
    destination.write_i32::<O>(save.unknown_constant)?;
    destination.write_bin::<O>(&save.unknown_int_list_7)?;
    destination.write_bin::<O>(&save.unknown_int_list_8)?;
    destination.write_bin::<O>(&save.unknown_int_list_list_1)?;
    save.unknown_structure_3
        .iter()
        .try_for_each(|unknown| destination.write_bin::<O>(unknown))?;
    destination.write_bin::<O>(&save.most_wanted)?;
    save.collected_bounties
        .iter()
        .try_for_each(|bounties| destination.write_i32::<O>(*bounties))?;
    destination.write_u8(save.unknown_bool_37.into())?;
    destination.write_i32::<O>(save.unknown_int_24)?;
    destination.write_u8(save.unknown_bool_38.into())?;
    destination.write_u8(save.unknown_bool_39.into())?;
    destination.write_u8(save.unknown_bool_40.into())?;
    destination.write_u8(save.unknown_bool_41.into())?;
    destination.write_u8(save.unknown_bool_42.into())?;
    destination.write_u8(save.unknown_bool_43.into())?;
    destination.write_u8(save.unknown_bool_44.into())?;
    destination.write_u8(save.unknown_bool_45.into())?;
    destination.write_bin::<O>(&save.unknown_bool_list_8)?;
    destination.write_u8(save.unknown_bool_46.into())?;
    destination.write_u8(save.unknown_bool_47.into())?;
    destination.write_u8(save.unknown_bool_48.into())?;
    destination.write_i32::<O>(save.unknown_int_25)?;
    destination.write_u8(save.unknown_bool_49.into())?;
    destination.write_u8(save.unknown_bool_50.into())?;
    destination.write_u8(save.unknown_bool_51.into())?;
    destination.write_u8(save.unknown_bool_52.into())?;
    destination.write_u8(save.unknown_bool_53.into())?;
    destination.write_u8(save.unknown_bool_54.into())?;
    destination.write_u8(save.unknown_bool_55.into())?;
    destination.write_u8(save.unknown_bool_56.into())?;

    Ok(())
}

fn calculate_save_hash(save_bytes: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(save_bytes);
    hasher.update(BINARY_HASH_CONSTANT);
    hasher.update(INJECTED_HASH_CONSTANT);

    hasher.finalize().to_vec()
}
