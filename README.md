# Rust "Inheritance" Examples

Rust does not have class inheritance. This repo shows nine examples of how how Rust provides the same benefits (shared code, types sharing a role) via other mechnisims.

* [Video](https://www.youtube.com/watch?v=3IyKC5EtNkM) Presentation to the Seattle Rust Users Group
* [Slides](https://bit.ly/rustinherit)

## Roadmap

1. `1_default_methods`
   Shows trait default methods as reusable behavior shared by multiple implementors.
   Run: `cargo run --example 1_default_methods`

2. `2_supertraits`
   Shows a supertrait (`ServoPlayer: Servo`) where one capability extends another and can be used anywhere the base trait is required.
   Run: `cargo run --example 2_supertraits`

3. `3_extension_traits`
   Shows extension traits adding methods to an existing type when inherent impls are not allowed.
   Run: `cargo run --example 3_extension_traits`

4. `4_derive_generated_impls`
   Shows `derive` generating standard trait implementations like `Default`, `Clone`, `Copy`, and ordering traits.
   Run: `cargo run --example 4_derive_generated_impls`

5. `5_deref_lookup`
   Shows wrapper types exposing methods from an inner type through `Deref` and `DerefMut` lookup.
   Run: `cargo run --example 5_deref_lookup`

6. `6_blanket_impls`
   Shows a blanket impl adding one shared method to every type that satisfies an iterator-shaped constraint.
   Run: `cargo run --example 6_blanket_impls`

7. `7_macro_generated_impl_reuse`
   Shows macros reusing the same trait implementation pattern across many concrete types.
   Run: `cargo run --example 7_macro_generated_impl_reuse`

8. `8_constraint_gated_methods`
   Shows methods that only exist for specific instantiations of a type, here with a const generic constraint.
   Run: `cargo run --example 8_constraint_gated_methods`

9. `9_method_level_constraints`
   Shows method-level trait bounds so the type stays broadly usable while only selected methods require serialization support.
   Run: `cargo run --example 9_method_level_constraints`

## Suggested Order

Read and run them in numeric order. The first seven focus on behavior reuse patterns, then the last two narrow in on method availability through constraints.

## PowerPoint Slide Generation (via Python)

Prerequisites:

- `uv`
- `npm install`
- `uv venv`
- `uv pip install Pillow python-pptx pygments python-pptx-merger`

Generate one walkthrough deck:

- `just generate 3`

Generate the full talk deck:

- `just generate-talk`

Outputs:

- `target/slides/example3_walkthrough.pptx`
- `target/slides/rust-inheritance.pptx`
