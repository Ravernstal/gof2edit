use crate::patch_addresses::binary_version::BinaryVersion;
use crate::patch_addresses::{write_value, WriteValueFn};

pub fn addresses(binary: BinaryVersion) -> &'static [(u64, &'static WriteValueFn)] {
    match binary {
        BinaryVersion::AndroidKiritoJpk => android_kirito_jpk_addresses(),
    }
}

fn android_kirito_jpk_addresses() -> &'static [(u64, &'static WriteValueFn)] {
    &[
        (0x91ED8, &write_value::default),
        (0xCF7C4, &write_value::default),
        (0xCF7D6, &write_value::default),
        (0xE915A, &write_value::default),
        (0x134DFE, &write_value::default),
        (0x135646, &write_value::default),
        (0x135B6A, &write_value::default),
        (0x170CFE, &write_value::default),
        (0x194CE0, &write_value::default),
        (0x194CE6, &write_value::default),
        (0x194D58, &write_value::default),
        (0x194DE0, &write_value::minus_one),
        (0x194DEA, &write_value::default),
    ]
}
