#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
import subprocess
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont
from pptx import Presentation
from pptx.dml.color import RGBColor
from pptx.util import Inches, Pt
from pygments import lex
from pygments.lexers import RustLexer
from pygments.token import Token


def extract_markdown_parts(puzzle_md: str) -> tuple[str, str, str]:
    title_match = re.search(r"^#\s+(.+)$", puzzle_md, re.M)
    title = title_match.group(1).strip() if title_match else "Puzzle"

    mermaid_match = re.search(r"```mermaid\n([\s\S]*?)\n```", puzzle_md)
    if not mermaid_match:
        raise RuntimeError("No mermaid block found in puzzle markdown")
    mermaid = mermaid_match.group(1).strip() + "\n"

    prose = re.sub(r"^#\s+.+$", "", puzzle_md, flags=re.M)
    prose = re.sub(r"```mermaid[\s\S]*?```", "", prose).strip()
    prose = re.sub(r"\s+", " ", prose)
    return title, prose, mermaid


def render_mermaid_png(mermaid_text: str, out_png: Path, workdir: Path) -> None:
    temp_mmd = workdir / ".tmp_mermaid.mmd"
    temp_mmd.write_text(mermaid_text, encoding="utf-8")
    try:
        cmd = [
            "npx",
            "@mermaid-js/mermaid-cli",
            "-i",
            str(temp_mmd),
            "-o",
            str(out_png),
            "-b",
            "transparent",
        ]
        subprocess.run(cmd, cwd=workdir, check=True)
    finally:
        if temp_mmd.exists():
            temp_mmd.unlink()


def find_existing_mermaid_png(example_number: int, docs_dir: Path) -> Path | None:
    candidates = [
        docs_dir / f"{example_number}_puzzle_diagram.png",
        docs_dir / f"{example_number}_puzzle-1.png",
        docs_dir / f"{example_number}_puzzle.png",
    ]
    for candidate in candidates:
        if candidate.exists():
            return candidate
    return None


def token_color(token_type) -> tuple[int, int, int]:
    # VS Code dark-ish palette
    if token_type in Token.Keyword or token_type in Token.Operator.Word:
        return (197, 134, 192)
    if token_type in Token.Name.Builtin or token_type in Token.Name.Class or token_type in Token.Name.Namespace or token_type in Token.Name.Type:
        return (78, 201, 176)
    if token_type in Token.Literal.String:
        return (206, 145, 120)
    if token_type in Token.Literal.Number:
        return (181, 206, 168)
    if token_type in Token.Comment:
        return (106, 153, 85)
    if token_type in Token.Name.Function:
        return (220, 220, 170)
    return (220, 220, 220)


def token_color_rgb(token_type) -> RGBColor:
    r, g, b = token_color(token_type)
    return RGBColor(r, g, b)


def load_mono_font(size: int) -> ImageFont.FreeTypeFont:
    candidates = [
        "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf",
    ]
    for path in candidates:
        p = Path(path)
        if p.exists():
            return ImageFont.truetype(str(p), size)
    return ImageFont.load_default()


def render_rust_code_png(code: str, out_png: Path, width: int = 1800) -> None:
    font = load_mono_font(24)
    pad_x = 28
    pad_y = 22
    line_gap = 6  # tighter spacing

    lines = code.splitlines()
    draw_dummy = ImageDraw.Draw(Image.new("RGB", (10, 10)))
    line_height = font.getbbox("Ag")[3] - font.getbbox("Ag")[1]
    content_height = len(lines) * line_height + max(0, len(lines) - 1) * line_gap
    height = pad_y * 2 + content_height

    image = Image.new("RGB", (width, height), (30, 30, 30))
    draw = ImageDraw.Draw(image)

    y = pad_y
    lexer = RustLexer()
    for line in lines:
        x = pad_x
        for token_type, value in lex(line, lexer):
            if not value:
                continue
            value = value.replace("\n", "")
            if not value:
                continue
            draw.text((x, y), value, font=font, fill=token_color(token_type))
            x += int(draw.textlength(value, font=font))
        y += line_height + line_gap

    image.save(out_png)


