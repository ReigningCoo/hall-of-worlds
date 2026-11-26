"""Makes crates/client/assets/music/music.sf2, the music's instruments, from
two SoundFonts (SF2), keeping only the presets the music plays. MuseScore
General's keep their General MIDI bank and program, so tunes name instruments
as General MIDI does; its drum kit keeps only the keys listed. The Early
European Instruments' go in bank 8 (General MIDI's variations) under the
program of the instrument nearest them, one channel at a quarter of their
192 kHz. Run from the repository's root:

    python tools/subset_soundfont.py <MuseScore_General.sf2> <EarlyEuropean.sf2> [out.sf2 | --sizes]

MuseScore General 0.2 (MIT):
https://ftp.osuosl.org/pub/musescore/soundfont/MuseScore_General/
Early European Instruments (public domain, a signed-in download):
https://www.polyphone.io/en/soundfonts/instrument-sets/516-early-european-instruments
Plain Python, no packages."""

import math
import struct
import sys

OUT = "crates/client/assets/music/music.sf2"

# Each source's presets: (bank, program) in it -> (bank, program) kept as,
# from 0, and what it stands for. Aiming for about 10 MB: Choir Aahs (4.8 MB)
# gave way to Voice Oohs, and the harpsichord, violin, flute, pan flute, oboe
# and banjo to those kept (`--sizes` prints what each preset alone brings).
MUSESCORE = {
    (0, 14): (0, 14, "tubular bells"),
    (0, 15): (0, 15, "dulcimer (psaltery)"),
    (0, 19): (0, 19, "church organ"),
    (0, 20): (0, 20, "reed organ (portative)"),
    (0, 24): (0, 24, "nylon guitar (lute)"),
    (0, 46): (0, 46, "harp"),
    (0, 47): (0, 47, "timpani (nakers)"),
    (0, 53): (0, 53, "voice oohs (choir)"),
    (0, 56): (0, 56, "trumpet (buisine)"),
    (0, 57): (0, 57, "trombone (sackbut)"),
    (0, 74): (0, 74, "recorder"),
    (0, 109): (0, 109, "bagpipe"),
    (0, 110): (0, 110, "fiddle"),
    (0, 111): (0, 111, "shanai (shawm)"),
    (0, 112): (0, 112, "tinkle bell"),
    (0, 116): (0, 116, "taiko (large drum)"),
    (128, 0): (128, 0, "standard drum kit (keys below)"),
}
EARLY_EUROPEAN = {
    (0, 0): (8, 110, "hurdy-gurdy (a fiddle's variation)"),
    (0, 1): (8, 109, "cornemuse (a bagpipe's variation)"),
}

# Drum keys kept: bass drum (tabor's low beat), snare, low and high toms
# (frame drums), tambourine, bongos and congas (hand drums), claves and wood
# blocks, triangles (bells).
DRUM_KEYS = {36, 38, 41, 45, 54, 60, 61, 62, 63, 64, 75, 76, 80, 81}

# The Early European Instruments' samples: left channel only, a quarter rate.
DECIMATE = 4

KEY_RANGE, PAN, INSTRUMENT, SAMPLE_ID = 43, 17, 41, 53
MONO, RIGHT, LEFT = 1, 2, 4


def chunks(data, start, end):
    at = start
    while at + 8 <= end:
        cid, size = struct.unpack_from("<4sI", data, at)
        yield cid, at + 8, at + 8 + size
        at += 8 + size + (size & 1)


def lists(data):
    """Every chunk inside the file's LISTs, by four-letter id."""
    found = {}
    riff, size, form = struct.unpack_from("<4sI4s", data, 0)
    assert riff == b"RIFF" and form == b"sfbk", "not a SoundFont"
    for cid, start, end in chunks(data, 12, 8 + size):
        if cid == b"LIST":
            for sub, s, e in chunks(data, start + 4, end):
                found[sub] = data[s:e]
    return found


def records(blob, size):
    return [blob[i:i + size] for i in range(0, len(blob), size)]


def zones(c, kind, bag, next_bag):
    """[(generators, modulator records)] of bags bag to next_bag."""
    bags = records(c[kind + b"bag"], 4)
    out = []
    for b in range(bag, next_bag):
        g0, m0 = struct.unpack_from("<HH", bags[b])
        g1, m1 = struct.unpack_from("<HH", bags[b + 1])
        gens = [struct.unpack_from("<HH", c[kind + b"gen"], i * 4) for i in range(g0, g1)]
        out.append((gens, records(c[kind + b"mod"], 10)[m0:m1]))
    return out


