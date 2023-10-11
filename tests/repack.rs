use byteorder::{BigEndian, LittleEndian};
use gof2edit::data::{Item, LangString, SavePreview, Ship, ShipPosition, Station, System, Wanted};
use std::io::Cursor;
use std::ops::Not;

const ITEMS_JSON: &str = include_str!("data/items.json");
const LANG_JSON: &str = include_str!("data/lang.json");
const SAVE_PREVIEW_JSON: &str = include_str!("data/gof2_save_game_preview_0.json");
const SHIPS_JSON: &str = include_str!("data/ships.json");
const SHIP_POSITIONS_JSON: &str = include_str!("data/ship_positions.json");
const STATIONS_JSON: &str = include_str!("data/stations.json");
const SYSTEMS_JSON: &str = include_str!("data/systems.json");
const WANTED_JSON: &str = include_str!("data/wanted.json");

#[test]
fn convert_items_json_to_bin() {
    let items =
        serde_json::from_str::<Vec<Item>>(ITEMS_JSON).expect("failed to parse item test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    assert!(gof2edit::write_object_list::<Item, BigEndian>(&mut buffer, items).is_ok());

    let bin = buffer.into_inner();

    assert!(bin.is_empty().not());
}

#[test]
fn convert_lang_json_to_bin() {
    let lang_strings = serde_json::from_str::<Vec<LangString>>(LANG_JSON)
        .expect("failed to parse lang string test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    assert!(
        gof2edit::write_object_list::<LangString, BigEndian>(&mut buffer, lang_strings).is_ok()
    );

    let bin = buffer.into_inner();

    assert!(bin.is_empty().not());
}

#[test]
fn convert_save_preview_json_to_bin() {
    let save_preview = serde_json::from_str::<SavePreview>(SAVE_PREVIEW_JSON)
        .expect("failed to parse save preview test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    assert!(
        gof2edit::write_object::<SavePreview, LittleEndian>(&mut buffer, &save_preview).is_ok()
    );

    let bin = buffer.into_inner();

    assert!(bin.is_empty().not());
}

#[test]
fn convert_ships_json_to_bin() {
    let ships =
        serde_json::from_str::<Vec<Ship>>(SHIPS_JSON).expect("failed to parse ship test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    assert!(gof2edit::write_object_list::<Ship, BigEndian>(&mut buffer, ships).is_ok());

    let bin = buffer.into_inner();

    assert!(bin.is_empty().not());
}

#[test]
fn convert_ship_positions_json_to_bin() {
    let ship_positions = serde_json::from_str::<Vec<ShipPosition>>(SHIP_POSITIONS_JSON)
        .expect("failed to parse ship positions test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    assert!(
        gof2edit::write_object_list::<ShipPosition, LittleEndian>(&mut buffer, ship_positions)
            .is_ok()
    );

    let bin = buffer.into_inner();

    assert!(bin.is_empty().not());
}

#[test]
fn convert_stations_json_to_bin() {
    let stations = serde_json::from_str::<Vec<Station>>(STATIONS_JSON)
        .expect("failed to parse station test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    assert!(gof2edit::write_object_list::<Station, BigEndian>(&mut buffer, stations).is_ok());

    let bin = buffer.into_inner();

    assert!(bin.is_empty().not());
}

#[test]
fn convert_systems_json_to_bin() {
    let systems = serde_json::from_str::<Vec<System>>(SYSTEMS_JSON)
        .expect("failed to parse system test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    assert!(gof2edit::write_object_list::<System, BigEndian>(&mut buffer, systems).is_ok());

    let bin = buffer.into_inner();

    assert!(bin.is_empty().not());
}

#[test]
fn convert_most_wanted_json_to_bin() {
    let most_wanted = serde_json::from_str::<Vec<Wanted>>(WANTED_JSON)
        .expect("failed to parse most wanted test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    assert!(gof2edit::write_object_list::<Wanted, BigEndian>(&mut buffer, most_wanted).is_ok());

    let bin = buffer.into_inner();

    assert!(bin.is_empty().not());
}
