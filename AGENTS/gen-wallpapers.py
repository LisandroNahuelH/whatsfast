"""Dense 4K chat wallpapers: tiny designed icons, one palette per family."""
import math
import random
from pathlib import Path

from PIL import Image, ImageDraw

W, H = 3840, 2160
ROOT = Path(__file__).resolve().parents[1] / "assets" / "wallpapers"

# (bg, stroke) inside the brief's ranges. Three slots, three mixes.
FAMILIES = {
    "black": [
        ((8, 12, 16), (30, 40, 48)),
        ((9, 14, 18), (33, 44, 52)),
        ((11, 20, 26), (36, 48, 58)),
    ],
    "green": [
        ((6, 23, 19), (20, 56, 47)),
        ((7, 26, 22), (23, 62, 52)),
        ((9, 31, 26), (26, 71, 60)),
    ],
    "gray": [
        ((21, 24, 30), (45, 53, 64)),
        ((23, 27, 34), (49, 57, 69)),
        ((26, 31, 38), (53, 62, 75)),
    ],
    "red": [
        ((23, 11, 16), (56, 31, 38)),
        ((26, 13, 18), (60, 34, 42)),
        ((30, 15, 21), (66, 37, 46)),
    ],
    "white": [
        ((240, 242, 245), (192, 200, 205)),
        ((239, 234, 226), (184, 193, 198)),
        ((236, 238, 241), (176, 186, 192)),
    ],
}


def unit(n, salt):
    x = (n * 2246822519 + salt * 3266489917) & 0xFFFFFFFF
    x ^= x >> 16
    return (x & 0xFFFFFF) / 0xFFFFFF


def rot(pts, cx, cy, deg):
    a = math.radians(deg)
    c, s = math.cos(a), math.sin(a)
    out = []
    for x, y in pts:
        dx, dy = x - cx, y - cy
        out.append((cx + dx * c - dy * s, cy + dx * s + dy * c))
    return out


def line(d, pts, col, w=1):
    if len(pts) >= 2:
        d.line(pts, fill=col, width=w)


def oval(d, cx, cy, rx, ry, col, w=1):
    d.ellipse((cx - rx, cy - ry, cx + rx, cy + ry), outline=col, width=w)


def poly(d, pts, col, w=1):
    d.polygon(pts, outline=col)
    line(d, list(pts) + [pts[0]], col, w)


