# Trading plan

Trading with shops, NPCs and players, in 7 PRs, all in order, all built. The owner's decisions (2026-11-27), planned in chat local_e08c7143 ("Trading"); this file is now the source for them. Marked **(built)** as each PR lands.

## Item values (PR 1, built: `shared::value`)

- Shared and deterministic, in whole numbers: value = (materials + labour) × quality.
  - Materials: weight × a price per kg per material, rarer (tier) dearer.
  - Labour: the time to make it **without any bonuses** (a skilled crafter is quicker but paid for the full time); crafting reuses these figures later.
  - Raw goods (herbs, fruit, vegetables, fungi) by abundance; liquids by the ml; containers add their contents.
- Quality doubles: Poor 1×, Common 2×, Uncommon 4×, Rare 8×, Epic 16×, Legendary 32× (Artifact 64×, Claude). Items without quality count as Common.
- Unique and set items: their rules are the Equipment quality bonuses chat's (`shared::item_set`); they still have values, and nothing there stops trading them.
- Anchored so a Common apple is 1 copper (a Poor one ½). Values are kept in fine units (thousandths of a copper); only what is paid or shown is rounded **up** (a stack's total, a trade's total, "each" on its own).
- Item popup, bottom right: coin pictures and numbers, zero denominations left out; stacks show "gT sT cT (gE sE cE each)", "each" only for more than one.

## Shops (PR 3, built: `shared::trade`, the server's `connections/trade.rs`)

