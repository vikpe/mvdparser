use crate::qw::primitives::{Angle, Coord};
use binrw::{BinRead, BinResult, Endian};
use bitflags::bitflags;
use std::io::{Read, Seek};

#[derive(Debug, PartialEq)]
pub struct Playerinfo {
    pub player_number: u8,
    pub flags: u8,
    pub frame: u8,
    pub origin: [Coord; 3],
    pub angles: [Angle; 3],
    pub model: u8,
    pub skinnum: u8,
    pub effects: u8,
    pub weaponframe: u8,
}

impl BinRead for Playerinfo {
    type Args<'a> = ();

    #[rustfmt::skip]
    fn read_options<R: Read + Seek>(
        r: &mut R,
        _: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let player_number = u8::read_le(r)?;
        let flags = u8::read_le(r)?;
        let frame = u8::read_le(r)?;
        let df = DfFlags::from_bits(flags as u16).unwrap();

        let mut origin = [Coord::default(); 3];
        if df.contains(DfFlags::ORIGIN1) { origin[0] = Coord::read_le(r)?; }
        if df.contains(DfFlags::ORIGIN2) { origin[1] = Coord::read_le(r)?; }
        if df.contains(DfFlags::ORIGIN3) { origin[2] = Coord::read_le(r)?; }

        let mut angles = [Angle::default(); 3];
        if df.contains(DfFlags::ANGLE1) {angles[0] = Angle::read_le(r)?;}
        if df.contains(DfFlags::ANGLE2) {angles[1] = Angle::read_le(r)?;}
        if df.contains(DfFlags::ANGLE3) {angles[2] = Angle::read_le(r)?;}

        let model = if df.contains(DfFlags::MODEL) { u8::read_le(r)? } else { 0 };
        let skinnum = if df.contains(DfFlags::SKINNUM) { u8::read_le(r)? } else { 0 };
        let effects = if df.contains(DfFlags::EFFECTS) { u8::read_le(r)? } else { 0 };
        let weaponframe = if df.contains(DfFlags::WEAPONFRAME) { u8::read_le(r)? } else { 0 };

        Ok(Playerinfo {
            player_number,
            flags,
            frame,
            origin,
            angles,
            model,
            skinnum,
            effects,
            weaponframe,
        })
    }
}

bitflags! {
 #[derive(Debug, Eq, PartialEq)]
    pub struct DfFlags: u16 {
        const ORIGIN1 = 1;
        const ORIGIN2 = 1 << 1;
        const ORIGIN3 = 1 << 2;
        const ANGLE1 = 1 << 3;
        const ANGLE2 = 1 << 4;
        const ANGLE3 = 1 << 5;
        const EFFECTS = 1 << 6;
        const SKINNUM = 1 << 7;
        const DEAD = 1 << 8;
        const GIB = 1 << 9;
        const WEAPONFRAME = 1 << 10;
        const MODEL = 1 << 11;
    }
}
