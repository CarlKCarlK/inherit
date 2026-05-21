set shell := ["bash", "-euc"]

_require-uv:
	@command -v uv >/dev/null 2>&1 || { \
		echo "Missing required tool: uv" >&2; \
		echo "Install uv, then add the slide dependencies with:" >&2; \
		echo "  uv pip install Pillow python-pptx Pygments python-pptx-merger" >&2; \
		exit 127; \
	}

# Generate walkthrough slides for one example number.
# Example: just generate 3
generate example_number: _require-uv
    @example_file=$(ls examples/{{example_number}}_*.rs | head -n 1); \
    example_name=$(basename "$example_file" .rs); \
    cargo run --example "$example_name" || { echo "Example run failed (assert or runtime error): $example_name" >&2; exit 1; }; \
    uv run python scripts/build_example_slides.py --example {{example_number}} --project-root .

# Generate full talk: intro + examples 1..9 + outro.
# Requires docs/intro.pptx and docs/outro.pptx to exist.
generate-talk: _require-uv
    @for number in 1 2 3 4 5 6 7 8 9; do \
        just generate "$number"; \
    done
    @test -f docs/intro.pptx || { echo "Missing docs/intro.pptx" >&2; exit 1; }
    @test -f docs/outro.pptx || { echo "Missing docs/outro.pptx" >&2; exit 1; }
    uv run python scripts/assemble_talk.py --output target/slides/rust-inheritance.pptx \
      docs/intro.pptx \
      target/slides/example1_walkthrough.pptx \
      target/slides/example2_walkthrough.pptx \
      target/slides/example3_walkthrough.pptx \
      target/slides/example4_walkthrough.pptx \
      target/slides/example5_walkthrough.pptx \
      target/slides/example6_walkthrough.pptx \
      target/slides/example7_walkthrough.pptx \
      target/slides/example8_walkthrough.pptx \
      target/slides/example9_walkthrough.pptx \
      docs/outro.pptx
