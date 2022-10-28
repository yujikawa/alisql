# alisql
[![test](https://github.com/yujikawa/alisql/actions/workflows/test.yml/badge.svg)](https://github.com/yujikawa/alisql/actions/workflows/test.yml)
[![alisql at crates.io](https://img.shields.io/crates/v/alisql.svg)](https://crates.io/crates/alisql)
[![alisql at docs.rs](https://docs.rs/alisql/badge.svg)](https://docs.rs/alisql)

This is library to analize SQL with jinja template.

```rust
use alias;
let d = alias::get_dependencies("sqls") // You chose directory name
```