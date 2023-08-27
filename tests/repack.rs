use gof2edit::data::{Item, LangString, Ship, Station, System};
use std::io::Cursor;
use std::ops::Not;

const ITEMS_JSON: &str = include_str!("data/items.json");
const LANG_JSON: &str = include_str!("data/lang.json");
const SHIPS_JSON: &str = include_str!("data/ships.json");
const STATIONS_JSON: &str = include_str!("data/stations.json");
const SYSTEMS_JSON: &str = include_str!("data/systems.json");

#[test]
fn convert_items_json_to_bin() {
    let items =
        serde_json::from_str::<Vec<Item>>(ITEMS_JSON).expect("failed to parse item test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    assert!(gof2edit::write_object_list(&mut buffer, items).is_ok());

    let bin = buffer.into_inner();

    assert!(bin.is_empty().not());
}

#[test]
fn convert_lang_json_to_bin() {
    let lang_strings = serde_json::from_str::<Vec<LangString>>(LANG_JSON)
        .expect("failed to parse lang string test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    assert!(gof2edit::write_object_list(&mut buffer, lang_strings).is_ok());

    let bin = buffer.into_inner();

    assert!(bin.is_empty().not());
}

#[test]
fn convert_ships_json_to_bin() {
    let ships =
        serde_json::from_str::<Vec<Ship>>(SHIPS_JSON).expect("failed to parse ship test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    assert!(gof2edit::write_object_list(&mut buffer, ships).is_ok());

    let bin = buffer.into_inner();

    assert!(bin.is_empty().not());
}

#[test]
fn convert_stations_json_to_bin() {
    let stations = serde_json::from_str::<Vec<Station>>(STATIONS_JSON)
        .expect("failed to parse station test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    assert!(gof2edit::write_object_list(&mut buffer, stations).is_ok());

    let bin = buffer.into_inner();

    assert!(bin.is_empty().not());
}

#[test]
fn convert_systems_json_to_bin() {
    let systems = serde_json::from_str::<Vec<System>>(SYSTEMS_JSON)
        .expect("failed to parse system test JSON");

    let mut buffer = Cursor::new(vec![0u8]);

    assert!(gof2edit::write_object_list(&mut buffer, systems).is_ok());

    let bin = buffer.into_inner();

    assert!(bin.is_empty().not());
}