def draw_icon(d, kind, cx, cy, s, col, n):
    tilt = (unit(n, 3) - 0.5) * 16
    k = 0.88 + unit(n, 9) * 0.24  # within 50% of the base size
    s *= k

    def P(*pairs):
        return rot([(cx + x * s, cy + y * s) for x, y in pairs], cx, cy, tilt)

    if kind == 0:  # heart
        poly(d, P((0, 0.15), (-0.55, -0.35), (-0.25, -0.75), (0, -0.4), (0.25, -0.75), (0.55, -0.35)), col)
    elif kind == 1:  # star
        pts = []
        for i in range(10):
            r = 0.85 if i % 2 == 0 else 0.38
            a = -math.pi / 2 + i * math.pi / 5
            pts.append((cx + math.cos(a) * r * s, cy + math.sin(a) * r * s))
        poly(d, rot(pts, cx, cy, tilt), col)
    elif kind == 2:  # note
        oval(d, cx - 0.25 * s, cy + 0.35 * s, 0.32 * s, 0.22 * s, col)
        line(d, P((-0.05, 0.35), (-0.05, -0.7), (0.55, -0.45), (0.55, -0.15)), col)
    elif kind == 3:  # cup
        line(d, P((-0.45, -0.2), (0.35, -0.2), (0.25, 0.45), (-0.35, 0.45), (-0.45, -0.2)), col)
        d.arc((cx + 0.2 * s, cy - 0.25 * s, cx + 0.7 * s, cy + 0.25 * s), 300, 70, fill=col, width=1)
        line(d, P((-0.15, -0.55), (-0.15, -0.3)), col)
        line(d, P((0.1, -0.6), (0.1, -0.3)), col)
    elif kind == 4:  # camera
        poly(d, P((-0.7, -0.2), (-0.3, -0.2), (-0.15, -0.45), (0.35, -0.45), (0.5, -0.2), (0.7, -0.2), (0.7, 0.5), (-0.7, 0.5)), col)
        oval(d, cx + 0.05 * s, cy + 0.1 * s, 0.28 * s, 0.28 * s, col)
    elif kind == 5:  # book
        line(d, P((-0.6, -0.55), (0.05, -0.4), (0.05, 0.6), (-0.6, 0.45), (-0.6, -0.55)), col)
        line(d, P((0.05, -0.4), (0.65, -0.55), (0.65, 0.45), (0.05, 0.6)), col)
    elif kind == 6:  # cat
        oval(d, cx, cy + 0.1 * s, 0.48 * s, 0.42 * s, col)
        line(d, P((-0.4, -0.15), (-0.62, -0.7), (-0.1, -0.35)), col)
        line(d, P((0.4, -0.15), (0.62, -0.7), (0.1, -0.35)), col)
        oval(d, cx - 0.16 * s, cy + 0.05 * s, 0.05 * s, 0.05 * s, col)
        oval(d, cx + 0.16 * s, cy + 0.05 * s, 0.05 * s, 0.05 * s, col)
    elif kind == 7:  # dog
        oval(d, cx, cy + 0.12 * s, 0.42 * s, 0.4 * s, col)
        oval(d, cx - 0.48 * s, cy - 0.15 * s, 0.18 * s, 0.28 * s, col)
        oval(d, cx + 0.48 * s, cy - 0.15 * s, 0.18 * s, 0.28 * s, col)
        oval(d, cx, cy + 0.22 * s, 0.1 * s, 0.07 * s, col)
    elif kind == 8:  # bird
        line(d, P((-0.55, 0.1), (0.1, -0.15), (0.55, 0.15), (0.05, 0.2), (-0.55, 0.1)), col)
        line(d, P((0.45, 0.05), (0.75, -0.15), (0.5, 0.2)), col)
        oval(d, cx - 0.15 * s, cy - 0.02 * s, 0.05 * s, 0.05 * s, col)
    elif kind == 9:  # bicycle
        oval(d, cx - 0.4 * s, cy + 0.25 * s, 0.28 * s, 0.28 * s, col)
        oval(d, cx + 0.4 * s, cy + 0.25 * s, 0.28 * s, 0.28 * s, col)
        line(d, P((-0.4, 0.25), (0.05, 0.25), (0.05, -0.35), (-0.15, -0.35)), col)
        line(d, P((0.05, 0.25), (0.4, 0.25), (0.15, -0.15), (-0.15, -0.05), (0.05, 0.25)), col)
    elif kind == 10:  # car
        poly(d, P((-0.75, 0.15), (-0.45, -0.25), (0.35, -0.25), (0.6, 0.15), (0.75, 0.15), (0.75, 0.4), (-0.75, 0.4)), col)
        oval(d, cx - 0.4 * s, cy + 0.4 * s, 0.14 * s, 0.14 * s, col)
        oval(d, cx + 0.4 * s, cy + 0.4 * s, 0.14 * s, 0.14 * s, col)
    elif kind == 11:  # house
        line(d, P((-0.6, 0.15), (0, -0.6), (0.6, 0.15)), col)
        poly(d, P((-0.45, 0.15), (0.45, 0.15), (0.45, 0.65), (-0.45, 0.65)), col)
    elif kind == 12:  # plant
        line(d, P((0, 0.65), (0, -0.1)), col)
        oval(d, cx, cy - 0.35 * s, 0.22 * s, 0.32 * s, col)
        oval(d, cx - 0.28 * s, cy - 0.05 * s, 0.18 * s, 0.26 * s, col)
        oval(d, cx + 0.28 * s, cy - 0.05 * s, 0.18 * s, 0.26 * s, col)
        line(d, P((-0.35, 0.55), (0.35, 0.55), (0.25, 0.7), (-0.25, 0.7), (-0.35, 0.55)), col)
    elif kind == 13:  # key
        oval(d, cx - 0.35 * s, cy - 0.15 * s, 0.24 * s, 0.24 * s, col)
        line(d, P((-0.12, -0.05), (0.7, 0.35)), col)
        line(d, P((0.45, 0.22), (0.58, 0.05)), col)
        line(d, P((0.62, 0.32), (0.75, 0.15)), col)
    elif kind == 14:  # bulb
        d.arc((cx - 0.38 * s, cy - 0.6 * s, cx + 0.38 * s, cy + 0.2 * s), 200, 340, fill=col, width=1)
        line(d, P((-0.22, 0.15), (-0.22, 0.4), (0.22, 0.4), (0.22, 0.15)), col)
        line(d, P((-0.14, 0.4), (0.14, 0.4)), col)
        line(d, P((-0.1, 0.5), (0.1, 0.5)), col)
    elif kind == 15:  # envelope
        poly(d, P((-0.7, -0.4), (0.7, -0.4), (0.7, 0.45), (-0.7, 0.45)), col)
        line(d, P((-0.7, -0.4), (0, 0.1), (0.7, -0.4)), col)
    elif kind == 16:  # compass
        oval(d, cx, cy, 0.62 * s, 0.62 * s, col)
        poly(d, P((0, -0.45), (0.12, 0.05), (0, 0.15), (-0.12, 0.05)), col)
    elif kind == 17:  # watch
        line(d, P((-0.16, -0.72), (0.16, -0.72), (0.16, -0.48), (-0.16, -0.48), (-0.16, -0.72)), col)
        oval(d, cx, cy + 0.05 * s, 0.4 * s, 0.4 * s, col)
        line(d, P((0, 0.05), (0, -0.22)), col)
        line(d, P((0, 0.05), (0.18, 0.16)), col)
    elif kind == 18:  # leaf
        d.arc((cx - 0.15 * s, cy - 0.7 * s, cx + 0.7 * s, cy + 0.2 * s), 120, 300, fill=col, width=1)
        d.arc((cx - 0.7 * s, cy - 0.2 * s, cx + 0.15 * s, cy + 0.7 * s), 300, 120, fill=col, width=1)
        line(d, P((-0.35, 0.4), (0.4, -0.4)), col)
    elif kind == 19:  # headphones
        d.arc((cx - 0.5 * s, cy - 0.45 * s, cx + 0.5 * s, cy + 0.4 * s), 200, 340, fill=col, width=1)
        poly(d, P((-0.55, 0.0), (-0.35, 0.0), (-0.35, 0.45), (-0.55, 0.45)), col)
        poly(d, P((0.35, 0.0), (0.55, 0.0), (0.55, 0.45), (0.35, 0.45)), col)
    elif kind == 20:  # plane
        line(d, P((-0.7, 0.15), (0.7, -0.35), (0.15, 0.15), (0.35, 0.55), (0.05, 0.15), (-0.2, 0.4), (-0.15, 0.05), (-0.7, 0.15)), col)
    elif kind == 21:  # ball
        oval(d, cx, cy, 0.5 * s, 0.5 * s, col)
        d.arc((cx - 0.5 * s, cy - 0.15 * s, cx + 0.5 * s, cy + 0.55 * s), 200, 340, fill=col, width=1)
        line(d, P((0, -0.5), (0, 0.5)), col)
    elif kind == 22:  # moon
        d.arc((cx - 0.4 * s, cy - 0.55 * s, cx + 0.45 * s, cy + 0.55 * s), 100, 260, fill=col, width=1)
        d.arc((cx - 0.05 * s, cy - 0.4 * s, cx + 0.55 * s, cy + 0.4 * s), 110, 250, fill=col, width=1)
    elif kind == 23:  # flower
        for i in range(5):
            a = -math.pi / 2 + i * math.tau / 5 + tilt * 0.02
            oval(d, cx + math.cos(a) * 0.32 * s, cy + math.sin(a) * 0.32 * s, 0.18 * s, 0.18 * s, col)
        oval(d, cx, cy, 0.12 * s, 0.12 * s, col)
    elif kind == 24:  # umbrella
        d.arc((cx - 0.6 * s, cy - 0.55 * s, cx + 0.6 * s, cy + 0.25 * s), 180, 360, fill=col, width=1)
        line(d, P((0, -0.15), (0, 0.45)), col)
        d.arc((cx - 0.22 * s, cy + 0.25 * s, cx + 0.05 * s, cy + 0.6 * s), 0, 180, fill=col, width=1)
    elif kind == 25:  # boat
        line(d, P((-0.65, 0.1), (0.65, 0.1), (0.4, 0.45), (-0.4, 0.45), (-0.65, 0.1)), col)
        line(d, P((0, 0.1), (0, -0.6)), col)
        line(d, P((0, -0.55), (0.4, -0.1)), col)
    elif kind == 26:  # guitar
        oval(d, cx, cy + 0.25 * s, 0.32 * s, 0.4 * s, col)
        oval(d, cx, cy + 0.28 * s, 0.12 * s, 0.12 * s, col)
        line(d, P((-0.08, -0.1), (-0.08, -0.75), (0.08, -0.75), (0.08, -0.1)), col)
    elif kind == 27:  # pizza
        line(d, P((0, -0.7), (0.65, 0.5), (-0.65, 0.5), (0, -0.7)), col)
        oval(d, cx, cy + 0.05 * s, 0.08 * s, 0.08 * s, col)
        oval(d, cx - 0.2 * s, cy + 0.2 * s, 0.07 * s, 0.07 * s, col)
    elif kind == 28:  # glasses
        oval(d, cx - 0.32 * s, cy, 0.26 * s, 0.2 * s, col)
        oval(d, cx + 0.32 * s, cy, 0.26 * s, 0.2 * s, col)
        line(d, P((-0.06, 0), (0.06, 0)), col)
        line(d, P((-0.58, -0.05), (-0.75, -0.15)), col)
        line(d, P((0.58, -0.05), (0.75, -0.15)), col)
    elif kind == 29:  # pencil
        line(d, P((-0.55, 0.2), (0.35, -0.55), (0.55, -0.35), (-0.35, 0.4), (-0.55, 0.2)), col)
        line(d, P((-0.45, 0.28), (-0.62, 0.42)), col)
    else:
        oval(d, cx, cy, 0.2 * s, 0.2 * s, col)


