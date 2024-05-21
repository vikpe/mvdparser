// ported from: https://github.com/QW-Group/mvdparser/blob/master/src/fragfile.dat
pub const WILDCARD: &str = r#""??""#;

pub const X_DEATH: [&str; 17] = [
    " sleeps with the fishes",
    " sucks it down",
    " gulped a load of slime",
    " can't exist on slime alone",
    " burst into flames",
    " turned into hot slag",
    " visits the Volcano God",
    " cratered",
    " fell to his death",
    " fell to her death",
    " blew up",
    " was spiked",
    " was zapped",
    " ate a lavaball",
    " died",
    " tried to leave",
    " was squished",
];

pub const X_SUICIDE_BY_WEAPON: [&str; 11] = [
    " tries to put the pin back in",
    " becomes bored with life",
    " discovers blast radius",
    " electrocutes himself",
    " electrocutes herself",
    " railcutes himself",
    " railcutes herself",
    " discharges into the slime",
    " discharges into the lava",
    " discharges into the water",
    " heats up the water",
];

pub const X_SUICIDE: &str = " suicides";

pub const X_TEAMKILL_UNKNOWN: [&str; 6] = [
    " squished a teammate",
    " mows down a teammate",
    " checks his glasses",
    " checks her glasses",
    " gets a frag for the other team",
    " loses another friend",
];

pub const UNKNOWN_TEAMKILL_X: [&str; 6] = [
    " was telefragged by his teammate",
    " was telefragged by her teammate",
    " was crushed by his teammate", // ktpro stomp tk
    " was crushed by her teammate", // ktpro stomp tk
    " was jumped by his teammate",  // ktx addon for ktpro stomp tk
    " was jumped by her teammate",  // ktx addon for ktpro stomp tk
];

pub const X_FRAG_Y: [&str; 3] = [
    //
    r#" stomps "#,
    r#" squishes "#,
    r#" rips "??" a new one"#,
];

pub const Y_FRAG_X: [&str; 28] = [
    r#" was ax-murdered by "#,
    r#" softens "??"'s fall"#,
    r#" tried to catch "#,
    r#" was crushed by "#,
    r#" was jumped by "#,
    r#" chewed on "??"'s boomstick"#,
    r#" was body pierced by "#,
    r#" was nailed by "#,
    r#" was railed by "#,
    r#" was telefragged by "#,
    r#" accepts "??"'s discharge"#,
    r#" drains "??"'s batteries"#,
    r#" was lead poisoned by "#,
    r#" accepts "??"'s shaft"#,
    r#" ate 2 loads of "??"'s buckshot"#,
    r#" was perforated by "#,
    r#" was punctured by "#,
    r#" was ventilated by "#,
    r#" ate 8 loads of "??"'s buckshot"#,
    r#" gets a natural disaster from "#,
    r#" rides "??"'s rocket"#,
    r#" was gibbed by "??"'s rocket"#,
    r#" was straw-cuttered by "#,
    r#" eats "??"'s pineapple"#,
    r#" was gibbed by "??"'s grenade"#,
    r#" was brutalized by "??"'s quad rocket"#,
    r#" was smeared by "??"'s quad rocket"#,
    r#" was hooked by "#,
];

pub const X_CAPTURE_FLAG: [&str; 4] = [
    " captured the RED flag!",
    " ãáðôõòåä the ÒÅÄ flag!",
    " captured the BLUE flag!",
    " ãáðôõòåä the ÂÌÕÅ flag!",
];

pub const X_RETURN_FLAG_ASSIST: [&str; 2] = [
    " gets an assist for returning his flag!",
    " gets an assist for fragging the flag carrier!",
];

pub const X_RETURN_FLAG: [&str; 4] = [
    " returned the RED flag!",
    " returned the ÒÅÄ flag!",
    " returned the BLUE flag!",
    " returned the ÂÌÕÅ flag!",
];

pub const X_DEFEND_FLAG: [&str; 4] = [
    " defends the RED flag",
    " defends the ÒÅÄ flag",
    " defends the BLUE flag",
    " defends the ÂÌÕÅ flag",
];

pub const X_DEFEND_CARRIER: [&str; 4] = [
    " defends RED's flag carrier",
    " defends ÒÅÄ's flag carrier",
    " defends BLUE's flag carrier",
    " defends ÂÌÕÅ's flag carrier",
];

pub const X_DEFEND_CARRIER_VS_AGGRESSIVE: [&str; 4] = [
    " defends RED's flag carrier against an aggressive enemy",
    " defends ÒÅÄ's flag carrier against an aggressive enemy",
    " defends BLUE's flag carrier against an aggressive enemy",
    " defends ÂÌÕÅ's flag carrier against an aggressive enemy",
];
