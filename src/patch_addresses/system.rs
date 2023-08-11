use crate::patch_addresses::binary_version::BinaryVersion;

const ANDROID_KIRITO_JPK_ADDRESSES: &[(u64, i8)] = &[
    (0xA5DB2, 0),
    (0xA5DDA, 0),
    (0xC69A8, 0),
    (0xC69BA, 0),
    (0x135C66, 0),
    (0x135E80, 0),
];

pub fn addresses(binary: BinaryVersion) -> &'static [(u64, i8)] {
    match binary {
        BinaryVersion::AndroidKiritoJpk => ANDROID_KIRITO_JPK_ADDRESSES,
    }
}
