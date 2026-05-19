#!/usr/bin/env python3
from __future__ import annotations

import argparse
from io import BytesIO
from pathlib import Path

try:
    from pptx_merger import Merger
except ImportError as import_error:
    raise SystemExit(
        "Missing dependency: python-pptx-merger.\n"
        "Install it in your uv environment with:\n"
        "  uv pip install python-pptx-merger\n"
        "Then run generate-talk again."
    ) from import_error


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Assemble multiple PPTX files into one deck with embedded media preserved."
    )
    parser.add_argument("--output", required=True, type=Path, help="Output PPTX path.")
    parser.add_argument("inputs", nargs="+", type=Path, help="Input PPTX paths in order.")
    args = parser.parse_args()

    input_paths = [Path(path) for path in args.inputs]
    for input_path in input_paths:
        if not input_path.exists():
            raise FileNotFoundError(f"Missing input deck: {input_path}")

    source_documents = [BytesIO(path.read_bytes()) for path in input_paths]
    merger = Merger()
    merged_document = merger.merge_slides(source_documents)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_bytes(merged_document.getvalue())


if __name__ == "__main__":
    main()
