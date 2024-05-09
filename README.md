# mvdparser [![Test](https://github.com/vikpe/mvdparser/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/mvdparser/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/mvdparser)](https://crates.io/crates/mvdparser) [![docs.rs](https://img.shields.io/docsrs/mvdparser)](https://docs.rs/mvdparser/)

> Extract information from QuakeWorld MVD demos

## Functions

### duration

```rust
mvdparser::countdown_duration(&data) // Option<Duration>
mvdparser::match_duration(&data)     // Option<Duration>
mvdparser::demo_duration(&data)      // Option<Duration>
```

### timestamp

Gets timestamp from `epoch` in serverinfo (preferred) _or_ from `matchdate` print.

```rust
mvdparser::timestamp(&data) // Option<DateTime<Utc>>
```

### clients

```rust
mvdparser::clients(&data) // Result<Vec<Client>>

struct Client {
    pub number: u8,
    pub name: String,
    pub team: String,
    pub color: [u8; 2],
    pub is_spectator: bool,
    pub is_bot: bool,
}
```

### players

```rust
mvdparser::players(&data) // Result<Vec<Player>>

struct Player {
    pub name: String,
    pub team: String,
    pub color: [u8; 2],
    pub frags: i32,
    pub ping: u32,
    pub is_bot: bool,
}
```

### teams

```rust
mvdparser::teams(&data) // Result<Vec<Team>>

struct Team {
    pub name: String,
    pub color: [u8; 2],
    pub frags: i32,
    pub ping: u32,
    pub players: Vec<Player>,
}
```

### prints

```rust
mvdparser::prints(&data) // Vec<Print>

struct Print {
    pub id: PrintId,
    pub content: Vec<u8>,
}
```

### ktxstats

See crate [ktxstats](https://github.com/vikpe/ktxstats) for full definition.

```rust
mvdparser::ktxstats_v3(&data)     // Option<KtxstatsV3>

pub struct KtxstatsV3 {
    pub version: i32,
    pub date: DateTime<Utc>,
    pub map: String,
    pub hostname: String,
    pub ip: String,
    pub port: i32,
    pub mode: String,
    pub tl: i32,
    pub dm: i32,
    pub tp: i32,
    pub duration: i32,
    pub demo: String,
    pub teams: Vec<String>,
    pub players: Vec<Player>,
}
```

### serverinfo

See crate [quake_serverinfo](https://github.com/vikpe/quake_serverinfo) for more info.

```rust
mvdparser::serverinfo(&data)        // Option<Serverinfo>

struct Serverinfo {
    pub admin: Option<String>,
    pub deathmatch: Option<i32>,
    pub epoch: Option<i32>,
    pub fpd: Option<i32>,
    pub fraglimit: Option<i32>,
    pub gamedir: Option<String>,
    pub hostname: Option<String>,
    pub ktxmode: Option<String>,
    pub ktxver: Option<String>,
    pub map: Option<String>,
    pub matchtag: Option<String>,
    pub maxclients: Option<i32>,
    pub maxfps: Option<i32>,
    pub maxspectators: Option<i32>,
    pub mode: Option<String>,
    pub needpass: Option<i32>,
    pub pm_ktjump: Option<i32>,
    pub progs: Option<String>,
    pub qvm: Option<String>,
    pub status: Option<String>,
    pub serverdemo: Option<String>,
    pub sv_antilag: Option<i32>,
    pub teamplay: Option<i32>,
    pub timelimit: Option<i32>,
    pub version: Option<String>,
    pub z_ext: Option<i32>,
}

```

