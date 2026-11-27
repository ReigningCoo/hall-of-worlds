//! Trades between players and with NPCs, through the real tick (both sides
//! are only reached once every client's messages are read).

use shared::creature::CreatureKind;
use shared::id::Id;
use shared::item::{Item, ItemKind, Rotation, StackLimits};
use shared::poi::PoiKind;
use shared::protocol::{
    ClientLink, ClientMessage, ServerMessage, TradeAsk, TradeEnd, TradeNews, local_pair,
};
use shared::spot::Spot;
use shared::trade::{Offer, Refusal, TradingSetting};
use shared::world::WorldSeed;

use crate::CharacterId;
use crate::ServerWorld;
use crate::connections::tests::{HELLO, tick_both};
use crate::connections::{Client, Connections};

const CASTLE: ClientMessage = ClientMessage::DevEnterSite {
    kind: PoiKind::Castle,
    seed: 7,
    level: 0,
    ruin: 0,
};

/// Two players side by side in a castle, two cells apart, each carrying a
/// dagger (ids 11 and 12).
struct Two {
    connections: Connections,
    world: ServerWorld,
    ends: [ClientLink; 2],
    at: (i64, i64),
}

impl Two {
    fn new() -> Self {
        let mut connections = Connections::default();
        let ends = [1, 2].map(|n| {
            let (end, link) = local_pair();
            connections.add_playing(link, CharacterId(Id::for_test(n)));
            end.send(&HELLO);
            end
        });
        let mut two = Self {
            connections,
            world: ServerWorld::new(WorldSeed::new(1)),
            ends,
            at: (0, 0),
        };
        two.tick();
        for end in &two.ends {
            end.send(&CASTLE);
        }
        two.tick();
        let visit = two.connections.clients[0].visit.as_ref().unwrap();
        two.at = visit.room_to_walk_east(8);
        for n in 0..2 {
            two.teleport(n, (two.at.0 + 2 * n as i64, two.at.1));
            let client = &mut two.connections.clients[n];
            client.inventory.items.clear();
            let dagger = Item::new(Id::for_test(11 + n as u128), ItemKind::Dagger);
            (client
                .inventory
                .give(dagger, StackLimits::NONE, &mut crate::ids::new_id))
            .unwrap();
        }
        two.tick();
        two
    }

    fn teleport(&mut self, n: usize, cell: (i64, i64)) {
        let Spot { x, y } = Spot::middle_of(cell);
        self.ends[n].send(&ClientMessage::DevTeleport { x, y });
    }

    fn client(&self, n: usize) -> &Client {
        &self.connections.clients[n]
    }

    fn id(n: usize) -> Id {
        Id::for_test(n as u128 + 1)
    }

    fn send(&mut self, n: usize, ask: TradeAsk) {
        self.ends[n].send(&ClientMessage::Trade(ask));
    }

    /// One tick, and the trade news each was told.
    fn tick(&mut self) -> [Vec<TradeNews>; 2] {
        tick_both(&mut self.connections, &mut self.world);
        [0, 1].map(|n| {
            let answers = std::iter::from_fn(|| self.ends[n].receive().ok().flatten());
            let news = answers.filter_map(|answer| match answer {
                ServerMessage::Trade(news) => Some(news),
                _ => None,
            });
            news.collect()
        })
    }

    /// Player 0 opens a trade with player 1, who allows it.
    fn opened(&mut self) {
        self.ends[0].send(&ClientMessage::OpenTrade { with: Self::id(1) });
        let [first, second] = self.tick();
        assert!(first.contains(&TradeNews::Asking { with: Self::id(1) }));
        assert!(matches!(&second[..], [TradeNews::Asked { by, .. }] if *by == Self::id(0)));
        let allow = TradeAsk::Answer {
            by: Self::id(0),
            allow: true,
        };
        self.send(1, allow);
        let [first, second] = self.tick();
        assert!(
            first
                .iter()
                .any(|news| matches!(news, TradeNews::Between(_)))
        );
        assert!(
            second
                .iter()
                .any(|news| matches!(news, TradeNews::Between(_)))
        );
    }
}

fn offer(item: u128) -> TradeAsk {
    TradeAsk::Offer(Offer {
        item: Id::for_test(item),
        x: 0,
        y: 0,
        rotation: Rotation::None,
    })
}

fn locks(news: &[TradeNews]) -> Option<(bool, bool)> {
    news.iter().rev().find_map(|news| match news {
        TradeNews::Between(between) => Some((between.mine.locked, between.theirs.locked)),
        _ => None,
    })
}

fn ended(news: &[TradeNews]) -> Option<TradeEnd> {
    news.iter().find_map(|news| match news {
        TradeNews::Ended { why, .. } => Some(*why),
        _ => None,
    })
}

fn refused(news: &[TradeNews]) -> Option<Refusal> {
    news.iter().find_map(|news| match news {
        TradeNews::Refused(why) => Some(*why),
        _ => None,
    })
}

