# mvdparser [![Test](https://github.com/vikpe/mvdparser/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/mvdparser/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/mvdparser)](https://crates.io/crates/mvdparser) [![docs.rs](https://img.shields.io/docsrs/mvdparser)](https://docs.rs/mvdparser/)

> Extract information from QuakeWorld MVD demos

## Functions

## ktxstats

```
mvdparser::ktxstats(&demo_path) -> Option<String>
```

## matchdate

```
mvdparser::matchdate(&demo_path) -> Option<DateTime<Utc>>
mvdparser::matchdate_string(&demo_path) -> Option<String>
```

### serverinfo

See crate [quake_serverinfo](https://github.com/vikpe/quake_serverinfo) for `Serverinfo` definition.

```
mvdparser::serverinfo(&demo_path) -> Option<Serverinfo>
mvdparser::serverinfo_string(&demo_path) -> Option<String>
```