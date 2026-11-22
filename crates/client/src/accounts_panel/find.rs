//! The Find page: a character's name to look up, then their account's role,
//! where it came from and whether Discord is linked, and, if the player may
//! change it, a role to give, for how long and why, or Follow Discord to hand
//! it back to the server's Discord.

use shared::protocol::{FoundAccount, RoleSource};

use super::show::numbered;
use super::state::Panel;
use crate::dom::Dom;

pub(super) fn show_find(panel: &mut Panel, dom: &mut Dom) {
    dom.set_flag("accounts/found", "hidden", panel.found.is_none());
    let Some(found) = &panel.found else {
        return;
    };
    dom.set_text("accounts/about", about(found));
    dom.set_flag("accounts/others", "hidden", found.others.is_empty());
    if !found.others.is_empty() {
        let others = format!("Also plays {}.", found.others.join(", "));
        dom.set_text("accounts/others", others);
    }
    dom.set_text("accounts/source", source(found));
    dom.set_flag("accounts/follow", "hidden", !found.may_follow_discord);
    let may_give = !found.may_give.is_empty();
    dom.set_flag("accounts/cannot", "hidden", may_give);
    dom.set_flag("accounts/give", "hidden", !may_give);
    if !may_give {
        dom.set_text("accounts/cannot", why_not(found));
    } else if panel.new_found {
        // Only for a new answer: a choice made since is the dropdown's own.
        let roles = found.may_give.iter().map(|badge| badge.shown());
        dom.set_options("accounts/give_role", numbered(roles));
        let role = panel.give_role.to_string();
        dom.set("accounts/give_role", "value", &role);
    }
    panel.new_found = false;
}

/// Who was found, their account's role, and until when and why if it says.
pub(super) fn about(found: &FoundAccount) -> String {
    let mut about = format!("{}: {}", found.name, found.role.shown());
    if let Some(until) = &found.until {
        about += &format!(" until {until}");
    }
    if !found.reason.is_empty() {
        about += &format!(" ({})", found.reason);
    }
    about
}

/// Where the account's role came from, and whether it has Discord linked.
pub(super) fn source(found: &FoundAccount) -> String {
    let from = match found.source {
        RoleSource::Unset => "No role given",
        RoleSource::Discord => "Role from Discord",
        RoleSource::Hand => "Role given by hand, which Discord never changes",
        RoleSource::Server => "Role claimed or set in roles.ron, which Discord never changes",
    };
    let linked = match found.discord_linked {
        true => "Discord linked",
        false => "no Discord linked",
    };
    format!("{from}; {linked}.")
}

/// Why this account's role can't be changed by the player.
fn why_not(found: &FoundAccount) -> &'static str {
    if found.yours {
        "You can't change your own role."
    } else if found.guest {
        "A guest can't be given a role."
    } else {
        "You can't change this account's role."
    }
}