#[test]
fn both_locked_in_swaps_into_the_receive_boxes() {
    let mut two = Two::new();
    two.opened();
    two.send(0, offer(11));
    two.send(0, TradeAsk::Lock(true));
    let [_, second] = two.tick();
    assert_eq!(
        locks(&second),
        Some((false, true)),
        "the other sees it locked"
    );
    // A change to a side unlocks it.
    two.send(
        0,
        TradeAsk::Unsell {
            item: Id::for_test(11),
        },
    );
    two.send(0, offer(11));
    let [first, _] = two.tick();
    assert_eq!(locks(&first), Some((false, false)));
    assert!(
        two.client(0).inventory.find(Id::for_test(11)).is_some(),
        "nothing moved"
    );
    two.send(1, offer(12));
    two.send(0, TradeAsk::Lock(true));
    two.send(1, TradeAsk::Lock(true));
    let [first, second] = two.tick();
    assert_eq!(ended(&first), Some(TradeEnd::Exchanged));
    assert_eq!(ended(&second), Some(TradeEnd::Exchanged));
    for (n, gave, got) in [(0, 11, 12), (1, 12, 11)] {
        let client = two.client(n);
        assert!(client.inventory.find(Id::for_test(gave)).is_none());
        assert!(
            client.inventory.find(Id::for_test(got)).is_none(),
            "in the box"
        );
        let received = &client.trade.received().items;
        assert_eq!(
            received.iter().map(|item| item.id).collect::<Vec<_>>(),
            [Id::for_test(got)]
        );
        assert_eq!(client.trade.to_save().received.items.len(), 1, "saved");
    }
    // No trade starts while the box isn't empty, a shop's neither.
    two.ends[0].send(&ClientMessage::OpenTrade { with: Two::id(1) });
    let [first, _] = two.tick();
    assert_eq!(refused(&first), Some(Refusal::Unclaimed));
    two.send(0, TradeAsk::TakeAll);
    let [first, _] = two.tick();
    assert!(first.contains(&TradeNews::Received(Default::default())));
    assert!(two.client(0).inventory.find(Id::for_test(12)).is_some());
}

#[test]
fn the_setting_decides_how_a_trade_opens() {
    let mut two = Two::new();
    two.ends[1].send(&ClientMessage::SetTrading(TradingSetting::Never));
    let [_, second] = two.tick();
    assert!(second.contains(&TradeNews::Setting(TradingSetting::Never)));
    assert!(two.client(1).never_trades());
    two.ends[0].send(&ClientMessage::OpenTrade { with: Two::id(1) });
    let [first, _] = two.tick();
    assert_eq!(refused(&first), Some(Refusal::Never));

    two.ends[1].send(&ClientMessage::SetTrading(TradingSetting::Allow));
    two.ends[0].send(&ClientMessage::OpenTrade { with: Two::id(1) });
    let [first, second] = two.tick();
    assert!(
        first
            .iter()
            .any(|news| matches!(news, TradeNews::Between(_)))
    );
    assert!(
        second
            .iter()
            .any(|news| matches!(news, TradeNews::Between(_)))
    );
}

#[test]
fn being_denied_closing_or_walking_away_moves_nothing() {
    let mut two = Two::new();
    two.ends[0].send(&ClientMessage::OpenTrade { with: Two::id(1) });
    two.tick();
    two.send(
        1,
        TradeAsk::Answer {
            by: Two::id(0),
            allow: false,
        },
    );
    let [first, _] = two.tick();
    assert_eq!(ended(&first), Some(TradeEnd::Denied));

    two.opened();
    two.send(1, TradeAsk::Close);
    let [first, _] = two.tick();
    assert_eq!(ended(&first), Some(TradeEnd::Closed));

    two.opened();
    two.send(0, offer(11));
    two.tick();
    two.teleport(1, (two.at.0 + 8, two.at.1));
    let [first, second] = two.tick();
    assert_eq!(ended(&first), Some(TradeEnd::OutOfReach));
    assert_eq!(ended(&second), Some(TradeEnd::OutOfReach));
    assert!(two.client(0).inventory.find(Id::for_test(11)).is_some());
    assert!(two.client(0).trade.received().is_empty());
}

#[test]
fn an_npc_takes_what_is_offered() {
    let mut two = Two::new();
    let cell = (two.at.0 + 1, two.at.1);
    let population = two.world.populations.0.values_mut().next().unwrap();
    let npc = population.remake_for_test(0, CreatureKind::Human, cell).id;
    two.ends[0].send(&ClientMessage::OpenTrade { with: npc });
    let [first, _] = two.tick();
    let theirs = first.iter().find_map(|news| match news {
        TradeNews::Between(between) => Some(between.theirs.clone()),
        _ => None,
    });
    assert!(
        theirs.expect("it opens").locked,
        "an NPC is always locked in"
    );
    two.send(0, TradeAsk::Lock(true));
    let [first, _] = two.tick();
    assert_eq!(refused(&first), Some(Refusal::Nothing));
    two.send(0, offer(11));
    two.send(0, TradeAsk::Lock(true));
    let [first, _] = two.tick();
    assert_eq!(ended(&first), Some(TradeEnd::Exchanged));
    assert!(two.client(0).inventory.find(Id::for_test(11)).is_none());
    assert!(
        two.client(0).trade.received().is_empty(),
        "nothing comes back"
    );
}
