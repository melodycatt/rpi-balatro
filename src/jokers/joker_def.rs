use serde::{Serialize, Deserialize};

use crate::{
    game::{deck::DeckType, GameData, RoundData, Game},
    jokers::{JokerType, JokerEnhancements},
    cards::{Card,HandType},

};
macro_rules! def_joker {
    (($ident:ident, $id:expr, $name:expr), 
    { $($field:ident: $type:ident),* }, 
    { 
        buy => ($buyg:ident) $buy:block, 
        sell => ($selg:ident) $sell:block,
        apply => ($appc:ident, $appm:ident, $appg:ident, $apph:ident) $apply:block,
        score => ($scoc:ident, $scom:ident, $scoca:ident, $scog:ident, $scor:ident) $score:block,
        cashout => ($cashr:ident, $cashg:ident) $cashout:block
    }) => {//, sell $sell:block, apply $apply:block, score $score:block, cashout $cashout:block }) => {
//    (($ident:ident, $id:expr, $name:expr), { $($field:ident: $type:ident),* }, fn buy<T: DeckType>(&self, game: &mut Game<T>) $buy:block ) => {
        #[derive(Serialize, Deserialize, Clone, Copy)]
        pub struct $ident {
            enhancements: JokerEnhancements,
            $(pub $field: $type),*
        }

        impl<'de> JokerType<'de> for $ident {
            fn id(&self) -> &'static str { $id }
            fn name(&self) -> &'static str { $name }

            fn buy<T: DeckType>(&self, $buyg: &mut Game<T>) $buy
            fn sell<T: DeckType>(&self, $selg: &mut Game<T>) $sell
            fn apply<T: DeckType>(&self, $appc: &mut f64, $appm: &mut f64, $appg: &mut Game<T>, $apph: HandType) $apply
            fn score<T: DeckType>(&self, $scoc: &mut f64, $scom: &mut f64, $scoca: &mut Card, $scog: &mut Game<T>, $scor: bool) -> bool $score
            fn cashout(&self, $cashr: &RoundData, $cashg: &GameData) -> usize $cashout

            fn enhancements(&mut self) -> &mut JokerEnhancements { &mut self.enhancements }
        }
    };
}


/*def_joker!((Showman, "Showman", "showman") {} {
    fn buy<T: DeckType>(&self, game: &mut Game<T>) {
        game.game_data.hand_size -= 2
    },
    sell {
        game.game_data.hand_size += 2
    },
    apply {
        *chips += 200.0;
    },
    score {

    },
    cashout {

    }
});*/
def_joker!((JNothing, "Nothing Joker", "nothing"), {}, {
    buy => (_game) {},
    sell => (_game) {},
    apply => (_chips, _mult, _game, _hand) {},
    score => (_chips, _mult, _card, _game, _retrigger) { false },
    cashout => (_round, _game_data) { 0 }
});
def_joker!((JBasic, "Joker", "basic"), {}, {
    buy => (_game) {},
    sell => (_game) {},
    apply => (_chips, mult, _game, _hand) { *mult += 1.0 },
    score => (_chips, _mult, _card, _game, _retrigger) { false },
    cashout => (_round, _game_data) { 0 }
});
def_joker!((JStuntman, "Stuntman", "stuntman"), {}, {
    buy => (game) { game.game_data.hand_size -= 2 },
    sell => (game) { game.game_data.hand_size += 2 },
    apply => (chips, _mult, _game, _hand) { *chips += 200.0 },
    score => (_chips, _mult, _card, _game, _retrigger) { false },
    cashout => (_round, _game_data) { 0 }
});
def_joker!((JBaron, "Baron", "baron"), {}, {
    buy => (_game) {},
    sell => (_game) {},
    apply => (_chips, _mult, _game, _hand) { },
    score => (_chips, mult, card, _game, _retrigger) { if card.rank > 10 { *mult *= 1.5; }; false },
    cashout => (_round, _game_data) { 0 }
});