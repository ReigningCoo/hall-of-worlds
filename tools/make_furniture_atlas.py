"""Makes crates/client/assets/furniture_atlas.png: grey pictures of wood grain
and stone that furniture is drawn from, each tinted by its wood's or stone's
colour in the client (`dungeon_map::furniture`). Wood's grain runs down the
picture (along v), and every tile repeats seamlessly both ways, so a plank
longer than a cell carries on without a seam. The order must match
`furniture::atlas` in the client. Run from the repository's root:
python tools/make_furniture_atlas.py (needs Pillow). Everything is drawn
here from seeded noise: no third-party pictures."""

import math
import random

from PIL import Image

OUT = "crates/client/assets/furniture_atlas.png"
COLUMNS, ROWS, SIZE = 4, 3, 64
TAU = 2 * math.pi


def lattice(period, seed):
    rng = random.Random(seed)
    return [[rng.random() for _ in range(period)] for _ in range(period)]


def noise(grid, x, y):
    """Smooth value noise on a wrapping lattice: x and y from 0 to 1 repeat."""
    period = len(grid)
    gx, gy = x * period, y * period
    x0, y0 = math.floor(gx), math.floor(gy)
    fx, fy = gx - x0, gy - y0
    sx, sy = fx * fx * (3 - 2 * fx), fy * fy * (3 - 2 * fy)

    def at(i, j):
        return grid[j % period][i % period]

    top = at(x0, y0) * (1 - sx) + at(x0 + 1, y0) * sx
    bottom = at(x0, y0 + 1) * (1 - sx) + at(x0 + 1, y0 + 1) * sx
    return top * (1 - sy) + bottom * sy


def octaves(grids, x, y):
    """Noise from several lattices, finer ones weaker: 0 to 1."""
    total, weight, scale = 0.0, 0.0, 1.0
    for grid in grids:
        total += noise(grid, x, y) * scale
        weight += scale
        scale /= 2
    return total / weight


def picture(shade):
    """A tile from `shade(u, v)`, 0 dark to 1 light, mapped to light greys
    (so tinting keeps most of the colour)."""
    out = Image.new("L", (SIZE, SIZE))
    for py in range(SIZE):
        for px in range(SIZE):
            value = max(0.0, min(1.0, shade((px + 0.5) / SIZE, (py + 0.5) / SIZE)))
            out.putpixel((px, py), int(120 + value * 135))
    return out


def straight(seed):
    """Ash: fine, straight grain lines with a little drift."""
    warp = [lattice(4, seed), lattice(8, seed + 1)]
    fine = [lattice(32, seed + 2)]

    def shade(u, v):
        drift = (octaves(warp, u, v) - 0.5) * 0.05
        line = 0.5 + 0.5 * math.sin(TAU * (u * 11 + drift * 11))
        return 0.75 - 0.35 * line**4 - 0.1 * noise(fine[0], u, v * 0.25)

    return picture(shade)


def coarse(seed):
    """Oak: coarse grain with dark pores, and pale ray flecks across it."""
    warp = [lattice(4, seed), lattice(8, seed + 1)]
    pores = lattice(32, seed + 2)
    rng = random.Random(seed + 3)
    flecks = [(rng.random(), rng.random(), 0.02 + rng.random() * 0.03) for _ in range(9)]

    def shade(u, v):
        drift = (octaves(warp, u, v) - 0.5) * 0.25
        line = 0.5 + 0.5 * math.sin(TAU * (u * 6 + drift * 6))
        value = 0.72 - 0.3 * line**3
        # Pores: dark dashes along the grain.
        if noise(pores, u * 2, v * 0.25) > 0.68:
            value -= 0.25
        for fu, fv, length in flecks:
            du = (u - fu + 0.5) % 1 - 0.5
            dv = (v - fv + 0.5) % 1 - 0.5
            if (du / length) ** 2 + (dv / 0.012) ** 2 < 1:
                value = 0.95
        return value

    return picture(shade)


def wavy(seed):
    """Walnut: grain swaying in long waves, with darker streaks."""
    warp = [lattice(4, seed), lattice(8, seed + 1)]
    streaks = [lattice(4, seed + 2)]

    def shade(u, v):
        sway = math.sin(TAU * v * 2) * 0.05 + (octaves(warp, u, v) - 0.5) * 0.35
        line = 0.5 + 0.5 * math.sin(TAU * (u * 7 + sway * 7))
        streak = octaves(streaks, u, v * 0.5)
        return 0.78 - 0.3 * line**2 - 0.25 * max(0.0, streak - 0.5)

    return picture(shade)


