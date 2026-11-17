# Wiki style: how the players' pages are written

The rules every page of the players' wiki (`docs/`, outside `docs/context/`) keeps to, first set down for the armour pages. Other tasks use them and add to them. **Each rule is checked off and signed off by the owner**; a task that relies on them doesn't start until they are. A new rule goes in unticked until the owner ticks it.

Signed off by the owner on 2026-11-04.

## Where pages go

- [x] A subject with several pages gets its own folder (`docs/armour/`) with an `index.md` overview, which has `has_children: true`, and one page per part, each with `parent:` set to the overview's title.
- [x] Front matter: `title`, `nav_order`, and `parent` / `has_children` where they apply. A page's place in the menu follows how a player would come to it, not when it was written.
- [x] A subject with only one page sits at the top of `docs/`, beside the others, with a `nav_order` next to the subject a player would look at with it (`footwear.md`, 2, after Armour at 1 under Items). Signed off on 2026-11-14.
- [ ] Every page belongs to one of three sections, each a page with `has_children: true` and a link in the top bar (`aux_links` in `docs/_config.yml`), which work like tabs: **Gameplay** (`gameplay.md`: playing the game, and the **Items** group, `items.md`, for things found, worn and carried), **Client** (`client-section.md`: installing, running and the client's own tools) and **Server** (`server-section.md`: hosting and its settings). Only the Introduction and Credits and licence sit outside them. A page that would suit two sections goes in one and the other links to it (Saves and data is under Client; Server links to it).
- [ ] A page's front matter names its `parent` and, below a section, its `grand_parent` too (Head armour: `parent: Armour`, `grand_parent: Items`). Files stay where they are when they join a section, so addresses don't change.
- [ ] `nav_order` counts in whole numbers within each parent; a new page is fitted in by renumbering its neighbours rather than with a fraction.
- [x] File names are lower case words joined by hyphens (`full-sets.md`); titles are plain words ("Torso armour").

## What goes in

- [x] What the game makes now: the kinds of thing, what they are called, what they look like in each style, what they can be made of, and roughly what they weigh.
- [x] Rules a player feels: which body place and layer a thing takes, what clashes with what, what goes over or under what.
- [x] Names as the game shows them, in the same spelling and capitals, with a plain description beside any unfamiliar one ("Kusazuri: hanging panels round the hips").
- [x] Weights as typical figures for an ordinary-sized piece in its usual material, rounded (about 1.2 kg), with a line saying that size and material change them.

## What stays out

- [x] Questions, decisions, who decided what, and research notes: those live in `docs/context/`.
- [x] Evidence marks, sources and article names, Ngram figures.
- [x] ~~Pictures and screenshots, while the game changes quickly.~~ Replaced by the next rule (owner, 2026-11-17).
- [ ] Screenshots of play stay out. Pictures of items are allowed when made by `cargo xtask wiki --pictures` (the game's own item pictures, on a clear background, in `docs/assets/<subject>/`), never drawn or taken by hand, so they change with the game.
- [x] Code names (`TorsoForm`, `Construction::PlatesInCloth`), file paths and numbers only code needs (roll weights, millimetres on the body frame).
- [x] Plans and things not made yet, except one line where a player would otherwise wonder ("Gem sockets are planned").

## How each page is laid out

- [x] A subject's page may use sub-headings under Kinds (such as **Shape** and **Names** for footwear) where its things vary in more ways than one table shows. Signed off on 2026-11-14.
- [x] One template for every page of a subject. For armour: a short introduction (what the location covers, its layer, what goes under and over it), then **Kinds** (a table), **Styles** (a table), **Materials**, **Weights**. A heading with nothing to say is left out rather than left empty.
- [x] Tables have a first column naming the thing and one row per thing; the same columns in the same order on every page of a subject.
- [ ] A page per item (herbs first): made by `cargo xtask wiki` from the game's data, never by hand, with a comment saying so. Front matter `parent` (the subject) and `grand_parent` (Items); `nav_order` by name. Then the picture, the name as a heading, the Latin or other second name in italics with the plain description, a link back to the overview, and a two-column table of facts (for herbs: Rarity, Grows in, Warmth, Part used, Weight, Plant, Growth, Picking, Picked early; for weapons, which have no second name: Blade, Handle, Hands, Edges, Curve, Guard, Weight, Takes up, from many made the way a shop would, so the usual first and "some" for the rest). A subject with groups (weapons' families) gets a page per group, generated too, with `parent` the group and `grand_parent` the subject. What a player learns by playing (herbs' effects) stays out.
- [ ] A subject's overview may list its items in a table linking each to its page; a table made from the game's data is written between two comments by `cargo xtask wiki`, and the words round it by hand.
- [x] The overview page explains what the subject's pages share once (layers, styles, constructions, materials), and the other pages link to it rather than repeat it.

## How it reads

- [x] Plain British English, as the rest of the site: short sentences, everyday words, no jargon a player wouldn't know without explaining it.
- [x] Addressed to the player ("you can wear a surcoat over it"), in the present tense, about what is true now.
- [x] Links between pages are relative (`[Sets](sets)`), and every page links back to its overview.
