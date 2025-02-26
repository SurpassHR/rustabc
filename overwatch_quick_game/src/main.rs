use overwatch_lib::{self, enter_quick_game};

// 使用 open_game 和 enter_quick_game 子模块，使用 use 关键字
// use crate::overwatch_quick_game::open_game;
// use crate::overwatch_quick_game::enter_quick_game;

// use self::overwatch_quick_game::open_game;
// use self::overwatch_quick_game::enter_quick_game;

// 另一种使用 use 的方式
// use self::overwatch_quick_game::{open_game::*, enter_quick_game::*};

// 或者不使用 use 直接在使用的位置通过绝对路径的方式调用
fn main() {
    enter_quick_game();
}
