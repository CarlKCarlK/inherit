# Rust "Inheritance" Example Roadmap

Rust has no class inheritance. Instead, it offers several smaller mechanisms that cover much of what inheritance is often used for: behavior reuse, method resolution, capability propagation, and polymorphism.

## Behavior Reuse

1. `1a_default_methods`  
   File: `examples/1a_default_methods.rs`  
   Command: `cargo run --example 1a_default_methods`

2. `1b_supertraits`  
   File: `examples/1b_supertraits.rs`  
   Command: `cargo run --example 1b_supertraits`

3. `1c_blanket_impls`  
   File: `examples/1c_blanket_impls.rs`  
   Command: `cargo run --example 1c_blanket_impls`

4. `1d_derive_reuse`  
   File: `examples/1d_derive_reuse.rs`  
   Command: `cargo run --example 1d_derive_reuse`  
   Note: `derive` does not inherit behavior; it generates trait implementations.

## Method Resolution

5. `2a_deref_lookup`  
   File: `examples/2a_deref_lookup.rs`  
   Command: `cargo run --example 2a_deref_lookup`

6. `2b_extension_traits`  
   File: `examples/2b_extension_traits.rs`  
   Command: `cargo run --example 2b_extension_traits`

7. `2c_ufcs_disambiguation`  
   File: `examples/2c_ufcs_disambiguation.rs`  
   Command: `cargo run --example 2c_ufcs_disambiguation`

8. `2d_constraint_gated_methods`  
   File: `examples/2d_constraint_gated_methods.rs`  
   Command: `cargo run --example 2d_constraint_gated_methods`

## Capability Propagation and Polymorphism

9. `3a_auto_traits`  
   File: `examples/3a_auto_traits.rs`  
   Command: `cargo run --example 3a_auto_traits`

10. `3b_trait_objects`  
    File: `examples/3b_trait_objects.rs`  
    Command: `cargo run --example 3b_trait_objects`

11. `3c_composition_delegation`  
    File: `examples/3c_composition_delegation.rs`  
    Command: `cargo run --example 3c_composition_delegation`

## Suggested Learning Order

Run them in the order listed.
