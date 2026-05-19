set shell := ["bash", "-cu"]

# Generate walkthrough slides for one example number.
# Example: just generate 3
generate example_number:
    @example_file=$(ls examples/{{example_number}}_*.rs | head -n 1); \
    example_name=$(basename "$example_file" .rs); \
    cargo run --example "$example_name" || { echo "Example run failed (assert or runtime error): $example_name" >&2; exit 1; }; \
    uv run python scripts/build_example_slides.py --example {{example_number}} --project-root .

# Generate full talk: intro + examples 1..9 + outro.
# Requires docs/intro.pptx and docs/outro.pptx to exist.
generate-talk:
    @for number in 1 2 3 4 5 6 7 8 9; do \
        just generate "$number"; \
    done
    @test -f docs/intro.pptx || { echo "Missing docs/intro.pptx" >&2; exit 1; }
    @test -f docs/outro.pptx || { echo "Missing docs/outro.pptx" >&2; exit 1; }
    uv run python scripts/assemble_talk.py --output docs/rust-inheritance.pptx \
      docs/intro.pptx \
      docs/example1_walkthrough.pptx \
      docs/example2_walkthrough.pptx \
      docs/example3_walkthrough.pptx \
      docs/example4_walkthrough.pptx \
      docs/example5_walkthrough.pptx \
      docs/example6_walkthrough.pptx \
      docs/example7_walkthrough.pptx \
      docs/example8_walkthrough.pptx \
      docs/example9_walkthrough.pptx \
      docs/outro.pptx
