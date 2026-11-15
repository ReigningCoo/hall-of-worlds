//! A tunic's name from the research's signatures, and its weight.

use super::{Closure, Cut, Length, Neck, Sleeves, Tunic, TunicDecoration};

impl Tunic {
    /// Its name from the research's signatures; "Tunic" when none fits.
    pub(in crate::clothing) fn name(&self) -> &'static str {
        let (length, sleeves) = (self.length, self.sleeves);
        if self.cut == Cut::Draped {
            return match sleeves {
                Sleeves::OneShoulder => "Exomis",
                _ if self.overfold => "Peplos",
                _ => "Chiton",
            };
        }
        match self.decoration {
            TunicDecoration::BrocadedPanels => return "Huipil",
            TunicDecoration::Smocking => return "Smock-Frock",
            TunicDecoration::Clavi if sleeves == Sleeves::Wide => return "Dalmatic",
            TunicDecoration::Clavi if length == Length::Knee && sleeves == Sleeves::Short => {
                return "Tunica";
            }
            _ => {}
        }
        if self.neck == Neck::OffCentre {
            return "Kosovorotka";
        }
        if self.neck == Neck::ButtonedSlit && length == Length::Knee && sleeves == Sleeves::Wrist {
            return "Kurta";
        }
        match self.cut {
            Cut::Fitted if sleeves == Sleeves::Hanging => "Bliaut",
            Cut::Fitted if self.closure == Closure::Buttoned => "Cotehardie",
            Cut::Fitted if self.closure.laced() && length >= Length::Calf => "Kirtle",
            Cut::Gored if sleeves == Sleeves::None && length == Length::Ankle => "Sarafan",
            Cut::Gored if sleeves == Sleeves::Wrist && length >= Length::Calf => "Cotte",
            Cut::Rectangles if sleeves == Sleeves::None && length == Length::Ankle => "Stola",
            Cut::Rectangles if sleeves == Sleeves::Wrist && length >= Length::Ankle => "Thawb",
            _ => "Tunic",
        }
    }

    /// Grams in plain linen at an ordinary size.
    pub(in crate::clothing) fn base_grams(&self) -> u32 {
        let body = match self.length {
            Length::Hip => 300,
            Length::Thigh => 360,
            Length::Knee => 450,
            Length::Calf => 540,
            Length::Ankle => 620,
            Length::Floor => 700,
        };
        let more = match self.sleeves {
            Sleeves::Wide => 80,
            Sleeves::Hanging => 120,
            Sleeves::LacedOn => 20,
            _ => 0,
        } + match self.cut {
            Cut::Rectangles => 0,
            Cut::Gored => 60,
            Cut::Fitted => 40,
            Cut::Draped => 100,
        } + if self.overfold { 80 } else { 0 };
        let less = match self.sleeves {
            Sleeves::None | Sleeves::OneShoulder => 80,
            Sleeves::Short | Sleeves::Pinned => 50,
            Sleeves::Elbow => 25,
            _ => 0,
        };
        body + more - less
    }
}