def knotty(seed):
    """Pine: wide growth rings with sharp dark latewood, and a few knots the
    grain bends round."""
    warp = [lattice(4, seed), lattice(8, seed + 1)]
    knots = [(0.3, 0.25, 0.07), (0.72, 0.7, 0.05)]

    def shade(u, v):
        bend = 0.0
        for ku, kv, size in knots:
            du = (u - ku + 0.5) % 1 - 0.5
            dv = (v - kv + 0.5) % 1 - 0.5
            distance = math.hypot(du, dv * 0.6)
            if distance < size:
                # The knot itself: dark rings round a darker heart.
                rings = 0.5 + 0.5 * math.cos(TAU * distance / size * 2.5)
                return 0.2 + 0.3 * rings * distance / size
            bend += math.copysign(size * size * 4 / (distance + 0.02), du)
        drift = (octaves(warp, u, v) - 0.5) * 0.15 + bend
        phase = (u * 5 + drift * 5) % 1
        # Light earlywood fading into a sharp dark band.
        return 0.8 - 0.6 * max(0.0, (phase - 0.55) / 0.45) ** 2

    return picture(shade)


def end_grain(seed):
    """Wood's end: growth rings round a heart off the tile."""
    warp = [lattice(4, seed)]

    def shade(u, v):
        distance = math.hypot(u + 0.4, v - 1.3)
        wobble = (octaves(warp, u, v) - 0.5) * 0.05
        ring = 0.5 + 0.5 * math.sin(TAU * (distance + wobble) * 12)
        return 0.75 - 0.3 * ring**3

    return picture(shade)


def granite(seed):
    """Granite: speckled light and dark grains."""
    rng = random.Random(seed)
    grains = {}
    for _ in range(700):
        grains[(rng.randrange(SIZE), rng.randrange(SIZE))] = rng.choice([0.15, 0.3, 0.95])
    cloud = [lattice(4, seed + 1), lattice(8, seed + 2)]

    def shade(u, v):
        cell = (int(u * SIZE), int(v * SIZE))
        if cell in grains:
            return grains[cell]
        return 0.55 + 0.25 * octaves(cloud, u, v)

    return picture(shade)


def sandstone(seed):
    """Sandstone: soft bands lying level, and a fine sandy grain."""
    warp = [lattice(4, seed), lattice(8, seed + 1)]
    sand = lattice(64, seed + 2)

    def shade(u, v):
        drift = (octaves(warp, u, v) - 0.5) * 0.2
        band = 0.5 + 0.5 * math.sin(TAU * (v * 4 + drift * 4))
        return 0.6 + 0.2 * band + 0.18 * (noise(sand, u, v) - 0.5)

    return picture(shade)


def limestone(seed):
    """Limestone: plain and pale, faintly mottled, with a few tiny shells."""
    cloud = [lattice(4, seed), lattice(8, seed + 1), lattice(16, seed + 2)]
    rng = random.Random(seed + 3)
    shells = [(rng.random(), rng.random()) for _ in range(5)]

    def shade(u, v):
        for su, sv in shells:
            du = (u - su + 0.5) % 1 - 0.5
            dv = (v - sv + 0.5) % 1 - 0.5
            if 0.012 < math.hypot(du, dv) < 0.025:
                return 0.55
        return 0.72 + 0.16 * (octaves(cloud, u, v) - 0.5)

    return picture(shade)


def marble(seed):
    """Marble: bright, with thin dark veins wandering across it."""
    turbulence = [lattice(4, seed), lattice(8, seed + 1), lattice(16, seed + 2)]

    def shade(u, v):
        twist = octaves(turbulence, u, v)
        vein = abs(math.sin(TAU * (u + v + twist * 1.5)))
        return 0.9 - 0.55 * (1 - vein) ** 12 - 0.06 * twist

    return picture(shade)


def slate(seed):
    """Slate: fine level layers, some a little darker, and a dull sheen."""
    rng = random.Random(seed)
    layers = [rng.random() for _ in range(32)]
    cloud = [lattice(4, seed + 1)]

    def shade(u, v):
        layer = layers[int(v * 32) % 32]
        return 0.55 + 0.15 * layer + 0.15 * (octaves(cloud, u, v) - 0.5)

    return picture(shade)


# The order `furniture::atlas` in the client expects.
TILES = [
    straight(10),
    coarse(20),
    wavy(30),
    knotty(40),
    granite(50),
    sandstone(60),
    limestone(70),
    marble(80),
    slate(90),
    end_grain(100),
]


def main():
    atlas = Image.new("RGBA", (COLUMNS * SIZE, ROWS * SIZE), (200, 200, 200, 255))
    for index, tile in enumerate(TILES):
        column, row = index % COLUMNS, index // COLUMNS
        atlas.paste(tile.convert("RGBA"), (column * SIZE, row * SIZE))
    atlas.save(OUT)


if __name__ == "__main__":
    main()
