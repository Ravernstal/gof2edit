use crate::patch_addresses::binary_version::BinaryVersion;
use crate::patch_addresses::{write_value, WriteValueFn};

pub fn addresses(binary: BinaryVersion) -> &'static [(u64, &'static WriteValueFn)] {
    match binary {
        BinaryVersion::AndroidKiritoJpk => android_kirito_jpk_addresses(),
    }
}

fn android_kirito_jpk_addresses() -> &'static [(u64, &'static WriteValueFn)] {
    &[
        (0xA5DB2, &write_value::default),
        (0xA5DDA, &write_value::default),
        (0xC69A8, &write_value::default),
        (0xC69BA, &write_value::default),
        (0x135C66, &write_value::default),
        (0x135E80, &write_value::default),
    ]
}
