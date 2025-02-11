use binrw::{BinRead, BinResult, Endian};
use std::io::{Read, Seek};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Angle(f32);

impl BinRead for Angle {
    type Args<'a> = bool;
    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _: Endian,
        float_coords: Self::Args<'_>,
    ) -> BinResult<Self> {
        Ok(Angle(if float_coords {
            (u16::read_le(reader)? as f32) * 360f32 / 65536f32
        } else {
            (i8::read_le(reader)? as f32) * 360f32 / 256f32
        }))
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Coord(f32);

impl BinRead for Coord {
    type Args<'a> = bool;
    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _: Endian,
        float_coords: Self::Args<'_>,
    ) -> BinResult<Self> {
        Ok(Coord(if float_coords {
            f32::from_bits(u32::read_le(reader)?)
        } else {
            i16::read_le(reader)? as f32 / 8f32
        }))
    }
}
