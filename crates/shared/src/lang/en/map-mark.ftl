# Personal map marks: the name the full map's hover box and a mark's popup
# give one (an automatic one on a pile by its icon), and what the popup says
# when no more may be placed by hand.

map-mark-icon = { $icon ->
    [flag] Flag
    [house] House
    [cave] Cave
    [mine] Mine
    [farm] Farm
    [orchard] Orchard
    [bag] Bag
    [logs] Logs
    [camp] Camp
   *[danger] Danger
}
map-mark-name = Your mark: { map-mark-icon }
map-mark-pile = { $icon ->
    [logs] Log pile
   *[other] Pile
}
map-mark-too-many = You have { $most } marks here already: remove one to place another.
