mod board;
mod player;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement};
use crate::board::{Board, BoardT};
use crate::player::Player;

fn reset() {
    let window = web_sys::window().unwrap();
    let nodes = window.document().unwrap().query_selector_all(".board button").unwrap();
    for i in 0..nodes.length() {
        let node = nodes.item(i).unwrap();
        node.dyn_into::<HtmlElement>().unwrap().remove_attribute("data-player").unwrap();
    }
}

fn win(player: &Player) {
    let window = web_sys::window().unwrap();

    window.alert_with_message(
        &*format!("{} won the game!", match player {
            Player::X => "X",
            Player::O => "O",
            _ => "???",
        })
    ).unwrap();

    reset();
}

fn draw() {
    web_sys::window()
        .unwrap()
        .alert_with_message("It's a draw!")
        .unwrap();
    
    reset();
}

fn setup_board(document: Document) -> Result<Rc<RefCell<Board>>, JsValue> {
    let mut state = Board { current_player: Player::X, state: [[Player::None; 3]; 3] };
    state.clear();

    let state_rc = Rc::new(RefCell::new(state));

    let body = document.body().unwrap();

    let container = document.create_element("div")?.dyn_into::<HtmlElement>()?;
    container.set_class_name("container");
    let board = document.create_element("div")?.dyn_into::<HtmlElement>()?;
    board.set_class_name("board");
    for y in 0..3 {
        let row = document.create_element("div")?;
        for x in 0..3 {
            let col = document.create_element("button")?;
            let col_clone = col.clone();

            let state_clone = Rc::clone(&state_rc);
            let on_click = Closure::<dyn FnMut()>::new(move || {
                let mut state = state_clone.borrow_mut();

                let player = state.set(x, y);
                if player.is_err() {
                    return;
                }

                col_clone.set_attribute("data-player", match player.unwrap() {
                    Player::X => "X",
                    Player::O => "O",
                    _ => "",
                }).unwrap();

                if state.is_draw() {
                    draw();
                    state.clear();
                } else if state.has_winner() {
                    win(state.get_winner());
                    state.clear();
                }
            });
            col.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref())?;
            on_click.forget();

            row.append_child(&*col)?;
        }
        board.append_child(&*row)?;
    }
    container.append_child(&*board)?;
    body.append_child(&*container)?;

    Ok(state_rc)
}

#[wasm_bindgen]
pub fn run() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let app = setup_board(document).expect("board setup unsuccessful");
}
