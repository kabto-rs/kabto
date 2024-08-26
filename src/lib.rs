#![allow(
    non_snake_case,
    non_camel_case_types,
    incomplete_features,
)]

#![feature(
    generic_const_exprs,
    const_trait_impl,
    extend_one,
    adt_const_params,
    variant_count,
    tuple_trait,
    fn_traits,
    unboxed_closures,
)]

mod dsl;
mod vdom;

pub use dsl::{tag, nodes::*};
