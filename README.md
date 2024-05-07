# mvdparser [![Test](https://github.com/vikpe/mvdparser/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/mvdparser/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/mvdparser)](https://crates.io/crates/mvdparser) [![docs.rs](https://img.shields.io/docsrs/mvdparser)](https://docs.rs/mvdparser/)

> Extract information from QuakeWorld MVD demos

## Functions

### duration

```rust
mvdparser::countdown_duration(&data) // Option<Duration>
mvdparser::match_duration(&data)     // Option<Duration>
mvdparser::demo_duration(&data)      // Option<Duration>
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

See crate [ktxstats](https://github.com/vikpe/ktxstats) for `KtxstatsV3` definition.

```rust
mvdparser::ktxstats_v3(&data)     // Option<KtxstatsV3>
mvdparser::ktxstats_string(&data) // Option<String>
```

### serverinfo

See crate [quake_serverinfo](https://github.com/vikpe/quake_serverinfo) for `Serverinfo` definition.

```rust
mvdparser::serverinfo(&data)        // Option<Serverinfo>
mvdparser::serverinfo_string(&data) // Option<String>
```

### timestamp

Gets timestamp - from `epoch` (preferred, found in serverinfo) _or_ from `matchdate`.

```rust
mvdparser::timestamp(&data) // Option<DateTime<Utc>>
```
