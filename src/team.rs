use crate::player::Player;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Team {
    pub name: String,
    pub color: [u8; 2],
    pub frags: i32,
    pub ping: u32,
    pub players: Vec<Player>,
}
