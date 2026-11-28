# Mob behaviour plan

How mobs and NPCs decide what to do, in 9 PRs. Planned with the owner on 2026-11-27, starting from RogueBasin's "A Better Monster AI": drives with weights (aggression, fear, desire, wander, misc), the strongest score winning; courage from allies seen and fear from enemies' strength; targets plus pathfinding rather than its 8-direction vectors; remembering the last strong desire; every mob's scores worked out before any moves.

## Before the plan

`Temper` and `Senses` were matches in `shared/src/creature/senses.rs`; `notice` (server `entities/notice.rs`) only sees players; memory is per mob per character (`shared::mob::Memory`, `provoked` meaning self-defence for alignment); `listen` turns idle mobs to fights heard (4, 8 or 14 cells); Sneak only slows the pace; no groups beyond a wild herd's den.

## Owner's decisions

- Behaviour is configuration per race (`behaviours.ron`), **moddable** like herbs: `HOW_MODS/behaviours/*.ron`, the same race replacing; server-only, never sent to clients.
- **Bands** within a race, each with its own friends and enemies (a band variant rolled per band). **Families** within bands: a field reserved, nothing built yet; for the more intelligent races, not animals.
- **Mobs fight mobs.** Wolves hunt wild prey (rabbits) and **always** go after prey they see (no hunger clock). A kill is eaten: no items left (a blood mark on the ground later). Prey running far enough (past the leash from the den) makes the hunters give up and go home. Fleeing is straight for some, **zigzag** for prey animals, by race.
- Hunting is **wild only** for now: never penned livestock or escortees. For wolves and the like, **fences and gates are nearly impossible to cross**, and they are **afraid of open fires** (campfires, lit fireplaces).
- **Sneak attacks:** a blow on an Unaware mob is a guaranteed critical (double harm for now), kept as its own rule for future critical hits.
- **No markers over mobs** (they would be confused with quest markers). Instead, **while Sneak is on only**, an eye over the player, closed to open by how noticeable they are.
- **Desire** (going for things on the ground): basic support now, improved later.

## PRs

1 first, then 2; 3 to 5 and 6 to 8 are two lines that may interleave (8's alarm calls best after 3); 9 any time after 2.

1. **Settings, bands, mods** (built: `shared::behaviour`, server `behaviours.rs`): drives, defends, alarm call, `hunts`, flee style, fence crossing, fears fire, desires; `Temper` moved in; a band on each `Spawn` (a wild pack or herd; per race per site), band variants with relations, `relation(a, b)` (Kin, Ally, Neutral, Rival, Enemy); family reserved; the server loads mods; no change in play.
2. **Scorer** (built: server `entities/choice`) replacing `notice`, `after_hit`, territory and timid with today's results, plus morale (fleeing when hurt by courage, braver with the band in sight, afraid when outnumbered), a margin against flip-flopping, straight or zigzag fleeing, fear of open fire, fences for wolves; scores in Inspect. Numbers chosen (Claude's, to check in play): attack 50 + aggression ÷ 2, guard 45 + guarding ÷ 4, approach 40, keep away 60, fear hurt × (100 − courage) ÷ 40 + 20 per extra enemy − 10 per friend seen (three at most), margin 10, fires kept 3 cells off; `Seldom` fences give way only to a mob fleeing.
3. **Pack defence:** mobs see mobs in their cone; seeing one of its band or an ally harmed by a player, it remembers the attacker (provoked) and joins.
4. **Mobs fighting mobs:** mob strikes on mobs by the same combat rules, harm, death and memory by attacker id; rival bands and enemies fight; defence works against mob attackers.
5. **Hunting:** `hunts` prey seen or smelt, the band chases, prey zigzags, a kill is eaten (nothing left), hunters give up past the leash and go home; check wild rabbits turn up near wolves.
6. **Noise:** a shared rule (place, loudness, walls and doors muffling, hearing and distance, a hashed roll); footsteps by pace, armour weight, Sneak and Stealth; blows, spells, doors, calls; Stealth practice; the sneaking eye (the client from the shared rule, wide when a mob is Alert to them, told by the server).
7. **Alertness:** Unaware, Alert, Aware, turning to noises after a reaction delay by Mind; a guaranteed critical (double harm) on Unaware.
8. **Curiosity and alarm calls:** `Doing::Investigating` to where it can see the noise's spot (timid ones move away); hurt social mobs call, and band members who hear come.
9. **Desire basics:** mobs take wanted things from the ground (food for animals, coins and weapons for greedy races), by race.
