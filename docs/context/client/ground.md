# client: ground

Bags on the ground of a site, where the server says or the character remembers: a canvas sack by how full it looks, with a sparkle behind while in sight. What is in one is shown by `inventory`, which also has the Drop icon. Native and web.

## Files

- `crates/client/src/ground/mod.rs`: `GroundPlugin`, `Bags`, `Heading`, `BagAssets`, `make_assets`, `read_server`, `find_hovered`, `change_of_mind`, `follow_click`, `open_on_arrival`, `show_bags`, `GroundDebug` (native). Beside it: `parts.rs`, `assets.rs`, `spawn.rs`, `server.rs`, `clicks.rs` (the hover box from each bag's raised ground), `show.rs`, `raise.rs` (`raise_bags`: each bag on the ground under it, `common::DrawnGround::drawn_between`, and moved by the Hall's bend, `common::HallBend`, when bags, heights or the bend change; `clicks.rs` measures the moved place too), `debug.rs` (`GroundDebug`, native only).
- `crates/client/src/ground/cask.rs`: a cask standing (`Load::Cask`): `cask_units`, `cask_models` (`item_art`'s mesh at creatures' scale × 1.3, and a copy with normals out from its middle to grow its outlines without cracks).
- `crates/client/src/ground/logs.rs`: logs lying (`Load::Logs`, woodcutting PR 4): `log_models` (1, 2 side by side, or 3 for three or more, two below and one on top, of `item_art::log_mesh` at `GROUND_DETAIL`, at casks' scale, a little apart so one's outline never hides another; outline normals out from each log's middle), `pile_units`. Their meshes are made the first time a pile is seen (`assets::ensure_meshes`), there being 60 woods × 3; drawn with `log_lit`, rough with no sheen.
- `crates/client/src/ground/bag.rs`: without Bevy: `Model`, `bag`, `outline`, `star`, `size_of`, the shapes. Tests. In `bag/`: `model.rs`, `size.rs`, `shape.rs`, `outlines.rs`.

## Public API

- `GroundPlugin`, `GroundDebug` (message, native): `ShowLooks`, for `dev_tools`.

## Depends on

- `common` (`HoveredBag`, `HoveredLoad` (set with it: how it looks, for `hover`), `ClickMode`, `MapPosition`, `SpotPosition`, `PlayerSize`, `PointerPosition`), `connection` (`FromServer`), `dungeon_map` (`CELL_SIZE`, `Shown`), `shared::ground` (`Load`, `in_reach`), `shared::spot`, `inventory` (`OpenCharacter`), `player` (`Player`, `Arrived`, `TakeStep`, `TravelTo`, `PlayerSystems`), `item_art` (`CaskMesh`, `cask_mesh`, `cask_height`, `cask_radius_share`, `log_mesh`, `LogMesh`, `GROUND_DETAIL`, `LOG_LENGTH`, `LOG_RADIUS`), `shared::cask`, `view3d` (`MODEL_SCALE`, `ClickedToken`, `View3dSystems`, `ViewCamera`, `XrayMaterial`, `XRAY_WHITE`), `shared::ground`, `shared::protocol`.

## Used by

- `dev_tools`. `dungeon_map` reads `common::HoveredBag`; `hover` reads `common::HoveredLoad`.

## Notes

- Owner's design. The look is only what the server last said (`GroundSeen`, `GroundGone`): out of sight it says nothing, so a remembered bag keeps its look, in sepia with no sparkle, until seen again, though it may have gone. Whether a bag is in sight is `Shown::sight`, as for tokens.
- Fuller looks are drawn bigger, from half size for the flattest to full size for the most bulging (`Size::scale`). A sparkle is half size on the biggest bag and shrinks with the bag, while its outline keeps its width as drawn. Bags are kept and drawn where they are, in units (free positions PR 6), each in sight while its cell is. Each look has `VARIETIES` shapes, and which one and how it is turned come from a hash of where it is, so bags side by side differ and a bag doesn't change as it fills unless its look does. Lit by real lights, as tokens are; grey ones are unlit.
- Seen through whatever hides it (`view3d`'s x-ray), in white, in sight or remembered: the bag's mesh as a faint fill and its outline mesh as the outline. Clicks reach it there too. Sparkles aren't.
- The sparkle is three stars facing the camera, behind the bag along the ground, so it shows above it. It goes by real time and is unlit, to be seen in the dark. Bag and stars always have a black outline (inverted hull; larger black stars behind them), wider while hovered, with the same check on screen for mouse and finger.
- Clicking a bag in reach (`shared::ground::in_reach` for `PlayerSize`) opens the Character panel; otherwise it walks to the bag's cell along a found route and opens it on `Arrived` there (a cell's middle reaches all of it), where `inventory` shows the ground section. A step of the player's own or a journey elsewhere drops that, and keyboard movement never opens it. Other click modes (teleporting) ignore bags. By touch, a tap arrives as a click, and there is no hover but while the finger is down.
- A pile holding a cask (`Load::Cask`, the biggest) is drawn as that cask standing, unturned variety 0, coloured by its mesh alone (`cask_lit`, `cask_remembered`: no weave), with the same outlines, memory sepia, x-ray and sparkle as a bag; its size for clicks and the sparkle comes from its height, at full scale from the fullest bag's height up. Show Looks (`GroundDebug`) adds one cask of each size after the bags, then piles of 1, 2 and 3 logs.
