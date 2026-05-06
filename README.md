# Rust "Inheritance" Example Roadmap

Rust has no class inheritance, but it supports several inheritance-like mechanisms for behavior reuse, method resolution, and capability propagation.

## Behavior Reuse

1. `default_methods`  
   File: `examples/default_methods.rs`  
   Command: `cargo run --example default_methods`

2. `supertraits`  
   File: `examples/supertraits.rs`  
   Command: `cargo run --example supertraits`

3. `blanket_impls`  
   File: `examples/blanket_impls.rs`  
   Command: `cargo run --example blanket_impls`

4. `derive_reuse`  
   File: `examples/derive_reuse.rs`  
   Command: `cargo run --example derive_reuse`

## Method Resolution

5. `deref_lookup`  
   File: `examples/deref_lookup.rs`  
   Command: `cargo run --example deref_lookup`

6. `trait_lookup_scope`  
   File: `examples/trait_lookup_scope.rs`  
   Command: `cargo run --example trait_lookup_scope`

7. `extension_traits`  
   File: `examples/extension_traits.rs`  
   Command: `cargo run --example extension_traits`

8. `ufcs_disambiguation`  
   File: `examples/ufcs_disambiguation.rs`  
   Command: `cargo run --example ufcs_disambiguation`

## Type Capability Propagation

9. `auto_traits`  
   File: `examples/auto_traits.rs`  
   Command: `cargo run --example auto_traits`

10. `trait_objects`  
    File: `examples/trait_objects.rs`  
    Command: `cargo run --example trait_objects`

11. `composition_delegation`  
    File: `examples/composition_delegation.rs`  
    Command: `cargo run --example composition_delegation`

## Suggested Learning Order

Run them in numeric order. The first four establish behavior reuse, the middle four explain how method calls resolve, and the last three show type-level propagation and runtime polymorphism.