- Buy at 50%, sell at 150%. Town standing (the quest `Standing` bands) moves both by up to 25%, the Trading skill by up to 25%, added (both full: 100% both ways).
- Things the shop doesn't normally sell: a further 50% off what it pays, after everything else; it resells them at its normal price.
- Sold items join its stock but time out sooner than its own stock (Claude's guess 7 game days, to balance).
- Payment is automatic on Complete trade.
- Coins added above the price: standing up by any amount; a tip of up to 5% gives Trading practice, more gives **negative** practice (new to the skill system).
- Coins below: accepted up to 5% short at full Trading skill (scaled down with skill; a fixed rule, no dice, so the client can say beforehand), with practice when accepted; refused beyond.
- As built (Claude's choices, to balance): standing takes off 0, 6, 12, 18, 25% from Stranger to Champion; the player offers a sum to pay (`TradeAsk::Pay`), its gap from what is due being the tip or haggle (the shop paying the player works the same way); a point of standing per 1,000 copper tipped, past what acts give; what a shop pays is rounded **down** to whole copper (rounding it up would let a pinch of anything sell for a copper again and again); what is bought is kept out of the inventory until completed, so nothing can be used unpaid (since trading tweaks PR 2, in the Buying box: see below); coins and anything holding coins or a quest item can't be sold; trading is within 5 cells of a shop's door.

## One trade window (PRs 4 and 6 built: client `trade`)

- T by default, and a trade icon at the bottom left beside the others (the owner is also planning a FAB menu).
- A dropdown at the top: players, shops (by door) and NPCs within 5 cells; anyone set to Never greyed out and unselectable; disabled once a trade starts, until both sides are clear.
- Your side 8 × 8: items stay in your inventory marked with a bag in the corner, still taking space. Their side: 8 × 8 for players and NPCs, the whole stock grid for a shop. From a shop you drag straight into your inventory: unpaid items highlighted (solid background), unusable until paid; you can't drop one you can't afford (coins + pending sales − pending buys).
- Middle: the value of each side, who gains and by how much. Shops: a Complete trade button and the total due or received at the bottom right. Players and NPCs: a Trade toggle; on, your side goes slightly darker with a solid surround (the other sees it); any change to your side turns your toggle off.
- Consistency (owner): every trade, shops included, only happens on Complete or both locked in. Closing the window, walking out of range, leaving the level or disconnecting resets everything, nothing transferred.
- Coins paid and change given automatically, highest coins first, allowed over the carry limit.
- As built for shops (PR 4, Claude's choices for the owner to check): a click on a shop's door opens the trade window on it, and the talk panel's Trade tab is gone; opening the window (T) asks for the nearest shop and opens the Character panel to drag into. Carried things let go on either side are sold; the middle shows each side's worth and what the shop pays or asks, and who gains (worth received − worth given − coins paid). Coins are written as words for now, and there is no way yet to offer more or less than is due (tips and haggling: the server takes `TradeAsk::Pay`).

- As built for characters (PR 6, Claude's choices for the owner to check): the dropdown lists players first, then shops, then NPCs, each nearest first, alike NPCs numbered ("Human 2"); opening the window still asks for the nearest shop only, never a player. The Trade toggle is a button that stays pressed while locked in, with "They are ready to trade." under it once the other is; a locked side is a shade darker with a 3-pixel ink edge, and their side shows so while being asked, with "Asking..." above it. The request is a small requester (Allow, Deny; a press outside denies). The receive box takes the window's whole place; dragging one thing into the inventory takes it out there. The Trading setting is a row in Settings › Gameplay, shown only while playing.

## Trading tweaks (owner, after testing; 3 PRs, chat local_e08c7143)

- PR 1 (built, #732): even grid lines; shopkeepers named in the dropdown ("Elien Radwell, tavern keeper").
- PR 2 (built: `shared::trade` `buying.rs`, the server's `connections/trade/change.rs`, `settle.rs`): **the Buying box.** Stock is bought into it, 8 wide with at least 4 rows, a row more when something added wouldn't fit, back to 4 when emptied (Claude's: its rows follow its lowest thing). Things lie in it like an 8-wide grid (x, y, rotation), as a player trade's sides do; moved within it they are bought there again, back on the stock unbought. Nothing bought goes into the inventory before the trade: on completing it goes wherever it fits, the rest into the receive box (Take all). Coins and sums unchanged. A buy with no place goes in the first free one (for right-click). Unpaid things no longer appear in the inventory; for now the box sits above the stock in the window.
- PR 3: the window redesigned: Selling and Buying labels, Buying above an 8-wide stock, coin totals under each box, the Trade button between with the signed difference, no middle values, right-click (two taps) adds and removes, a bag icon top left; the same layout for player trades.

## Player trades (PR 5 built on the server: `connections/trade/deals.rs`, `shared::trade::Side`, `Received`)

- Both locked in: the middle and your side vanish (your items go), and their side becomes your **receive box**: take things out by hand or with Take all (whatever doesn't fit stays).
- The receive box takes no inventory space or weight, is saved with the character, and blocks starting any trade (shops too) until empty, so it can't be used as storage.
- Trading setting (Settings › Gameplay, "Trading"): Allow (opens the window by itself), Ask (the default: a popup to Allow or Deny; the asker sees their side darkened with "Asking..."), Never. Kept on the character by the server.
- As built on the server (PR 5, Claude's choices for the owner to check): trades with characters only on a level (not the Hall or world map); each side's things placed in its 8 × 8 like a grid (`Offer`), quest items refused, coins allowed; someone trading, being asked or with things in their box is `Busy`; NPCs are the people kinds (an alignment) not unfriendly to the player and in sight: their side is empty and always locked, so locking in gives them what is offered as Give Item would, and nothing comes back (beggars build on this in PR 7); taking one thing out of the box goes where asked (no stacking onto what is there), Take all wherever it fits.

## Other

- The Trading skill, under Social (`social/trading`, built; it was already in the tree). Negative practice (`Skills::unpractise`, built in PR 2): flows up halving as practice does, never below 1 on a known node, the best kept.
- Beggars (`CreatureKind::Beggar`, in every town): the first NPCs in the list; giving them coins (their side empty) raises town standing slightly, by amount, with a daily cap. As built (PR 7, Claude's figures to balance): a point of standing a 10 copper, at most 3 points a town a UTC day (a fifth of a quest), counting past Known as tips do; by Give Item too; other gifts to a beggar still give the 2 points of a helpful act (the server's `quests/alms.rs`).
- New characters get 1 gold for testing (to be lowered to a real starting amount; the server's `starting_kit.rs`); the Nearby panel (`inventory/give.rs`) is removed (speaking to and trading with any NPC later). Built in PR 2, with the T key and the trade button opening an empty window (`client: trade`) for PRs 4 and 6 to fill.

## PRs

| # | What | |
| - | ---- | - |
| 1 | Item values and the popup's value; this file | built |
| 2 | 1 gold for new characters, the Nearby panel removed, the Trading skill under Social with negative practice, the T key and icon | built |
| 3 | The server's trade core and shop trades: rates, standing, not normally sold, sold stock timing out, tips, haggling | built |
| 4 | The window for shops | built |
| 5 | The server's player and NPC trades: asking, locking in, resets, the setting, the receive box saved | built |
| 6 | The window for players: sides, toggle, "Asking...", the request popup, the receive box with Take all, the Gameplay setting | built |
| 7 | Beggars | built |
