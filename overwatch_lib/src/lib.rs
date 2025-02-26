mod overwatch_quick_game;

use overwatch_quick_game::*;

pub fn enter_quick_game() {
    open_game::open_battle_net();
    open_game::open_overwatch();
    enter_quick_game::choose_game_mode();
    enter_quick_game::choose_hero();
}