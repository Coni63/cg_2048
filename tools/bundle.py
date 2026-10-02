"""Bundle the crate into a single file for CodinGame: dist/cg_2048.rs

`mod x;` lines of src/main.rs are replaced by the content of src/x.rs and
#[cfg(test)] modules (always at the end of a file) are dropped.
"""
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parent.parent
SRC = ROOT / "src"


def strip_tests(code: str) -> str:
    idx = code.find("#[cfg(test)]")
    return code if idx < 0 else code[:idx].rstrip() + "\n"


def main() -> None:
    out = []
    for line in (SRC / "main.rs").read_text().splitlines():
        m = re.fullmatch(r"\s*(pub\s+)?mod (\w+);", line)
        if m:
            body = strip_tests((SRC / f"{m.group(2)}.rs").read_text())
            out.append(f"mod {m.group(2)} {{\n#![allow(dead_code)]\n{body}}}")
        else:
            out.append(line)
    code = strip_tests("\n".join(out)) + "\n"
    dist = ROOT / "dist"
    dist.mkdir(exist_ok=True)
    (dist / "cg_2048.rs").write_text(code)
    print(f"dist/cg_2048.rs: {len(code)} chars (CG limit 100000)")


if __name__ == "__main__":
    main()
