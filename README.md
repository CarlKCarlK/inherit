# Rust "Inheritance" Example Roadmap

Rust has no class inheritance. Instead, it offers several smaller mechanisms that cover much of what inheritance is often used for: behavior reuse, method availability, and polymorphism.

## Behavior Reuse

1. `1_default_methods`
   File: `examples/1_default_methods.rs`
   Command: `cargo run --example 1_default_methods`

2. `2_supertraits`
   File: `examples/2_supertraits.rs`
   Command: `cargo run --example 2_supertraits`

3. `3_blanket_impls`
   File: `examples/3_blanket_impls.rs`
   Command: `cargo run --example 3_blanket_impls`

4. `4_derive_generated_impls`
   File: `examples/4_derive_generated_impls.rs`
   Command: `cargo run --example 4_derive_generated_impls`
   Note: `derive` does not inherit behavior; it generates trait implementations.

5. `5_macro_generated_impl_reuse`
   File: `examples/5_macro_generated_impl_reuse.rs`
   Command: `cargo run --example 5_macro_generated_impl_reuse`

## Method Availability

6. `6_deref_lookup`
   File: `examples/6_deref_lookup.rs`
   Command: `cargo run --example 6_deref_lookup`

7. `7_extension_traits`
   File: `examples/7_extension_traits.rs`
   Command: `cargo run --example 7_extension_traits`
   Note: Once `usize` implements `UsizeExtensions`, values of type `usize` gain the extension methods wherever the trait is in scope.

8. `8_constraint_gated_methods`
   File: `examples/8_constraint_gated_methods.rs`
   Command: `cargo run --example 8_constraint_gated_methods`

## Polymorphism

9. `9_trait_objects`
   File: `examples/9_trait_objects.rs`
   Command: `cargo run --example 9_trait_objects`

## Suggested Learning Order

Run them in the order listed.
