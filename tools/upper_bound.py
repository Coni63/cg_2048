"""Score upper bound per seed.

The tile mass (sum of tiles) only depends on the spawned values, which only depend on
the seed sequence, not on the moves. When the mass has 16 bits set (all >= 2), the board
must hold 16 distinct tiles: full board, no merge -> forced game over. The best possible
score at mass M is F(M) - 4 * (#spawned 4s), F summing (k-1) * 2^k over the bits of M.
"""
import sys

SEEDS = [42, 290797, 10682358, 38333962, 47049887, 11205586, 15242016, 32019767, 46946765,
         4424780, 2524322, 20797492, 28944706, 20969426, 20950077, 8601721, 44677966, 534357,
         970088, 8078305, 5731756, 45283038, 17769313, 41900735, 32506342, 28758123, 25880068,
         41359522, 704563, 29082488]


def F(m):
    return sum((k - 1) << k for k in range(1, 40) if m >> k & 1)


def bound(seed, max_moves=10**6):
    mass, n4, s = 0, 0, seed
    for i in range(max_moves + 2):
        v = 2 if s & 0x10 == 0 else 4
        mass += v
        n4 += v == 4
        s = s * s % 50515093
        if bin(mass).count("1") >= 16:
            return F(mass) - 4 * n4, mass, i - 1
    return None


if __name__ == "__main__":
    seeds = [int(x) for x in sys.argv[1:]] or SEEDS
    total = 0
    for seed in seeds:
        score, mass, moves = bound(seed)
        total += score
        print(f"seed {seed:>9} bound {score:>8} dead mass {mass:>7} after {moves:>6} moves")
    print(f"TOTAL bound {total} ({total / 1e6:.2f}M)")
