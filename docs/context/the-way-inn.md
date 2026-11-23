# The Way Inn: plan

Owner's request and decisions (2026-11-22). The Way Inn is the starting place for new players and a hub for everyone, like Honest John's in Feist's Midkemia books: it has always been here, a meeting place for those who travel the Hall of Worlds; magic stops all fighting inside, and weapons are kept. It is built in code from fixed constants like `shared::hall`, with no generator (golden fingerprint test). The Tavern Dev Area (`site::dev_area`, tavern furniture plan PR 8) shows the furniture it will be furnished with.

## Owner's decisions

### Getting in and out
1. An invisible entrance: stepping off either edge of the Hall's walkway halfway between two gates on that side (the void tile in the gates' row, rows -1 and 3, midway between two gates) enters the inn. Nothing drawn marks it; stepping off anywhere else is refused as now (no death).
2. The inn has no direction: you always arrive through its entrance door. Leaving through that door returns you to the walkway tile you stepped off from (saved on the character). Later: a random exit, revisiting places, exact places (with friends).
3. New players start in the inn and are told how to come back (PR 6). In the Hall, at most a very slight worn look of the marble at those edge tiles, drawing only, as a reminder for returning players.

### Layout (sizes Claude's to tune)
4. No windows out: it exists in the void.
5. A 3 × 3 foyer in front of the main room: its outer door is the entrance; the main room's doorway is an archway with no door. One side a chair, a 1 × 1 table and a chest in a line along the wall; the other a 1 × 3 chest; a clear lane up the middle. Only for looks for now; later replaced by the means to choose where you step out to.
6. The main room, larger: 7–8 open booths down the right side (large tables, seats), 7–8 private rooms of the same size down the left, closed off with doors; both rows run back to the wall behind the bar, so the bar sits between them.
7. Many tables up to 2 × 2 (about 30 groups) with chairs, scattered (a fixed seed, never a grid), with walking space round every group; a test checks clear space and paths to every booth, door and the bar.
8. A staircase up in the middle of the main room.
9. The bar across the back between the rows, a flap to get behind, a gap 2 wide, then drink storage against the back wall (wine, spirit and cask racks, always well stocked, refilling). A fireplace in the middle of the back wall, and one either side of the entrance.
10. Doors at either end of the back wall: left into the kitchen (left half: several cooking stations, ranges, hearths, stoves, worktables), right into two rooms, brewing and ageing.
11. The kitchen's back-left corner is left empty for a stair down later (staff rooms, a dungeon, to be decided).
12. A 5 × 5 treasury behind the kitchen: a locked door with a window either side so the kitchen sees in; chests of gold and treasure; only to look at, never to enter.
13. Well lit (chandeliers, sconces, candles on tables), decorations on most walls, interesting floors (planks, rugs, flagstones by the bar and the kitchen, perhaps an inlay).
14. Upstairs: corridors 2 wide in an H, the stairs arriving in the middle of the crossbar, rooms on every side; small near the stairs, larger further out, the corners largest: tiers bought later (with more chests and perks).

### Instances and rules
15. Instances of at most 30 players: the fullest with room, else a new one; you see only your own. Friends and parties joining one later.
16. No fighting anywhere in the inn (attacks, kicks, harmful abilities refused both ways; Heal allowed).
17. Private rooms: one door per tier; every player going through it gets their own copy at the same time (looking out shows the shared instance; looking in from the corridor, your own room). Other tiers' doors locked. A room's storage (chest, wardrobe) is saved with the character, the same on every world.
18. The kitchen, brewery and bar can be used by players (drinks free until there is trade). Staff later.
19. Items dropped in public rooms are cleared after a short time (about 2 minutes, to tune) and destroyed: a stand-in for cleaners.

## PRs

1 → 2 → {3, 4, independent} → 5 → 6.

