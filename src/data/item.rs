use crate::bin_io::read::{BinRead, BinReader};
use crate::bin_io::write::{BinWrite, BinWriter};
use crate::data::attribute::Attribute;
use crate::error::Error;
use crate::index::Index;
use crate::result::Result;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub index: u32,
    pub attributes: BTreeMap<Attribute, i32>,
    pub unknown_attributes: BTreeMap<u32, i32>,
    pub blueprint_ingredients: BTreeMap<u32, u32>,
}

impl Index for Item {
    fn index(&self) -> u32 {
        self.index
    }

    fn set_index(&mut self, index: u32) {
        self.index = index;
    }
}

impl BinRead for Item {
    fn read_bin<O: ByteOrder>(source: &mut impl Read) -> Result<Self> {
        let blueprint_item_ids: Vec<_> = source.read_bin::<O>()?;
        let blueprint_item_counts: Vec<_> = source.read_bin::<O>()?;

        let mut blueprint_ingredients = BTreeMap::new();

        blueprint_item_ids
            .into_iter()
            .zip(blueprint_item_counts)
            .for_each(|(id, count)| {
                blueprint_ingredients.insert(id, count);
            });

        let attribute_data_size = source.read_u32::<O>()?;

        if (attribute_data_size % 2 != 0) || (attribute_data_size == 0) {
            return Err(Error::AttributeFormat);
        }

        let attribute_pair_count = attribute_data_size / 2;

        let mut index = 0;
        let mut attributes = BTreeMap::new();
        let mut unknown_attributes = BTreeMap::new();

        for _ in 0..attribute_pair_count {
            let attribute_code = source.read_u32::<O>()?;
            let attribute_value = source.read_i32::<O>()?;

            match Attribute::try_from(attribute_code) {
                Ok(Attribute::Index) => index = attribute_value,
                Ok(attribute) => {
                    attributes.insert(attribute, attribute_value);
                }
                Err(_) => {
                    unknown_attributes.insert(attribute_code, attribute_value);
                }
            };
        }

        let index = index.try_into()?;

        Ok(Self {
            index,
            attributes,
            unknown_attributes,
            blueprint_ingredients,
        })
    }
}

impl BinWrite for Item {
    fn write_bin<O: ByteOrder>(&self, destination: &mut impl Write) -> Result<()> {
        let blueprint_item_ids = self
            .blueprint_ingredients
            .keys()
            .copied()
            .collect::<Vec<_>>();
        destination.write_bin::<O>(&blueprint_item_ids)?;

        let blueprint_counts = self
            .blueprint_ingredients
            .values()
            .copied()
            .collect::<Vec<_>>();
        destination.write_bin::<O>(&blueprint_counts)?;

        let attribute_data_size =
            (self.attributes.len() * 2) + (self.unknown_attributes.len() * 2) + 2;
        let attribute_data_size = attribute_data_size.try_into()?;
        destination.write_u32::<O>(attribute_data_size)?;

        let index = self.index.try_into()?;
        destination.write_u32::<O>(0)?;
        destination.write_i32::<O>(index)?;

        for (attribute, value) in self.attributes.iter() {
            destination.write_bin::<O>(attribute)?;
            destination.write_i32::<O>(*value)?;
        }

        for (attribute, value) in self.unknown_attributes.iter() {
            destination.write_u32::<O>(*attribute)?;
            destination.write_i32::<O>(*value)?;
        }

        Ok(())
    }
}
