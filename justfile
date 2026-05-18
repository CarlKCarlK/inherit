set shell := ["bash", "-cu"]

# Generate walkthrough slides for one example number.
# Example: just generate 3
generate example_number:
    python3 scripts/build_example_slides.py --example {{example_number}} --project-root .
