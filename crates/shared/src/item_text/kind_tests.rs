use super::*;
use crate::cooking::{Source, Tool, Wanted};
use crate::item::ItemKind;
use crate::potion::Water;

fn food(source: Source, key: &str) -> KindToken {
    KindToken::Food(Wanted::One(source, key.to_owned()))
}

#[test]
fn kinds_are_written_and_read_back() {
    let kinds = [
        (
            food(Source::Vegetable, "carrot"),
            "${kind:vegetable:carrot}",
        ),
        (KindToken::Food(Wanted::Any(Source::Egg)), "${kind:egg}"),
        (KindToken::Tool(Tool::Knife), "${kind:knife}"),
        (KindToken::Tool(Tool::Mortar), "${kind:mortar}"),
        (KindToken::Water(None), "${kind:bottle:water}"),
        (
            KindToken::Water(Some(Water::Rain)),
            "${kind:bottle:water:rain}",
        ),
        (
            food(Source::Pantry, "soy_sauce"),
            "${kind:pantry:soy_sauce}",
        ),
    ];
    for (kind, written) in kinds {
        assert_eq!(kind.token(), written);
        assert_eq!(pieces(written), vec![Piece::Kind(kind)]);
    }
}

#[test]
fn malformed_kinds_are_plain_text() {
    for text in [
        "${kind:}",
        "${kind:carrot}",
        "${kind:vegetable:Carrot}",
        "${kind:vegetable:}",
        "${kind:vegetable:carrot:raw}",
        "${kind:knife:sharp}",
        "${kind:bottle}",
        "${kind:bottle:water:mud}",
        "${kind:egg",
        "${kind:vegetable:car rot}",
    ] {
        assert_eq!(pieces(text), vec![Piece::Text(text)], "{text}");
        assert!(!has_tokens(text));
    }
}

#[test]
fn a_broken_kind_does_not_hide_one_after_it() {
    let text = "${kind:egg and ${kind:pan}!";
    assert_eq!(
        pieces(text),
        vec![
            Piece::Text("${kind:egg and "),
            Piece::Kind(KindToken::Tool(Tool::Pan)),
            Piece::Text("!"),
        ]
    );
}

#[test]
fn items_and_kinds_mix() {
    let item = crate::id::Id::for_test(7);
    let text = format!("{} on {}", item_token(item), "${kind:fruit}");
    let any_fruit = KindToken::Food(Wanted::Any(Source::Fruit));
    assert_eq!(
        pieces(&text),
        vec![
            Piece::Item(item),
            Piece::Text(" on "),
            Piece::Kind(any_fruit.clone())
        ]
    );
    assert_eq!(items_in(&text), vec![item]);
    let (token, length) = token_at("${kind:fruit}.").unwrap();
    assert_eq!((token, length), (Token::Kind(any_fruit), 13));
}

#[test]
fn kinds_are_named() {
    assert_eq!(food(Source::Vegetable, "carrot").name(), "Carrot");
    assert_eq!(KindToken::Food(Wanted::Any(Source::Egg)).name(), "Any egg");
    assert_eq!(KindToken::Tool(Tool::Mortar).name(), "Mortar & Pestle");
    assert_eq!(KindToken::Water(None).name(), "Water");
    assert_eq!(KindToken::Water(Some(Water::Salt)).name(), "Salt Water");
}

#[test]
fn stand_ins_are_the_kind_and_the_same_each_time() {
    let carrot = food(Source::Vegetable, "carrot").stand_in().unwrap();
    assert_eq!(carrot.kind, ItemKind::Vegetable);
    let egg = KindToken::Food(Wanted::Any(Source::Egg))
        .stand_in()
        .unwrap();
    assert_eq!(egg.kind, ItemKind::Egg);
    let knife = KindToken::Tool(Tool::Knife);
    assert_eq!(knife.stand_in().unwrap().kind, ItemKind::Knife);
    assert_eq!(knife.stand_in(), knife.stand_in());
    let milk = KindToken::Food(Wanted::Any(Source::Milk))
        .stand_in()
        .unwrap();
    assert_eq!(milk.kind, ItemKind::Bottle);
    assert_eq!(
        KindToken::Water(None).stand_in().unwrap().kind,
        ItemKind::Bottle
    );
    assert!(KindToken::Tool(Tool::Oven).stand_in().is_none());
}

#[test]
fn every_recipe_food_can_be_written() {
    use crate::cooking::{Dishes, RecipeInput, dish_recipe};
    for dish in Dishes::built_in().iter() {
        let recipe = dish_recipe(&dish.key).unwrap();
        for step in &recipe.steps {
            let tool = KindToken::Tool(step.tool);
            assert_eq!(kind_at(&tool.token()).unwrap().0, tool);
            for input in &step.inputs {
                let foods = match input {
                    RecipeInput::Food { foods, .. } | RecipeInput::Liquid { foods, .. } => foods,
                    RecipeInput::Water { .. } => continue,
                };
                for wanted in foods {
                    let kind = KindToken::Food(wanted.clone());
                    assert_eq!(kind_at(&kind.token()).unwrap().0, kind, "{}", dish.key);
                }
            }
        }
    }
}
