use crate::qw::entity::{EntityDeltaContext, EntityDeltas};
use crate::qw::frame::Command;
use crate::qw::message::{
    MessageType, ModelList, SoundList, SpawnBaseline, SpawnStaticSound, UpdateEntertime,
    UpdateFrags, UpdatePacketLoss, UpdatePing, UpdateUserinfo,
};
use crate::qw::playerinfo::Playerinfo;
use crate::qw::protocol::{Mvd1Extensions, ProtocolExtensions};
use crate::qw::server_data::ServerData;
use binrw::io::TakeSeekExt;
use binrw::{BinRead, BinResult, Endian, NullString};
use std::io::{Read, Seek};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Payload {
    SvcCdTrack(u8),
    SvcModelList(ModelList),
    SvcPlayerinfo(Playerinfo),
    SvcServerData(ServerData),
    SvcSoundList(SoundList),
    SvcSpawnBaseline(SpawnBaseline),
    SvcSpawnBaseline2(EntityDeltas),
    SvcSpawnStatic2(EntityDeltas),
    SvcSpawnStaticSound(SpawnStaticSound),
    SvcStuffText(NullString),
    SvcUpdateEnterTime(UpdateEntertime),
    SvcUpdateFrags(UpdateFrags),
    SvcUpdatePacketLoss(UpdatePacketLoss),
    SvcUpdatePing(UpdatePing),
    SvcUpdateUserInfo(UpdateUserinfo),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Frame {
    pub pad: u8,
    pub command: Command,
    pub size: u32,
    pub payload: Vec<Payload>,
}

#[derive(Default)]
#[allow(dead_code)]
pub struct FrameContext {
    pub use_float_coords: bool,
    pub test: bool,
}

impl BinRead for Frame {
    type Args<'a> = &'a FrameContext;

    #[rustfmt::skip]
    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _: Endian,
        ctx: Self::Args<'_>,
    ) -> BinResult<Self> {
        let pad = u8::read(reader)?;
        let command = Command::read(reader)?;
        let size = u32::read_le(reader)?;
        let mut body = reader.take_seek(size as u64);
        let mut payload: Vec<Payload> = Vec::new();

        while let Ok(svc) = MessageType::read_le(&mut body) {
            payload.push(match svc {
                MessageType::CdTrack => Payload::SvcCdTrack(u8::read(&mut body)?),
                MessageType::ModelList => Payload::SvcModelList(ModelList::read_le(&mut body)?),
                MessageType::Playerinfo => Payload::SvcPlayerinfo(Playerinfo::read_le(&mut body)?),
                MessageType::ServerData => Payload::SvcServerData(ServerData::read_le(&mut body)?),
                MessageType::SoundList => Payload::SvcSoundList(SoundList::read_le(&mut body)?),
                MessageType::StuffText => Payload::SvcStuffText(NullString::read(&mut body)?),
                MessageType::SpawnStatic2 => {
                    let entity_ctx = EntityDeltaContext::read_le_args(&mut body, ctx)?;
                    Payload::SvcSpawnStatic2(EntityDeltas::read_le_args(&mut body, &entity_ctx)?)
                }
                MessageType::SpawnBaseline => {
                    Payload::SvcSpawnBaseline(SpawnBaseline::read_le(&mut body)?)
                }
                MessageType::SpawnBaseline2 => {
                    let entity_ctx = EntityDeltaContext::read_le_args(&mut body, ctx)?;
                    Payload::SvcSpawnBaseline2(EntityDeltas::read_le_args(
                        &mut body,
                        &entity_ctx,
                    )?)
                }
                MessageType::SpawnStaticSound => Payload::SvcSpawnStaticSound(SpawnStaticSound::read_le(&mut body)?),
                MessageType::UpdateEnterTime => Payload::SvcUpdateEnterTime(UpdateEntertime::read_le(&mut body)?),
                MessageType::UpdateFrags => Payload::SvcUpdateFrags(UpdateFrags::read_le(&mut body)?),
                MessageType::UpdatePacketLoss => Payload::SvcUpdatePacketLoss(UpdatePacketLoss::read_le(&mut body)?),
                MessageType::UpdatePing => Payload::SvcUpdatePing(UpdatePing::read_le(&mut body)?),
                MessageType::UpdateUserinfo => Payload::SvcUpdateUserInfo(UpdateUserinfo::read_le(&mut body)?),
                _ => panic!("not supported {svc:?} ({})", body.stream_position()?),
            });
        }

        Ok(Self {
            pad,
            command,
            size,
            payload,
        })
    }
}

#[allow(dead_code)]
#[rustfmt::skip]
fn parsestuff<R: Read + Seek>(mut reader: R) -> BinResult<()> {
    let mut ctx = FrameContext {
        use_float_coords: false,
        test: true,
    };
    loop {
        let frame = match Frame::read_le_args(&mut reader, &ctx) {
            Ok(f) => f,
            Err(e) if e.is_eof() => {
                println!("EOF reached, done!");
                break;
            }
            Err(e) => return Err(e),
        };

        // the api of the lib, just match whatever you're interested in
        for payload in frame.payload {
            match payload {
                Payload::SvcModelList(model) => { dbg!(&model); }
                Payload::SvcPlayerinfo(info) => { dbg!(&info);}
                Payload::SvcSoundList(sounds) => { dbg!(&sounds); }
                Payload::SvcStuffText(text) => { dbg!(&text); }
                Payload::SvcServerData(data) => {
                    dbg!(&data);
                    for ext in data.extensions {
                        if let ProtocolExtensions::Mvd(mvd_exts) = ext {
                            ctx.use_float_coords = mvd_exts.contains(Mvd1Extensions::FLOAT_COORDS);
                        }
                    }
                }
                Payload::SvcCdTrack(track) => { dbg!(&track); }
                Payload::SvcSpawnBaseline(spawnbaseline) => { dbg!(&spawnbaseline); }
                Payload::SvcSpawnBaseline2(spawnbaseline) => { dbg!(&spawnbaseline); }
                Payload::SvcSpawnStatic2(spawnstatic) => { dbg!(&spawnstatic); }
                Payload::SvcSpawnStaticSound(staticsound) => { dbg!(&staticsound); }
                Payload::SvcUpdateEnterTime(enter_time) => { dbg!(&enter_time); }
                Payload::SvcUpdateFrags(frags) => { dbg!(&frags); }
                Payload::SvcUpdatePacketLoss(packet_loss) => { dbg!(&packet_loss); }
                Payload::SvcUpdatePing(ping) => { dbg!(&ping); }
                Payload::SvcUpdateUserInfo(user_info) => { dbg!(&user_info); }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn it_works() {
        let mut file = File::open("tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd").unwrap();
        parsestuff(&mut file).unwrap()
    }
}