def keeps_keys(gens):
    lo, hi = 0, 127
    for op, amount in gens:
        if op == KEY_RANGE:
            lo, hi = amount & 0xFF, amount >> 8
    return any(lo <= k <= hi for k in DRUM_KEYS)


def decimated(pcm, factor):
    """16-bit samples at 1/factor the rate, through a windowed-sinc low pass."""
    x = struct.unpack(f"<{len(pcm) // 2}h", pcm)
    taps, cutoff = 63, 0.45 / factor
    half = taps // 2
    h = []
    for k in range(taps):
        n = k - half
        sinc = 2 * cutoff if n == 0 else math.sin(2 * math.pi * cutoff * n) / (math.pi * n)
        h.append(sinc * (0.5 - 0.5 * math.cos(2 * math.pi * k / (taps - 1))))
    total = sum(h)
    h = [v / total for v in h]
    out = []
    for m in range(0, len(x), factor):
        acc = 0.0
        for k, v in enumerate(h):
            i = m + k - half
            if 0 <= i < len(x):
                acc += v * x[i]
        out.append(max(-32768, min(32767, round(acc))))
    return struct.pack(f"<{len(out)}h", *out)


def part(data, wanted, drums, mono):
    """The wanted presets, their instruments and samples, indexed locally.
    Samples are (header record, its sound) with loops relative to its start."""
    c = lists(data)
    smpl, shdr = c[b"smpl"], records(c[b"shdr"], 46)
    phdr, inst = records(c[b"phdr"], 38), records(c[b"inst"], 22)
    insts, samples, inst_map, sample_map = [], [], {}, {}

    def use_sample(index):
        if index not in sample_map:
            rec = shdr[index]
            start, end, loop_start, loop_end, rate = struct.unpack_from("<IIIII", rec, 20)
            pitch, correction, link, kind = struct.unpack_from("<BbHH", rec, 40)
            pcm = smpl[start * 2:end * 2]
            loop = (loop_start - start, loop_end - start)
            if mono:
                pcm = decimated(pcm, DECIMATE)
                loop = tuple(round(at / DECIMATE) for at in loop)
                rate, kind = rate // DECIMATE, MONO
            sample_map[index] = len(samples)
            samples.append([rec[:20], pcm, loop, rate, pitch, correction, None, kind])
            if kind & (RIGHT | LEFT | 8):
                # The other side of a stereo pair, kept with it.
                samples[sample_map[index]][6] = use_sample(link)
        return sample_map[index]

    def use_instrument(index, drum_kit):
        key = (index, drum_kit)
        if key not in inst_map:
            bag = struct.unpack_from("<H", inst[index], 20)[0]
            next_bag = struct.unpack_from("<H", inst[index + 1], 20)[0]
            kept = []
            for gens, mods in zones(c, b"i", bag, next_bag):
                sample = [a for op, a in gens if op == SAMPLE_ID]
                if sample and drum_kit and not keeps_keys(gens):
                    continue
                if sample and mono:
                    kind = struct.unpack_from("<H", shdr[sample[0]], 44)[0]
                    if kind & RIGHT:
                        continue
                    gens = [(op, a) for op, a in gens if op != PAN]
                gens = [(op, use_sample(a) if op == SAMPLE_ID else a) for op, a in gens]
                kept.append((gens, mods))
            inst_map[key] = len(insts)
            insts.append((inst[index][:20], kept))
        return inst_map[key]

    presets = []
    for i, rec in enumerate(phdr[:-1]):
        program, bank, bag = struct.unpack_from("<HHH", rec, 20)
        if (bank, program) not in wanted:
            continue
        new_bank, new_program, _ = wanted[(bank, program)]
        drum_kit = drums and bank == 128
        next_bag = struct.unpack_from("<H", phdr[i + 1], 24)[0]
        kept = []
        for gens, mods in zones(c, b"p", bag, next_bag):
            target = [a for op, a in gens if op == INSTRUMENT]
            if target and drum_kit and not keeps_keys(gens):
                continue
            gens = [(op, use_instrument(a, drum_kit) if op == INSTRUMENT else a) for op, a in gens]
            kept.append((gens, mods))
        presets.append((rec[:20], new_bank, new_program, rec[26:], kept))
    found = {(b, p) for _, b, p, _, _ in presets}
    missing = {v[:2] for v in wanted.values()} - found
    assert not missing, f"presets not in the file: {sorted(missing)}"
    return presets, insts, samples


