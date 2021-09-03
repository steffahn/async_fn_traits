#![no_std]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
// reasonable restriction lints
#![warn(
    clippy::clone_on_ref_ptr,
    clippy::float_arithmetic,
    clippy::float_cmp_const,
    clippy::integer_arithmetic,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::multiple_inherent_impl,
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::shadow_unrelated,
    clippy::string_add,
    clippy::unimplemented,
    clippy::use_debug
)]
// reasonable rustc lints
#![warn(
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

//! [![crates.io]](https://crates.io/crates/template_crate)
//! [![github]](https://github.com/steffahn/template_crate)
//! [![MIT / Apache 2.0 licensed]](https://github.com/steffahn/template_crate#License)
//! [![unsafe forbidden]](https://github.com/rust-secure-code/safety-dance/)
//!
//! Module-level documentation goes here!!
//!
//! [github]: https://img.shields.io/badge/github-steffahn/template__crate-yellowgreen.svg
//! [crates.io]: https://img.shields.io/crates/v/template_crate.svg
//! [MIT / Apache 2.0 licensed]: https://img.shields.io/crates/l/template_crate.svg
//! [docs.rs]: https://docs.rs/template_crate/badge.svg
//! [unsafe forbidden]: https://img.shields.io/badge/unsafe-forbidden-success.svg
