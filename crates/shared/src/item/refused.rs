//! `MoveRefused`: why a move or an addition was refused.

/// Why something can't go where it was asked to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveRefused {
    /// The item isn't there to move.
    NoSuchItem,
    /// The grid isn't there, or what was named has none.
    NoSuchGrid,
    /// It would be inside itself.
    IntoItself,
    /// It is too big for the container's outside.
    TooBig,
    /// It would go too many grids deep.
    TooDeep,
    /// It would stick out of the grid, or lie on something else.
    NoRoom,
    /// It is too heavy for a container it would end up in.
    TooHeavy,
    /// It doesn't look the same as the stack, or the stack has no room for it.
    NoStackRoom,
    /// It can't be tied to a belt (or is worn some other way).
    NotForBelt,
    /// Only an empty bag can be tied to a belt.
    NotEmpty,
    /// The belt has no such slot, or there is no belt.
    NoSlot,
    /// Untying it would leave things off the end of the own grid.
    BagInUse,
    /// With less Strength the own grid would lose cells that hold something
    /// (`Inventory::check_own_cells`).
    StrengthInUse,
    /// Without this belt, or with a worse one, bags would be left without slots.
    BeltInUse,
    /// The container takes only some kinds, not this one (a keyring: keys).
    NotTaken,
    /// A holder of this kind is tied already: only one of each at a time.
    HolderTied,
}