def micro(d, cx, cy, s, col, n):
    m = n % 6
    if m == 0:
        oval(d, cx, cy, s, s, col)
    elif m == 1:
        line(d, [(cx - s, cy), (cx + s, cy)], col)
        line(d, [(cx, cy - s), (cx, cy + s)], col)
    elif m == 2:
        oval(d, cx, cy, s * 0.7, s * 0.7, col)
        oval(d, cx, cy, s * 0.25, s * 0.25, col)
    elif m == 3:
        line(d, [(cx - s, cy + s * 0.3), (cx, cy - s * 0.4), (cx + s, cy + s * 0.2)], col)
    elif m == 4:
        pts = []
        for i in range(8):
            r = s if i % 2 == 0 else s * 0.4
            a = i * math.pi / 4
            pts.append((cx + math.cos(a) * r, cy + math.sin(a) * r))
        poly(d, pts, col)
    else:
        line(d, [(cx - s, cy), (cx - s * 0.2, cy - s * 0.6), (cx + s * 0.4, cy + s * 0.2), (cx + s, cy)], col)


def render(path, bg, fg, seed, step):
    rng = random.Random(seed)
    img = Image.new("RGB", (W, H), bg)
    drw = ImageDraw.Draw(img)
    n = 0
    row = 0
    y = 8.0
    prev = -1
    while y < H:
        x = 4.0 if row % 2 == 0 else step * 0.45
        while x < W:
            kind = rng.randrange(30)
            if kind == prev:
                kind = (kind + 11) % 30
            prev = kind
            jx = x + rng.uniform(-3, 3)
            jy = y + rng.uniform(-3, 3)
            s = 11 + rng.randrange(5)
            draw_icon(drw, kind, jx, jy, s, fg, n * 17 + seed)
            micro(drw, jx + 13, jy + 11, 2 + (n % 3), fg, n + seed)
            micro(drw, jx - 9, jy + 14, 1 + (n % 2), fg, n * 3 + seed)
            x += step
            n += 1
        y += step - 2
        row += 1
    y = 18.0
    row = 0
    k = 0
    while y < H:
        x = 16.0 if row % 2 else 3.0
        while x < W:
            micro(drw, x + rng.uniform(-2, 2), y, 2 + ((k + seed) % 3), fg, k * 13 + seed * 5)
            x += step * 0.5
            k += 1
        y += step * 0.48
        row += 1
    out = img.quantize(colors=128, method=Image.Quantize.MEDIANCUT, dither=Image.Dither.NONE)
    path.parent.mkdir(parents=True, exist_ok=True)
    out.save(path, optimize=True)
    print(path.name, out.size, out.mode, path.stat().st_size)


def main():
    slot_seeds = {
        "black": (11, 29, 47),
        "gray": (71, 97, 113),
        "green": (131, 157, 179),
        "red": (211, 239, 257),
        "white": (307, 331, 359),
    }
    steps = (32, 36, 30)
    for family, pairs in FAMILIES.items():
        for i, (bg, fg) in enumerate(pairs):
            render(
                ROOT / family / f"{i + 1:02}.png",
                bg,
                fg,
                slot_seeds[family][i],
                steps[i],
            )


if __name__ == "__main__":
    main()