def build_ppt(
    title: str,
    prose: str,
    mermaid_png: Path,
    code_png: Path,
    code_text: str,
    out_pptx: Path,
    code_mode: str,
) -> None:
    prs = Presentation()
    slide_w = prs.slide_width.inches
    slide_h = prs.slide_height.inches

    # Slide 1: Puzzle
    s1 = prs.slides.add_slide(prs.slide_layouts[6])

    title_box = s1.shapes.add_textbox(Inches(0.8), Inches(0.25), Inches(slide_w - 1.6), Inches(0.75))
    ttf = title_box.text_frame
    ttf.clear()
    p = ttf.paragraphs[0]
    p.text = title
    p.font.name = "Calibri"
    p.font.size = Pt(40)
    p.font.bold = True

    prose_box = s1.shapes.add_textbox(Inches(0.8), Inches(1.0), Inches(slide_w - 1.6), Inches(1.0))
    ptf = prose_box.text_frame
    ptf.clear()
    ptf.word_wrap = True
    pp = ptf.paragraphs[0]
    for part in re.split(r"(`[^`]+`)", prose):
        if not part:
            continue
        run = pp.add_run()
        if part.startswith("`") and part.endswith("`"):
            run.text = part[1:-1]
            run.font.name = "Consolas"
        else:
            run.text = part
            run.font.name = "Calibri"
        run.font.size = Pt(16)

    with Image.open(mermaid_png) as img:
        mw, mh = img.size
    ratio = mw / mh
    diagram_top = 2.05
    avail_h = slide_h - diagram_top - 0.35
    avail_w = slide_w - 2.0
    pic_w = min(avail_w, avail_h * ratio)
    pic_h = pic_w / ratio
    left = (slide_w - pic_w) / 2
    s1.shapes.add_picture(str(mermaid_png), Inches(left), Inches(diagram_top), width=Inches(pic_w), height=Inches(pic_h))

    # Slide 2: Rust
    s2 = prs.slides.add_slide(prs.slide_layouts[6])
    t2 = s2.shapes.add_textbox(Inches(0.8), Inches(0.25), Inches(slide_w - 1.6), Inches(0.75))
    t2f = t2.text_frame
    t2f.clear()
    p2 = t2f.paragraphs[0]
    p2.text = "Rust Solution"
    p2.font.name = "Calibri"
    p2.font.size = Pt(40)
    p2.font.bold = True

    code_top = 1.15
    code_avail_h = slide_h - code_top - 0.35
    code_avail_w = slide_w - 1.2

    if code_mode == "image":
        with Image.open(code_png) as img:
            cw, ch = img.size
        c_ratio = cw / ch
        code_w = min(code_avail_w, code_avail_h * c_ratio)
        code_h = code_w / c_ratio
        code_left = (slide_w - code_w) / 2
        s2.shapes.add_picture(
            str(code_png),
            Inches(code_left),
            Inches(code_top),
            width=Inches(code_w),
            height=Inches(code_h),
        )
    else:
        code_box = s2.shapes.add_textbox(
            Inches(0.6),
            Inches(code_top),
            Inches(code_avail_w),
            Inches(code_avail_h),
        )
        text_frame = code_box.text_frame
        text_frame.clear()
        text_frame.word_wrap = False

        for line_index, line in enumerate(code_text.splitlines()):
            paragraph = (
                text_frame.paragraphs[0]
                if line_index == 0
                else text_frame.add_paragraph()
            )
            paragraph.space_after = Pt(0)
            paragraph.space_before = Pt(0)
            paragraph.line_spacing = 1.0

            for token_type, value in lex(line, RustLexer()):
                value = value.replace("\n", "")
                if not value:
                    continue
                run = paragraph.add_run()
                run.text = value
                run.font.name = "Consolas"
                run.font.size = Pt(10)
                run.font.color.rgb = token_color_rgb(token_type)

    prs.save(out_pptx)


def main() -> None:
    parser = argparse.ArgumentParser(description="Build puzzle+solution slides for one example")
    parser.add_argument("--example", type=int, required=True, help="Example number, e.g. 1")
    parser.add_argument("--project-root", type=Path, default=Path.cwd())
    parser.add_argument(
        "--code-mode",
        choices=["image", "editable"],
        default="editable",
        help="Use image-based or editable-text code rendering in the solution slide",
    )
    parser.add_argument(
        "--offline",
        action="store_true",
        help="Skip Mermaid rendering and reuse an existing diagram image from docs/",
    )
    args = parser.parse_args()

    root = args.project_root.resolve()
    examples = root / "examples"
    docs = root / "docs"
    docs.mkdir(exist_ok=True)

    puzzle_md_path = examples / f"{args.example}_puzzle.md"
    solution_rs_candidates = sorted(examples.glob(f"{args.example}_*.rs"))
    if not solution_rs_candidates:
        raise RuntimeError(f"No solution .rs file found for example {args.example}")
    solution_rs_path = solution_rs_candidates[0]

    puzzle_md = puzzle_md_path.read_text(encoding="utf-8")
    solution_code = solution_rs_path.read_text(encoding="utf-8")

    title, prose, mermaid = extract_markdown_parts(puzzle_md)

    mermaid_png = docs / f"{args.example}_puzzle_diagram.png"
    code_png = docs / f"{args.example}_solution_code.png"
    out_pptx = docs / f"example{args.example}_walkthrough.pptx"

    if args.offline:
        existing = find_existing_mermaid_png(args.example, docs)
        if not existing:
            raise RuntimeError(
                "Offline mode requested, but no existing diagram image found in docs/"
            )
        if existing != mermaid_png:
            mermaid_png.write_bytes(existing.read_bytes())
    else:
        render_mermaid_png(mermaid, mermaid_png, root)
    render_rust_code_png(solution_code, code_png)
    build_ppt(
        title,
        prose,
        mermaid_png,
        code_png,
        solution_code,
        out_pptx,
        args.code_mode,
    )

    print(out_pptx)


if __name__ == "__main__":
    main()
