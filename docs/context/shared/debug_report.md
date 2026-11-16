# shared: debug_report

Debug reports for bug reports: what the client's Debug Info panel (`client: debug_info`) writes about where the player is and what they see, as one string to paste, and reading one back (for staff going there, PR 3 of the plan). Plain data and its encoding; nothing here goes anywhere or is trusted.

## Files

- `crates/shared/src/debug_report.rs`: the module's docs and re-exports. Tests in `debug_report/tests.rs`.
- `debug_report/report.rs`: `DebugReport` (`build`, `utc` seconds, `who`, `place`, `clock`, `camera`, `flags`, `dev`, `problem`), `Build` (`version`, `protocol`, `Platform` Native or Web, `system`), `Who` (character id, `CreatureKind`, `Gender`, `Difficulty`), `Where` (`WorldMap { seed, x, y }`, `Wilderness { seed, at, heading, poi }`, `Site { seed, x, y, chunk, kind, ruined, level, levels, at, heading }`, `Hall { origin, at, heading }`), `ReportClock` (game minutes, `ClockRate`), `Camera` (turn, tilt, zoom), `DebugFlags` (God Mode, Truesight, Freeze Mobs), `DevPlace` (`Site { kind, seed, ruin }` of Generate POI, `Area(DevArea)`). Serde, read as JSON.
- `debug_report/code.rs`: `encode` (`PREFIX` `HOW1.` + URL-safe base64 without padding of the JSON deflated in zlib's wrapping, level 10), `decode` (whitespace ignored; `NotAReport`, `UnknownVersion`, `TooLong` past `MOST_LETTERS`, `Damaged`: bad base64, a wrong Adler-32 checksum, inflating past 64 KiB, or JSON not a report), `DecodeError` (with `Display` wording).

## Public API

- Everything above.

## Depends on

- `creature`, `difficulty`, `facing` (`Heading`), `gender`, `poi` (`ChunkPos`, `PoiKind`), `protocol` (`CharacterId`), `rng` (`Seed`), `site` (`DevArea`), `spot`, `world` (`ClockRate`); `serde_json`, `base64`, `miniz_oxide` (MIT/Apache-2.0, MIT/Zlib/Apache-2.0).

## Used by

- `client: debug_info`.

## Notes

- Squeezing only shortens and obscures the string: anything acting on a report must be checked by the server.
- A new layout of the report means a new prefix (`HOW2.`): `decode` names versions it can't read rather than misreading them.
- Never put the email address, the server address or an account's name in a report.
