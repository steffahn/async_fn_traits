# async_fn_traits

[![docs.rs]](https://docs.rs/async_fn_traits)
[![crates.io]](https://crates.io/crates/async_fn_traits)
[![github]](https://github.com/steffahn/async_fn_traits)
[![MIT / Apache 2.0 licensed]](#License)
[![unsafe forbidden]](https://github.com/rust-secure-code/safety-dance/)

[github]: https://img.shields.io/badge/github-steffahn/async__fn__traits-yellowgreen.svg
[crates.io]: https://img.shields.io/crates/v/async_fn_traits.svg
[MIT / Apache 2.0 licensed]: https://img.shields.io/crates/l/async_fn_traits.svg
[docs.rs]: https://docs.rs/async_fn_traits/badge.svg
[unsafe forbidden]: https://img.shields.io/badge/unsafe-forbidden-success.svg

Trait synonyms for `Fn[…]`-trait bounds returning futures.

E.g. a 2-argument function `async fn foo(x: Bar, y: Baz) -> Qux` will implement
<code>[AsyncFn2](https://docs.rs/async_fn_traits/0.1/async_fn_traits/trait.AsyncFn2.html)<Bar, Baz, Output = Qux></code>.

## License
Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
