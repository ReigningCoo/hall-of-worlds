//! The headings Settings shows actions under.

/// The headings actions are shown under.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    Movement,
    Camera,
    Targeting,
    ActionBar,
    Interface,
}

impl Group {
    pub const ALL: [Self; 5] = [
        Self::Movement,
        Self::Camera,
        Self::Targeting,
        Self::ActionBar,
        Self::Interface,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Movement => "Movement",
            Self::Camera => "Camera",
            Self::Targeting => "Targeting",
            Self::ActionBar => "Action Bar",
            Self::Interface => "Interface",
        }
    }
}
