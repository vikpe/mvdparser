# mvdparser [![Test](https://github.com/vikpe/mvdparser/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/mvdparser/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/mvdparser)](https://crates.io/crates/mvdparser) [![docs.rs](https://img.shields.io/docsrs/mvdparser)](https://docs.rs/mvdparser/)

> Extract information from QuakeWorld MVD demos

## Functions

### duration

```rust
mvdparser::countdown_duration(&demo_path) // Option<Duration>
mvdparser::match_duration(&demo_path) // Option<Duration>
mvdparser::demo_duration(&demo_path) // Option<Duration>
```

### ktxstats

```rust
mvdparser::ktxstats(&demo_path) // Option<String>
```

### serverinfo

See crate [quake_serverinfo](https://github.com/vikpe/quake_serverinfo) for `Serverinfo` definition.

```rust
mvdparser::serverinfo(&demo_path)        // Option<Serverinfo>
mvdparser::serverinfo_string(&demo_path) // Option<String>
```

### timestamp

Gets timestamp - from `epoch` (preferred, found in serverinfo) _or_ from `matchdate`.

```rust
mvdparser::timestamp(&demo_path) // Option<DateTime<Utc>>
```
