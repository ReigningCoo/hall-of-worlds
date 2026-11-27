# Trading plan

Trading with shops, NPCs and players, in 7 PRs, all in order. The owner's decisions (2026-11-27), planned in chat local_e08c7143 ("Trading"); this file is now the source for them. Marked **(built)** as each PR lands.

## Item values (PR 1, built: `shared::value`)

- Shared and deterministic, in whole numbers: value = (materials + labour) × quality.
  - Materials: weight × a price per kg per material, rarer (tier) dearer.
  - Labour: the time to make it **without any bonuses** (a skilled crafter is quicker but paid for the full time); crafting reuses these figures later.
  - Raw goods (herbs, fruit, vegetables, fungi) by abundance; liquids by the ml; containers add their contents.
- Quality doubles: Poor 1×, Common 2×, Uncommon 4×, Rare 8×, Epic 16×, Legendary 32× (Artifact 64×, Claude). Items without quality count as Common.
- Unique and set items: their rules are the Equipment quality bonuses chat's (`shared::item_set`); they still have values, and nothing there stops trading them.
- Anchored so a Common apple is 1 copper (a Poor one ½). Values are kept in fine units (thousandths of a copper); only what is paid or shown is rounded **up** (a stack's total, a trade's total, "each" on its own).
- Item popup, bottom right: coin pictures and numbers, zero denominations left out; stacks show "gT sT cT (gE sE cE each)", "each" only for more than one.

## Shops

- Buy at 50%, sell at 150%. Town standing (the quest `Standing` bands) moves both by up to 25%, the Trading skill by up to 25%, added (both full: 100% both ways).
- Things the shop doesn't normally sell: a further 50% off what it pays, after everything else; it resells them at its normal price.
- Sold items join its stock but time out sooner than its own stock (Claude's guess 7 game days, to balance).
- Payment is automatic on Complete trade.
- Coins added above the price: standing up by any amount; a tip of up to 5% gives Trading practice, more gives **negative** practice (new to the skill system).
- Coins below: accepted up to 5% short at full Trading skill (scaled down with skill; a fixed rule, no dice, so the client can say beforehand), with practice when accepted; refused beyond.

## One trade window

- T by default, and a trade icon at the bottom left beside the others (the owner is also planning a FAB menu).
- A dropdown at the top: players, shops (by door) and NPCs within 5 cells; anyone set to Never greyed out and unselectable; disabled once a trade starts, until both sides are clear.
- Your side 8 × 8: items stay in your inventory marked with a bag in the corner, still taking space. Their side: 8 × 8 for players and NPCs, the whole stock grid for a shop. From a shop you drag straight into your inventory: unpaid items highlighted (solid background), unusable until paid; you can't drop one you can't afford (coins + pending sales − pending buys).
- Middle: the value of each side, who gains and by how much. Shops: a Complete trade button and the total due or received at the bottom right. Players and NPCs: a Trade toggle; on, your side goes slightly darker with a solid surround (the other sees it); any change to your side turns your toggle off.
- Consistency (owner): every trade, shops included, only happens on Complete or both locked in. Closing the window, walking out of range, leaving the level or disconnecting resets everything, nothing transferred.
- Coins paid and change given automatically, highest coins first, allowed over the carry limit.

## Player trades

- Both locked in: the middle and your side vanish (your items go), and their side becomes your **receive box**: take things out by hand or with Take all (whatever doesn't fit stays).
- The receive box takes no inventory space or weight, is saved with the character, and blocks starting any trade (shops too) until empty, so it can't be used as storage.
- Trading setting (Settings › Gameplay, "Trading"): Allow (opens the window by itself), Ask (the default: a popup to Allow or Deny; the asker sees their side darkened with "Asking..."), Never. Kept on the character by the server.

## Other

- The Trading skill, under Social.
- Beggars (`CreatureKind::Beggar`): the first NPCs in the list; giving them coins (their side empty) raises town standing slightly, by amount, with a daily cap.
- New characters get 1 gold for testing (to be lowered to a real starting amount); the Nearby panel (`inventory/give.rs`) is removed (speaking to and trading with any NPC later).

## PRs

| # | What | |
| - | ---- | - |
| 1 | Item values and the popup's value; this file | built |
| 2 | 1 gold for new characters, the Nearby panel removed, the Trading skill under Social with negative practice, the T key and icon | |
| 3 | The server's trade core and shop trades: rates, standing, not normally sold, sold stock timing out, tips, haggling | |
| 4 | The window for shops | |
| 5 | The server's player and NPC trades: asking, locking in, resets, the setting, the receive box saved | |
| 6 | The window for players: sides, toggle, "Asking...", the request popup, the receive box with Take all, the Gameplay setting | |
| 7 | Beggars | |
