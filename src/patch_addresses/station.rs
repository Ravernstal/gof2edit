use crate::patch_addresses::binary_version::BinaryVersion;

const ANDROID_KIRITO_JPK_ADDRESSES: &[(u64, i8)] = &[
    (0x91ED8, 0),
    (0xCF7C4, 0),
    (0xCF7D6, 0),
    (0xE915A, 0),
    (0x134DFE, 0),
    (0x135646, 0),
    (0x135B6A, 0),
    (0x170CFE, 0),
    (0x194CE0, 0),
    (0x194CE6, 0),
    (0x194D58, 0),
    (0x194DE0, -1),
    (0x194DEA, 0),
];

pub fn addresses(binary: BinaryVersion) -> &'static [(u64, i8)] {
    match binary {
        BinaryVersion::AndroidKiritoJpk => ANDROID_KIRITO_JPK_ADDRESSES,
    }
}
