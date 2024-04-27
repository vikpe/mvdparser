# mvdparser [![Test](https://github.com/vikpe/mvdparser/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/mvdparser/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/mvdparser)](https://crates.io/crates/mvdparser) [![docs.rs](https://img.shields.io/docsrs/mvdparser)](https://docs.rs/mvdparser/)

> Extract information from QuakeWorld MVD demos

## Benchmarks

Run on [tests/files/4on4_oeks_vs_tsq\[dm2\]20240426-1716.mvd](./tests/files/4on4_oeks_vs_tsq[dm2]20240426-1716.mvd) (14
mb).

```
matchdate           time:   [1.6835 µs 1.6861 µs 1.6896 µs]
                    thrpt:  [7704.6 GiB/s 7720.2 GiB/s 7732.2 GiB/s]

ktxstats            time:   [3.8596 µs 3.8624 µs 3.8658 µs]
                    thrpt:  [3367.3 GiB/s 3370.3 GiB/s 3372.7 GiB/s

serverinfo          time:   [110.77 ns 111.69 ns 112.57 ns]
                    thrpt:  [115642 GiB/s 116548 GiB/s 117515 GiB/s]
```
