// ported from: https://github.com/QW-Group/mvdparser/blob/master/src/fragfile.dat
pub const WILDCARD: &str = r#""??""#;

pub const PLAYER_DEATHS: [&str; 17] = [
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

pub const PLAYER_SUICIDES_BY_WEAPON: [&str; 11] = [
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

pub const PLAYER_SUICIDE: &str = " suicides";

pub const TEAMKILLS: [&str; 6] = [
    " squished a teammate",
    " mows down a teammate",
    " checks his glasses",
    " checks her glasses",
    " gets a frag for the other team",
    " loses another friend",
];

pub const X_FRAGS_Y: [&str; 3] = [
    //
    r#" stomps "#,
    r#" squishes "#,
    r#" rips "??" a new one"#,
];

pub const Y_FRAGS_X: [&str; 27] = [
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
];
