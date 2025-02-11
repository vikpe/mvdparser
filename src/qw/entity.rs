use crate::nano::FrameContext;
use crate::qw::primitives::{Angle, Coord};
use binrw::{BinRead, BinResult, Endian};
use bitflags::bitflags;
use std::io::{Read, Seek};

#[derive(Debug, PartialEq)]
pub struct EntityDeltas(#[allow(dead_code)] Vec<EntityDelta>);

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum EntityDelta {
    Index(u16),
    Angle1(Angle),
    Angle2(Angle),
    Angle3(Angle),
    Origin1(Coord),
    Origin2(Coord),
    Origin3(Coord),
    ColorMap(u8),
    ModelIndex(u16),
    Frame(u8),
    Skin(u8),
    Effects(u8),
    Trans(u8),
    ColourMod([u8; 3]),
}

bitflags! {
    #[derive(Debug, Default)]
    pub struct EntityBits: u32 {
        const Angle1 = 1 << 0;
        const Angle3 = 1 << 1;
        const Model = 1 << 2;
        const ColorMap = 1 << 3;
        const Skin = 1 << 4;
        const Effects = 1 << 5;
        const Solid = 1 << 6;
        const FteEvenMore = 1 << 7;
        const Origin1 = 1 << 9;
        const Origin2 = 1 << 10;
        const Origin3 = 1 << 11;
        const Angle2 = 1 << 12;
        const Frame = 1 << 13;
        const Remove = 1 << 14;
        const MoreBits = 1 << 15;
    }
}

bitflags! {
    #[derive(Debug, Default)]
    pub struct EntityBitsMore: u32 {
        const Trans = 1 << 1;
        const ModelDbl = 1 << 3;
        const EntityDbl = 1 << 5;
        const EntityDbl2 = 1 << 6;
        const YetMore = 1 << 7;
        const ColourMod = 1 << 10;
    }
}

#[derive(Debug, Default)]
pub struct EntityDeltaContext {
    pub entity_number: u16,
    pub bits: EntityBits,
    pub morebits: EntityBitsMore,
    pub use_float_coords: bool,
}

impl BinRead for EntityDeltaContext {
    type Args<'a> = &'a FrameContext;

    fn read_options<R: Read + Seek>(r: &mut R, _: Endian, ctx: Self::Args<'_>) -> BinResult<Self> {
        let raw = u16::read_le(r)?;
        if raw == 0 {
            return Ok(Default::default());
        }
        let mut entity_number = raw & 511;
        let mut bits = EntityBits::from_bits(raw as u32 & !511).unwrap();
        let mut morebits = EntityBitsMore::empty();
        if bits.contains(EntityBits::MoreBits) {
            bits |= EntityBits::from_bits(u8::read(r)? as u32).unwrap();
            if bits.contains(EntityBits::FteEvenMore) {
                morebits = EntityBitsMore::from_bits(u8::read(r)? as u32).unwrap();
                if morebits.contains(EntityBitsMore::YetMore) {
                    morebits |= EntityBitsMore::from_bits((u8::read(r)? as u32) << 8).unwrap();
                }
                if morebits.contains(EntityBitsMore::EntityDbl) {
                    entity_number += 512;
                }
                if morebits.contains(EntityBitsMore::EntityDbl2) {
                    entity_number += 1024;
                }
            }
        }
        Ok(EntityDeltaContext {
            entity_number,
            bits,
            morebits,
            use_float_coords: ctx.use_float_coords,
        })
    }
}

impl BinRead for EntityDeltas {
    type Args<'a> = &'a EntityDeltaContext;

    fn read_options<R: Read + Seek>(r: &mut R, _: Endian, ctx: Self::Args<'_>) -> BinResult<Self> {
        let mut deltas = vec![];

        deltas.push(EntityDelta::Index(ctx.entity_number));

        if ctx.bits.contains(EntityBits::Model) {
            let mut model_index = u8::read_le(r)? as u16;
            if ctx.morebits.contains(EntityBitsMore::ModelDbl) {
                model_index += 256;
            }
            deltas.push(EntityDelta::ModelIndex(model_index));
        } else if ctx.morebits.contains(EntityBitsMore::ModelDbl) {
            deltas.push(EntityDelta::ModelIndex(u16::read_le(r)?));
        }

        if ctx.bits.contains(EntityBits::Frame) {
            deltas.push(EntityDelta::Frame(u8::read_le(r)?));
        }
        if ctx.bits.contains(EntityBits::ColorMap) {
            deltas.push(EntityDelta::ColorMap(u8::read_le(r)?));
        }
        if ctx.bits.contains(EntityBits::Skin) {
            deltas.push(EntityDelta::Skin(u8::read_le(r)?));
        }
        if ctx.bits.contains(EntityBits::Effects) {
            deltas.push(EntityDelta::Effects(u8::read_le(r)?));
        }

        if ctx.bits.contains(EntityBits::Origin1) {
            deltas.push(EntityDelta::Origin1(Coord::read_le_args(
                r,
                ctx.use_float_coords,
            )?));
        }
        if ctx.bits.contains(EntityBits::Angle1) {
            deltas.push(EntityDelta::Angle1(Angle::read_le_args(
                r,
                ctx.use_float_coords,
            )?));
        }
        if ctx.bits.contains(EntityBits::Origin2) {
            deltas.push(EntityDelta::Origin2(Coord::read_le_args(
                r,
                ctx.use_float_coords,
            )?));
        }
        if ctx.bits.contains(EntityBits::Angle2) {
            deltas.push(EntityDelta::Angle2(Angle::read_le_args(
                r,
                ctx.use_float_coords,
            )?));
        }
        if ctx.bits.contains(EntityBits::Origin3) {
            deltas.push(EntityDelta::Origin3(Coord::read_le_args(
                r,
                ctx.use_float_coords,
            )?));
        }
        if ctx.bits.contains(EntityBits::Angle3) {
            deltas.push(EntityDelta::Angle3(Angle::read_le_args(
                r,
                ctx.use_float_coords,
            )?));
        }

        if ctx.morebits.contains(EntityBitsMore::Trans) {
            deltas.push(EntityDelta::Trans(u8::read_le(r)?));
        }
        if ctx.morebits.contains(EntityBitsMore::ColourMod) {
            deltas.push(EntityDelta::ColourMod([
                u8::read_le(r)?,
                u8::read_le(r)?,
                u8::read_le(r)?,
            ]));
        }

        Ok(EntityDeltas(deltas))
    }
}
