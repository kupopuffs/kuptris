#[macro_use]
extern crate stdweb;

mod canvas;
mod direction;
mod snake;

use canvas::Canvas;
use direction::Direction;
use snake::Snake;

use stdweb::traits::*;
use stdweb::web::{event::{KeyDownEvent, TouchEnter, TouchEnd}, IEventTarget};

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    stdweb::initialize();

    println!("hello there!");

    let canvas = Canvas::new("#canvas", 20, 20);
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
        move |event: TouchEnter| {
            let x = event.touches()[0].page_x();
            let y = event.touches()[0].page_y();
            snake.borrow_mut().touch_down(x, y);
        }
    });

    stdweb::web::document().add_event_listener({
        let snake = snake.clone();
        move |event: TouchEnd| {
            let x = event.touches()[0].page_x();
            let y = event.touches()[0].page_y();
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

    game_loop(snake, Rc::new(canvas), 50);

    stdweb::event_loop();
}
