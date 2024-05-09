use quake_clientinfo::Clientinfo;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Client {
    pub number: u8,
    pub name: String,
    pub team: String,
    pub color: [u8; 2],
    pub is_spectator: bool,
    pub is_bot: bool,
}

impl From<&Clientinfo> for Client {
    fn from(value: &Clientinfo) -> Self {
        Client {
            number: 0,
            name: value.name.clone().unwrap_or_default(),
            team: value.team.clone().unwrap_or_default(),
            color: [
                value.topcolor.unwrap_or(0) as u8,
                value.bottomcolor.unwrap_or(0) as u8,
            ],
            is_spectator: value.spectator.is_some_and(|v| v != 0),
            is_bot: value.bot.is_some_and(|v| v != 0),
        }
    }
}
