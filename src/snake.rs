use canvas::Canvas;
use direction::Direction;
use stdweb::unstable::TryInto;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Block(u32, u32);

#[derive(Debug)]
pub struct Snake {
    head: Block,
    tail: Vec<Block>,
    food: Block,
    height: u32,
    width: u32,
    direction: Option<Direction>,
    next_direction: Option<Direction>,
    last_direction: Direction,

    touch_start_x: f64,
    touch_start_y: f64,
}

impl Snake {
    pub fn new(width: u32, height: u32) -> Snake {
        let head_x: u32 = js! {return Math.floor(Math.random() * @{width})}
            .try_into()
            .unwrap();

        let head_y: u32 = js! {return Math.floor(Math.random() * @{height})}
            .try_into()
            .unwrap();

        let head = Block(head_x, head_y);

        let food_x: u32 = js! { return Math.floor(Math.random() * @{width}) }
            .try_into()
            .unwrap();
        let food_y: u32 = js! { return Math.floor(Math.random() * @{height}) }
            .try_into()
            .unwrap();

        let food = Block(food_x, food_y);
        let tail = Vec::new();

        Snake {
            head,
            tail,
            food,
            height,
            width,
            direction: None,
            next_direction: None,
            last_direction: Direction::Right,
            touch_start_x: 0_f64,
            touch_start_y: 0_f64,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if !self.last_direction.opposite(direction) && self.direction.is_none() {
            self.direction = Some(direction)
        } else if self.direction.iter().any(|d| !d.opposite(direction)) {
            self.next_direction = Some(direction)
        }
    }

    pub fn touch_down(&mut self, x: f64, y: f64) {
        self.touch_start_x = x;
        self.touch_start_y = y;
    }

    pub fn touch_up(&mut self, x: f64, y: f64) {
        let delta_x = x - self.touch_start_x;
        let delta_y = y - self.touch_start_y;
        if delta_x.abs() < 0.00000005_f64 {
            return;
        }
        let angle = delta_y.atan2(delta_x);
        match angle {
            x if (0.0..90.0).contains(&x) => self.change_direction(Direction::Right),
            x if (90.0..180.0).contains(&x) => self.change_direction(Direction::Up),
            x if (180.0..270.0).contains(&x) => self.change_direction(Direction::Left),
            _ => self.change_direction(Direction::Down),
        }
    }


    pub fn update(&mut self) {
        let direction = self.direction.unwrap_or(self.last_direction);
        self.last_direction = direction;

        let new_head = match direction {
            Direction::Up => Block(
                (self.head.0) % self.width,
                (self.head.1.checked_sub(1).unwrap_or(self.height - 1)) % self.height,
            ),
            Direction::Down => Block((self.head.0) % self.width, (self.head.1 + 1) % self.height),
            Direction::Right => Block((self.head.0 + 1) % self.width, (self.head.1) % self.height),
            Direction::Left => Block(
                (self.head.0.checked_sub(1).unwrap_or(self.width - 1)) % self.width,
                (self.head.1) % self.height,
            ),
        };

        self.tail.insert(0, self.head);
        let last_end = self.tail.pop();

        if self.tail.contains(&new_head) {
            *self = Snake::new(self.width, self.height);
        }

        // js! { console.log( "X:", @{self.head.0}, "Y:", @{self.head.1} ) }
        // uncomment to see x and y coordinates of the snake's head in browser console.
        self.head = new_head;
        if self.head == self.food {
            let mut food = self.food;
            while food == self.head || self.tail.contains(&food) {
                let food_x: u32 = js! { return Math.floor(Math.random() * @{self.width}) }
                    .try_into()
                    .unwrap();

                let food_y: u32 = js! { return Math.floor(Math.random() * @{self.height}) }
                    .try_into()
                    .unwrap();

                food = Block(food_x, food_y);
            }
            self.food = food;
            last_end.map(|x| self.tail.push(x));
        }
        self.direction = self.next_direction.take();
    }

    pub fn draw(&self, canvas: &Canvas) {
        canvas.clear_all();
        canvas.draw(self.head.0, self.head.1, "green");
        for &Block(x, y) in &self.tail {
            canvas.draw(x, y, "lightgreen ");
        }
        canvas.draw(self.food.0, self.food.1, "red");
    }
}
