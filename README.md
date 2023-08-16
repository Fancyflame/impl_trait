# Impl Trait

## Motivation

Up to this crate was created, feature `impl_trait_in_assoc_type` is not stable, so we cannot have a trait with asynchronous function. Although we already have [async-trait](https://crates.io/crates/async-trait), it has a shortcoming: *each `Future` stores on heap*. That is to say, more asynchronous function you call, more allocation caused. This crate is exists to settle this problem.

But notice, this crate is just for experiment.

## Implementation Idea

This crate provided a box with a compile-time determined memory layout. That is to say, the inner type is actually has a known size, but the type is elimated and can be casted to a unsized type (e.g. DST) at runtime.

## Type Coercion

Unfortunately, due to feature `coerce_unsized` is not stable, we cannot coerce the boxed value to any trait object. If needed, you should implement `Coerce` trait in `coerce` module manually with unsafe code yourself.

Only `Future` is implemented by default.