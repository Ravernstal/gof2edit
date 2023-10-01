use byteorder::{BigEndian, LittleEndian};
use gof2edit::data::{Item, LangString, Ship, ShipPosition, Station, System};
use std::io::Cursor;
use std::ops::Not;

const ITEMS_JSON: &str = include_str!("data/items.json");
const LANG_JSON: &str = include_str!("data/lang.json");
const SHIPS_JSON: &str = include_str!("data/ships.json");
const SHIP_POSITIONS_JSON: &str = include_str!("data/ship_positions.json");
const STATIONS_JSON: &str = include_str!("data/stations.json");
const SYSTEMS_JSON: &str = include_str!("data/systems.json");

#[test]
fn convert_items_bin_to_json() {
    let items =
        serde_json::from_str::<Vec<Item>>(ITEMS_JSON).expect("failed to parse item test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    gof2edit::write_object_list::<Item, BigEndian>(&mut buffer, items)
        .expect("failed to repack items to BIN");
    buffer.set_position(0);

    let items = gof2edit::read_object_list::<Item, BigEndian>(&mut buffer)
        .expect("failed to unpack items from BIN");

    assert!(items.is_empty().not())
}

#[test]
fn convert_lang_bin_to_json() {
    let lang_strings = serde_json::from_str::<Vec<LangString>>(LANG_JSON)
        .expect("failed to parse lang string test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    gof2edit::write_object_list::<LangString, BigEndian>(&mut buffer, lang_strings)
        .expect("failed to repack lang strings to BIN");
    buffer.set_position(0);

    let lang_strings = gof2edit::read_object_list::<LangString, BigEndian>(&mut buffer)
        .expect("failed to unpack lang strings from BIN");

    assert!(lang_strings.is_empty().not())
}

#[test]
fn convert_ships_bin_to_json() {
    let ships =
        serde_json::from_str::<Vec<Ship>>(SHIPS_JSON).expect("failed to parse ship test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    gof2edit::write_object_list::<Ship, BigEndian>(&mut buffer, ships)
        .expect("failed to repack ships to BIN");
    buffer.set_position(0);

    let ships = gof2edit::read_object_list::<Ship, BigEndian>(&mut buffer)
        .expect("failed to unpack ships from BIN");

    assert!(ships.is_empty().not())
}

#[test]
fn convert_ship_positions_bin_to_json() {
    let ship_positions = serde_json::from_str::<Vec<ShipPosition>>(SHIP_POSITIONS_JSON)
        .expect("failed to parse ship positions test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    gof2edit::write_object_list::<ShipPosition, LittleEndian>(&mut buffer, ship_positions)
        .expect("failed to repack ship positions to BIN");
    buffer.set_position(0);

    let ship_positions = gof2edit::read_object_list::<ShipPosition, LittleEndian>(&mut buffer)
        .expect("failed to unpack ship positions from BIN");

    assert!(ship_positions.is_empty().not())
}

#[test]
fn convert_stations_bin_to_json() {
    let stations = serde_json::from_str::<Vec<Station>>(STATIONS_JSON)
        .expect("failed to parse station test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    gof2edit::write_object_list::<Station, BigEndian>(&mut buffer, stations)
        .expect("failed to repack stations to BIN");
    buffer.set_position(0);

    let stations = gof2edit::read_object_list::<Station, BigEndian>(&mut buffer)
        .expect("failed to unpack stations from BIN");

    assert!(stations.is_empty().not())
}

#[test]
fn convert_systems_bin_to_json() {
    let systems = serde_json::from_str::<Vec<System>>(SYSTEMS_JSON)
        .expect("failed to parse system test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    gof2edit::write_object_list::<System, BigEndian>(&mut buffer, systems)
        .expect("failed to repack systems to BIN");
    buffer.set_position(0);

    let systems = gof2edit::read_object_list::<System, BigEndian>(&mut buffer)
        .expect("failed to unpack systems from BIN");

    assert!(systems.is_empty().not())
}
