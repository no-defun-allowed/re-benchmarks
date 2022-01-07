# Some regular expression benchmarks

Okay, just one benchmark with a few variations. We play with literal
prefixes and submatching to get different behaviours out of regex
engines.

We test the regex `([0-9]+)x([0-9]+)` with and without
submatches/captures, and also with and without literal double quotes
around them (i.e. `"([0-9]+)x([0-9]+)"`), which provides more of a
prefix to optimise for.

The following results were produced on a Ryzen 5 1600 processor, with
the `simd-loops` one-more-re-nightmare branch, SBCL at commit
`2e6c6fac3b8caf618c0e4263244f7c3bed7f85ce`, and Rust 1.55.0.

|            | one-more-re-nightmare | rust-lang/regex |    Hyperscan |
|------------|----------------------:|----------------:|-------------:|
| neither    |          1540 Mchar/s |     603 Mchar/s |  362 Mchar/s |
| submatches |          1510 Mchar/s |     172 Mchar/s |              |
| quotes     |          7260 Mchar/s |    2550 Mchar/s | 1040 Mchar/s |
| both       |          6030 Mchar/s |     236 Mchar/s |              |
