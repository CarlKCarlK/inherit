set shell := ["bash", "-cu"]

# Generate walkthrough slides for one example number.
# Example: just generate 3
generate example_number:
    @example_file=$(ls examples/{{example_number}}_*.rs | head -n 1); \
    example_name=$(basename "$example_file" .rs); \
    cargo run --example "$example_name" || { echo "Example run failed (assert or runtime error): $example_name" >&2; exit 1; }; \
    python3 scripts/build_example_slides.py --example {{example_number}} --project-root .
