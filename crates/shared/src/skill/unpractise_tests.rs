use super::*;

fn trading() -> SkillId {
    SkillId::find("social/trading").expect("Trading under Social")
}

#[test]
fn trading_is_under_social() {
    let parent = trading().parent().expect("a parent");
    assert_eq!(parent.name(), "Social");
    assert_eq!(trading().name(), "Trading");
}

#[test]
fn negative_practice_flows_up_halving() {
    let mut skills = Skills::default();
    skills.practise(trading(), 1000);
    skills.unpractise(trading(), 400);
    assert_eq!(skills.practice(trading()), 600);
    let social = trading().parent().expect("Social");
    assert_eq!(skills.practice(social), 500 - 200);
    // The best is kept: practising back is doubled.
    assert_eq!(skills.best(trading()), 1000);
}

#[test]
fn negative_practice_never_forgets_or_teaches() {
    let mut skills = Skills::default();
    skills.unpractise(trading(), 100);
    assert!(!skills.knows(trading()));
    skills.practise(trading(), 10);
    skills.unpractise(trading(), 1000);
    assert_eq!(skills.practice(trading()), 1);
}