1. **Layout** (built): `shared::inn`, both floors as levels with rooms marked, the foyer, stairs, doors (the treasury's locked), inside windows, the fixed places furniture goes (`inn::spots`); the Hall's hidden way-ins (`hall::inn_way`); golden fingerprint, print and walk-round tests.
2. **In and out** (built): the Hall's edge into the foyer, leaving back to the same tile, the stairs, the client drawing it, `Where::Inn`, a "To The Way Inn" debug menu entry (and `docs/debug-menu.md`), the Hall's worn-marble hint. Needs 1.
3. **Instances** of 30, no fighting, dropped items cleared (built). Needs 2.
4. **Private rooms** per tier with personal copies and saved storage (built). Needs 2 and tavern furniture PR 7 (chests, wardrobes; built).
5. **Furnishing** everything: tables, booths, the bar, stocked racks, three fireplaces, lights, decorations, rugs, floors, the kitchen, brewing, ageing, the treasury, the foyer. Needs tavern furniture PRs 4–7 (built).
6. **New characters start in the inn** (their first exit: the Hall at their home world's first gate), the note about coming back, a wiki page, context files, decisions. Last.

## Settled in PR 1 (Claude's choices)

- 41 × 50 cells, walls included, both floors the same outline, `x` east and `y` south from the back wall's west end; the entrance in the middle of the south wall. Ground floor: 7 private rooms and 7 booths, each 6 × 4; storage, gap and counter rows 11 to 14; the kitchen also reaching down beside the bar's back to its door (as the brewing room does on the other side), the ageing room through the brewing room.
- Upstairs: 28 bedrooms in 8 strips, tiers 1 to 3 by place from the stairs and the last of each strip (the largest, at the far end) tier 4.
- The bar's counter, gap and storage are in the main room's `RoomId`, as a bar fills vessels only from what is stored in its own room.
- Rooms are carved out of solid wall, so anything not carved is wall; the outside of the building is the level's edge (no void cells). Archway: a floor cell in the foyer's wall, in the foyer's room.
- The Hall's way-ins are a rule beside `hall::tile` (`hall::inn_way`), not a `Tile` of their own: the tile stays `Void`, drawn and stepped on as before until PR 2.

## Settled in PR 2 (Claude's choices)

- The inn is a site (`SiteId::Inn`, `Generator::Inn`, `LevelKind::Inn`), drawn as a castle's stone (`inn::KIND`). Its save and the tile to go back to (`CharacterWorldSave::inn_return`) are kept with the world the player came into the Hall from, so each world has its own inn until PR 3's instances; players see others in the same world's inn only.
- In: the keys (or pad) held outward on the outer half of an edge tile in front of a way in sends `UseExit` (which in the Hall means that); clicking into the void doesn't. Out: any move onto the entrance door (`inn::is_way_out`), opened first like any door; it shuts again behind whoever comes in, so holding the keys on doesn't carry them straight back out (others already inside see it shut only when they next come to the level, until instances). The Leave key does nothing there.
- To The Way Inn (debug menu, Travel): from the Hall, back to the nearest way in; from anywhere else, beside the gate Teleport to World's Hall of Worlds would choose; already inside, where it was going to take you.
- The worn marble: a faint, soft, warm grey patch lying on each edge tile in front of a way in, towards its edge (`hall_view::worn`). Lighting those tiles less didn't show: each corner's light is shared with its neighbours and the void.
- Not yet (PR 3): fighting, kicking doors (the treasury's too) and dropping things are as anywhere else.

## Settled in PR 3 (Claude's choices)

- One inn for every world: its instances (`SiteId::InnInstance`) are kept by the server's home world, and while someone is in one the server counts them as in the home world, keeping the world they came from (`InnState::from`) for saving them, sending them back and its clock. So everyone in an instance shares its doors and floor whatever world they came from, with the code every site uses. Going in and out waits until the player's messages for the tick are read, as it needs every world and everyone else in the inn.
- An instance takes the fullest with room (30), else a new one numbered the lowest free; instances are never saved and close once empty, taking what was left in them. Coming back to a save in the inn (or PR 2's per-world one) goes into an instance as it is then, on the same floor and spot.
- What a player remembers of the layout goes with them from instance to instance; its doors and bags don't.
- No fighting: Attack, Kick and Trample (`Ability::harms`) are refused by the server and greyed on the client in the inn; kicking doors is Kick, so the treasury's is safe too. Heal and everything else work.
- Things lying in a public room (all but private rooms and bedrooms) are destroyed 120 real seconds after the server first sees them there, each on its own clock; a bag left empty goes. Private rooms keep theirs until PR 4 makes them each player's own.
- Debug tools that go elsewhere (Teleport to Hall, Back to Start, sites, Teleport to World, Load) first take the player out of the inn into their own world.

## Settled in PR 4 (Claude's choices)

- Upstairs reworked (owner, 2026-11-23): 4 small rooms along the crossbar's north side and 3 along its south, the strips between the legs behind them, and the two far-north rooms between the legs joined into one with a single door on the east leg (`shared/inn.md`).
- One door a tier, as decided: tier 1 is the crossbar's north room nearest the stairs on the west, tiers 2 and 3 the first two rooms behind it on the west, tier 4 the joined far-north room. The other bedrooms are locked for later (staff, guests, more tiers), and so are tiers 2 to 4's doors until they can be bought: a lock with no key, as the treasury's, the same for everyone (`inn::OPEN_TIER`). Unlocking one per character comes with buying.
- Each own room has a bed, chests and a wardrobe placed in code (`inn::own_room`), more for larger tiers; PR 5 furnishes the rest.
- Everyone walks the instance's level; the room is "their own" by what is kept and seen: what is in its chests and wardrobe and on its floor is kept in a site of their own (`SiteId::InnRoom`, by character), and nobody standing in an own room is shown to anyone else. So looking out shows the instance's corridor and who is in it, and looking in shows your own room, empty of others. Its door is the instance's, shared.
- That site lives in the home world while they are in the inn and is saved with the character (`CharacterSave::inn_room`, upstairs' level save), never as a site's file: the same on every world.
- In the Hall, a click or tap on a way in (the invisible void tile) travels to the tile in front and goes through, as walking off there does; clicks on the void elsewhere still do nothing.
- A private room downstairs is shared by whoever is in it, so things left there are cleared like any public room's.