def assemble(parts):
    """One SoundFont of every part, their indexes moved past the ones before."""
    presets, insts, samples = [], [], []
    for p, i, s in parts:
        i_at, s_at = len(insts), len(samples)
        presets += [(n, b, pr, tail, [([(op, a + i_at if op == INSTRUMENT else a) for op, a in g], m) for g, m in z])
                    for n, b, pr, tail, z in p]
        insts += [(n, [([(op, a + s_at if op == SAMPLE_ID else a) for op, a in g], m) for g, m in z]) for n, z in i]
        samples += [rec[:6] + [None if rec[6] is None else rec[6] + s_at] + rec[7:] for rec in s]

    smpl, shdr = bytearray(), bytearray()
    for name, pcm, (loop_start, loop_end), rate, pitch, correction, link, kind in samples:
        at = len(smpl) // 2
        smpl += pcm + bytes(92)
        shdr += name + struct.pack("<IIIIIBbHH", at, at + len(pcm) // 2, at + loop_start, at + loop_end,
                                   rate, pitch, correction, link or 0, kind)
    shdr += b"EOS".ljust(20, b"\0") + bytes(26)

    def write_zones(all_zones):
        bags, gens, mods, starts = bytearray(), bytearray(), bytearray(), []
        for zone_list in all_zones:
            starts.append(len(bags) // 4)
            for g, m in zone_list:
                bags += struct.pack("<HH", len(gens) // 4, len(mods) // 10)
                for op, a in g:
                    gens += struct.pack("<HH", op, a)
                for record in m:
                    mods += record
        starts.append(len(bags) // 4)
        bags += struct.pack("<HH", len(gens) // 4, len(mods) // 10)
        return starts, bags, gens + bytes(4), mods + bytes(10)

    i_starts, ibag, igen, imod = write_zones([z for _, z in insts])
    inst = b"".join(n + struct.pack("<H", s) for (n, _), s in zip(insts, i_starts))
    inst += b"EOI".ljust(20, b"\0") + struct.pack("<H", i_starts[-1])
    p_starts, pbag, pgen, pmod = write_zones([z for *_, z in presets])
    phdr = b"".join(n + struct.pack("<HHH", pr, b, s) + tail
                    for (n, b, pr, tail, _), s in zip(presets, p_starts))
    phdr += b"EOP".ljust(20, b"\0") + struct.pack("<HHHIII", 0, 0, p_starts[-1], 0, 0, 0)

    def chunk(cid, body):
        body = bytes(body)
        return cid + struct.pack("<I", len(body)) + body + (b"\0" if len(body) & 1 else b"")

    def lst(kind, items):
        body = kind + b"".join(items)
        return b"LIST" + struct.pack("<I", len(body)) + body

    def text(s):
        b = s.encode("ascii") + b"\0"
        return b + (b"\0" if len(b) & 1 else b"")

    info = lst(b"INFO", [
        chunk(b"ifil", struct.pack("<HH", 2, 1)),
        chunk(b"isng", text("EMU8000")),
        chunk(b"INAM", text("Hall of Worlds music")),
        chunk(b"ICOP", text("MuseScore General 0.2, MIT: FluidR3 (c) 2000-2002, 2008 Frank Wen; "
                            "FluidR3Mono (c) 2014-17 Michael Cowgill; MuseScore General (c) 2018-19 "
                            "S. Christian Collins. Early European Instruments: public domain.")),
    ])
    sdta = lst(b"sdta", [chunk(b"smpl", smpl)])
    pdta = lst(b"pdta", [chunk(b"phdr", phdr), chunk(b"pbag", pbag), chunk(b"pmod", pmod),
                         chunk(b"pgen", pgen), chunk(b"inst", inst), chunk(b"ibag", ibag),
                         chunk(b"imod", imod), chunk(b"igen", igen), chunk(b"shdr", shdr)])
    body = b"sfbk" + info + sdta + pdta
    return b"RIFF" + struct.pack("<I", len(body)) + body, len(samples)


def main():
    musescore, early = (open(path, "rb").read() for path in sys.argv[1:3])
    sources = [(musescore, MUSESCORE, True, False), (early, EARLY_EUROPEAN, False, True)]
    if sys.argv[3:] == ["--sizes"]:
        for data, wanted, drums, mono in sources:
            for key, kept in wanted.items():
                result, samples = assemble([part(data, {key: kept}, drums, mono)])
                print(f"{kept[:2]}: {len(result) / 1e6:5.1f} MB, {samples:3} samples  {kept[2]}")
        return
    out = sys.argv[3] if len(sys.argv) > 3 else OUT
    result, samples = assemble([part(*source) for source in sources])
    with open(out, "wb") as f:
        f.write(result)
    count = len(MUSESCORE) + len(EARLY_EUROPEAN)
    print(f"{out}: {count} presets, {samples} samples, {len(result) / 1e6:.1f} MB")


if __name__ == "__main__":
    main()
