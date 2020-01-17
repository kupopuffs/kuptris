#[macro_use]
extern crate stdweb;

mod canvas;
mod direction;
mod snake;

use canvas::Canvas;
use direction::Direction;
use snake::Snake;

use stdweb::traits::*;
use stdweb::web::{event::{KeyDownEvent, TouchStart, TouchEnd}, IEventTarget};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::document;
use stdweb::unstable::TryInto;

use std::cell::RefCell;
use std::rc::Rc;
use std::cmp;

const CANVAS_ELEMENT_ID: &str = "#canvas";
const GAME_LOOP_TIMEOUT: u32 = 100;

fn setup_canvas_size() {
    let canvas: CanvasElement = document()
        .query_selector(CANVAS_ELEMENT_ID)
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let inner_width = stdweb::web::window().inner_width() as u32;
    let inner_height = stdweb::web::window().inner_height() as u32;
    let min = cmp::min(inner_width, inner_height);

    canvas.set_width(min);
    canvas.set_height(min);

}

fn main() {
    stdweb::initialize();

    setup_canvas_size();

    let canvas = Canvas::new(CANVAS_ELEMENT_ID, 20, 20);
    let snake = Rc::new(RefCell::new(Snake::new(20, 20)));

    snake.borrow().draw(&canvas);

    stdweb::web::document().add_event_listener({
        let snake = snake.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "a" => snake.borrow_mut().change_direction(Direction::Left),
                "d" => snake.borrow_mut().change_direction(Direction::Right),
                "s" => snake.borrow_mut().change_direction(Direction::Down),
                "w" => snake.borrow_mut().change_direction(Direction::Up),
                _ => {}
            };
        }
    });

    stdweb::web::document().add_event_listener({
        let snake = snake.clone();
        move |event: TouchStart| {
            let touches = event.changed_touches();
            if touches.len() == 0 {
                return;
            }
            let x = touches[0].page_x();
            let y = touches[0].page_y();
            snake.borrow_mut().touch_down(x, y);
        }
    });

    stdweb::web::document().add_event_listener({
        let snake = snake.clone();
        move |event: TouchEnd| {
            let touches = event.changed_touches();
            if touches.len() == 0 {
                return;
            }
            let x = touches[0].page_x();
            let y = touches[0].page_y();
            snake.borrow_mut().touch_up(x, y);
        }
    });

    fn game_loop(snake: Rc<RefCell<Snake>>, canvas: Rc<Canvas>, time: u32) {
        stdweb::web::set_timeout(
            move || {
                game_loop(snake.clone(), canvas.clone(), time);
                snake.borrow_mut().update();
                snake.borrow().draw(&canvas);
            },
            time,
        );
    }

    game_loop(snake, Rc::new(canvas), GAME_LOOP_TIMEOUT);

    stdweb::event_loop();
}
