//! Negative practice: something done badly enough takes practice away (trading
//! plan: tipping a shop more than 5% teaches the wrong lesson). It flows up the
//! tree as practice does, halving at each level.

use super::SkillId;
use super::practice::Skills;

impl Skills {
    /// Takes `amount` from this node, half that from its parent, and so on until it
    /// rounds down to nothing or reaches the top. Only known nodes lose any, and
    /// never below 1 practice, so a known node stays known (as with rust). The best
    /// a node reached is kept, so practising it back is quick.
    pub fn unpractise(&mut self, id: SkillId, amount: u32) {
        let mut amount = amount;
        for node in id.upwards() {
            if amount == 0 {
                break;
            }
            if let Some(learned) = self.0.get_mut(&node) {
                learned.practice = learned.practice.saturating_sub(amount).max(1);
            }
            amount /= 2;
        }
    }
}
