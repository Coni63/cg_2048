"""Run bench configs and compare every game with its theoretical bound.

usage: python tools/sweep.py [--seeds a,b] "<bench args 1>" "<bench args 2>" ...
"""
import pathlib
import re
import subprocess
import sys

from upper_bound import SEEDS, bound

ROOT = pathlib.Path(__file__).resolve().parent.parent
EXE = ROOT / "target" / "release" / "cg_2048.exe"


def run(cfg, seeds):
    args = [str(EXE), "bench", "--threads", "12", "--seeds", ",".join(map(str, seeds))] + cfg.split()
    out = subprocess.run(args, capture_output=True, text=True).stdout
    return {int(m[1]): int(m[2]) for m in re.finditer(r"seed\s+(\d+) score\s+(\d+)", out)}, out


def main():
    argv = sys.argv[1:]
    seeds = SEEDS
    if argv and argv[0] == "--seeds":
        seeds = [int(s) for s in argv[1].split(",")]
        argv = argv[2:]
    bounds = {s: bound(s)[0] for s in seeds}
    for cfg in argv:
        scores, out = run(cfg, seeds)
        total = sum(scores.values())
        missed = [s for s in seeds if scores.get(s, 0) < bounds[s] - 1000]
        flags = " CAPPED" if "CAPPED" in out else ""
        flags += " INVALID" if "INVALID" in out else ""
        print(f"{total / 1e6:6.2f}M gap {(sum(bounds.values()) - total) / 1e6:5.2f}M "
              f"missed {len(missed):2} {missed}{flags} | {cfg}", flush=True)


if __name__ == "__main__":
    main()
