#![no_std]
#![forbid(unsafe_code)]
// reasonable clippy categories
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
// reasonable clippy::restriction lints
#![warn(
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::default_numeric_fallback,
    clippy::else_if_without_else,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::exit,
    clippy::expect_used,
    clippy::filetype_is_file,
    clippy::float_arithmetic,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::modulo_arithmetic,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::pattern_type_mismatch,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_to_string,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm
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
// reasonable rustdoc lints
#![warn(
    rustdoc::missing_crate_level_docs,
    rustdoc::missing_doc_code_examples,
    rustdoc::private_doc_tests,
    rustdoc::invalid_html_tags
)]

//! [![crates.io]](https://crates.io/crates/async_fn_traits)
//! [![github]](https://github.com/steffahn/async_fn_traits)
//! [![MIT / Apache 2.0 licensed]](https://github.com/steffahn/async_fn_traits#License)
//! [![unsafe forbidden]](https://github.com/rust-secure-code/safety-dance/)
//!
//! Module-level documentation goes here!!
//!
//! [github]: https://img.shields.io/badge/github-steffahn/async__fn__traits-yellowgreen.svg
//! [crates.io]: https://img.shields.io/crates/v/async_fn_traits.svg
//! [MIT / Apache 2.0 licensed]: https://img.shields.io/crates/l/async_fn_traits.svg
//! [docs.rs]: https://docs.rs/async_fn_traits/badge.svg
//! [unsafe forbidden]: https://img.shields.io/badge/unsafe-forbidden-success.svg
//!
//! Trait synonyms for `Fn[…]`-trait bounds of functions returning futures.
//!
//! E.g. a 2-argument function `async fn foo(x: Bar, y: Baz) -> Qux` will implement `AsyncFn2<Bar, Baz, Output = Qux>`.
//!
//! _TODO: this crate is still undocumented._

use core::future::Future;
use paste::paste;

#[allow(clippy::missing_docs_in_private_items)]
macro_rules! define_async_fn_traits {
    ($($J:literal)+) => {
        define_async_fn_traits!{
            [Once][] $($J)+
        }
        define_async_fn_traits!{
            [Mut][] $($J)+
        }
        define_async_fn_traits!{
            [][] $($J)+
        }
    };
    ([$($FNTYPE:ident)?][$($I:literal)*] $N:literal $($J:literal)*) => {
        paste!{
            pub trait [<AsyncFn $($FNTYPE)? $N>]<$([<Arg $I>]),*>
                : [<Fn $($FNTYPE)?>]($([<Arg $I>]),*) -> <Self as [<AsyncFn $($FNTYPE)? $N>]<$([<Arg $I>]),*>>::OutputFuture
            {
                type OutputFuture: Future<Output = <Self as [<AsyncFn $($FNTYPE)? $N>]<$([<Arg $I>]),*>>::Output>;
                type Output;
            }
            impl<F: ?Sized, Fut, $([<Arg $I>]),*> [<AsyncFn $($FNTYPE)? $N>]<$([<Arg $I>]),*> for F
            where
                F: [<Fn $($FNTYPE)?>]($([<Arg $I>]),*) -> Fut,
                Fut: Future,
            {
                type OutputFuture = Fut;
                type Output = Fut::Output;
            }
        }
        define_async_fn_traits!{
            [$($FNTYPE)?][$($I)* $N] $($J)*
        }
    };
    ([$($FNTYPE:ident)?][$($I:literal)*]) => {};
}

define_async_fn_traits!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
    17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);
