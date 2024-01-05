use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::data::save::agent::SaveAgent;
use crate::data::save::blueprint::SaveBlueprint;
use crate::data::save::inventory_item::SaveInventoryItem;
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
use std::io;
use std::io::{Read, Write};

mod agent;
mod agent_mission;
mod blueprint;
mod image_parts;
mod inventory_item;
mod mission;
mod pending_product;
mod ship;
mod ship_equipment;
mod station;
mod unknown_structure_1;
mod unknown_structure_2;
mod wanted;

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
    pub unknown_int_3: i32,
    pub unknown_int_4: i32,
    pub unknown_bool_list_1: Vec<bool>,
    pub unknown_bool_list_2: Vec<bool>,
    pub unknown_int_5: i32,
    pub unknown_int_6: i32,
    pub unknown_int_7: i32,
    pub unknown_int_8: i32,
    pub unknown_bool_list_3: Vec<bool>,
    pub unknown_int_9: i32,
    pub unknown_bool_list_4: Vec<bool>,
    pub unknown_int_10: i32,
    pub unknown_long_int_1: i64,
    pub unknown_int_11: i32,
    pub unknown_int_12: i32,
    pub unknown_int_13: i32,
    pub unknown_int_14: i32,
    pub unknown_int_15: i32,
    pub unknown_int_16: i32,
    pub unknown_int_17: i32,
    pub unknown_int_18: i32,
    pub unknown_int_19: i32,
    pub unknown_int_20: i32,
    pub unknown_int_list_1: Vec<i32>,
    pub ship: SaveShip,
    pub ship_equipment: Vec<Option<SaveShipEquipment>>,
    pub ship_cargo: Vec<SaveInventoryItem>,
    pub station_stack: Vec<Option<SaveStation>>,
    pub standings: Vec<i32>,
    pub blueprints: Vec<SaveBlueprint>,
    pub pending_products: Vec<SavePendingProduct>,
    pub unknown_structure_1: Option<UnknownStructure1>,
    pub unknown_int_21: i32,
    pub unknown_bool_list_5: Vec<bool>,
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
    pub unknown_int_23: i32,
    pub unknown_bool_33: bool,
    pub station_items: Vec<SaveInventoryItem>,
    pub station_ships: Vec<SaveShip>,
    pub unknown_bool_34: bool,
    pub unknown_bool_35: bool,
    pub unknown_bool_36: bool,
    pub unknown_bool_list_7: Vec<bool>,
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
            unknown_int_3: source.read_i32::<O>()?,
            unknown_int_4: source.read_i32::<O>()?,
            unknown_bool_list_1: source.read_bin::<O>()?,
            unknown_bool_list_2: source.read_bin::<O>()?,
            unknown_int_5: source.read_i32::<O>()?,
            unknown_int_6: source.read_i32::<O>()?,
            unknown_int_7: source.read_i32::<O>()?,
            unknown_int_8: source.read_i32::<O>()?,
            unknown_bool_list_3: source.read_bin::<O>()?,
            unknown_int_9: source.read_i32::<O>()?,
            unknown_bool_list_4: source.read_bin::<O>()?,
            unknown_int_10: source.read_i32::<O>()?,
            unknown_long_int_1: source.read_i64::<O>()?,
            unknown_int_11: source.read_i32::<O>()?,
            unknown_int_12: source.read_i32::<O>()?,
            unknown_int_13: source.read_i32::<O>()?,
            unknown_int_14: source.read_i32::<O>()?,
            unknown_int_15: source.read_i32::<O>()?,
            unknown_int_16: source.read_i32::<O>()?,
            unknown_int_17: source.read_i32::<O>()?,
            unknown_int_18: source.read_i32::<O>()?,
            unknown_int_19: source.read_i32::<O>()?,
            unknown_int_20: source.read_i32::<O>()?,
            unknown_int_list_1: source.read_bin::<O>()?,
            ship: source.read_bin::<O>()?,
            ship_equipment: source.read_bin::<O>()?,
            ship_cargo: source.read_bin::<O>()?,
            station_stack: source.read_bin::<O>()?,
            standings: source.read_bin::<O>()?,
            blueprints: source.read_bin::<O>()?,
            pending_products: source.read_bin::<O>()?,
            unknown_structure_1: source.read_bin::<O>()?,
            unknown_int_21: source.read_i32::<O>()?,
            unknown_bool_list_5: source.read_bin::<O>()?,
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
            unknown_int_23: source.read_i32::<O>()?,
            unknown_bool_33: source.read_u8()? != 0,
            station_items: source.read_bin::<O>()?,
            station_ships: source.read_bin::<O>()?,
            unknown_bool_34: source.read_u8()? != 0,
            unknown_bool_35: source.read_u8()? != 0,
            unknown_bool_36: source.read_u8()? != 0,
            unknown_bool_list_7: source.read_bin::<O>()?,
            unknown_constant: source.read_i32::<O>()?,
            unknown_int_list_7: source.read_bin::<O>()?,
            unknown_int_list_8: source.read_bin::<O>()?,
            unknown_int_list_list_1: source.read_bin::<O>()?,
            unknown_structure_3: vec![
                // TODO: Potentially dodgy. Possibly based on station stack count
                source.read_bin::<O>()?,
                source.read_bin::<O>()?,
                source.read_bin::<O>()?,
                source.read_bin::<O>()?,
            ],
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
            hash: {
                let remaining_bytes = read_remaining(source)?;

                println!("{} bytes remaining", remaining_bytes.len());

                remaining_bytes
            },
        })
    }
}

impl BinWrite for Save {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        destination.write_bin::<O>(&self.visited_stations)?;
        destination.write_i32::<O>(self.credits)?;
        destination.write_i32::<O>(self.rating)?;
        destination.write_i64::<O>(self.play_time_ms)?;
        destination.write_i32::<O>(self.kills)?;
        destination.write_i32::<O>(self.mission_count)?;
        destination.write_i32::<O>(self.level)?;
        destination.write_i32::<O>(self.last_xp)?;
        destination.write_i32::<O>(self.goods_produced)?;
        destination.write_i32::<O>(self.stations_visited)?;
        destination.write_i32::<O>(self.current_campaign_mission)?;
        destination.write_bin::<O>(&self.freelance_mission)?;
        destination.write_bin::<O>(&self.campaign_mission)?;
        destination.write_all(&self.hash)?;

        Ok(())
    }
}

fn read_remaining(source: &mut impl Read) -> io::Result<Vec<u8>> {
    let mut buffer = vec![];
    source.read_to_end(&mut buffer)?;

    Ok(buffer)
}
