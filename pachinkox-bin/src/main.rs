use std::{thread::sleep, time::Duration};

use pachinkox::{Entity, Point, Velocity, World};
use raylib::prelude::*;

const WALL_HEIGHT: i32 = 999;
const FRAMETIME: u64 = 16;
const END: f64 = -600.0;
const KEY_QUIT: char = 'q';
const KEY_RESTART: char = 'r';
const KEY_DROP_BALL: char = ' ';
const FONT_SIZE: i32 = 30;
const BALL_START_MIN: f64 = -100.0;
const BALL_START_MAX: f64 = 100.0;
const BALL_START_POS_INCREMENT: f64 = 4.0;
const CIRCLE_COLOR: Color = Color::RED;
const BORDER_COLOR: Color = Color::BLACK;
const PIN_COLOR: Color = Color::BLACK;
const PATH_COLOR: Color = Color::BLUE;

fn main() {
    let (mut rl, thread) =
        raylib::init().size(1024, 768).title("Pachinkox").build();

    let world_builder = World::builder().pin_density(10.0);
    let mut world = world_builder.clone().build();
    let mut restart = false;
    let mut quit = false;
    let mut ball_dropped = false;
    let mut ball_starting_position: f64 = 0.0;
    let mut ball_direction = Direction::Right;
    let mut ball_path = Vec::<(i32, i32)>::new();
    let mut tick: u128 = 0;

    while !rl.window_should_close() {
        match rl.get_char_pressed() {
            Some(c) => match c {
                KEY_RESTART => restart = true,
                KEY_QUIT => quit = true,
                KEY_DROP_BALL => ball_dropped = true,
                _ => (),
            },
            None => (),
        }
        let mut d = rl.begin_drawing(&thread);
        let screen_width = d.get_screen_width();

        if restart {
            world = world_builder.clone().build();
            ball_path = Vec::new();
            restart = false;
            ball_dropped = false;
        }
        if quit {
            break;
        }

        for entity in &mut world.entities {
            match entity {
                Entity::Ball(ball) => {
                    if !ball_dropped {
                        if (ball_direction == Direction::Left
                            && ball_starting_position <= BALL_START_MIN)
                            || (ball_direction == Direction::Right
                                && ball_starting_position >= BALL_START_MAX)
                        {
                            ball_direction.switch();
                        }
                        match ball_direction {
                            Direction::Left => {
                                ball_starting_position -=
                                    BALL_START_POS_INCREMENT;
                                ball.velocity = Velocity { x: -5.0, y: 0.0 };
                            }
                            Direction::Right => {
                                ball_starting_position +=
                                    BALL_START_POS_INCREMENT;
                                ball.velocity = Velocity { x: 5.0, y: 0.0 };
                            }
                        }
                        ball.origin = Point {
                            x: ball_starting_position,
                            y: -10.0,
                        };
                    }
                    if ball.origin.y <= END {
                        restart = true;
                    }
                    d.draw_circle(
                        con_x(ball.origin.x, screen_width) as i32,
                        con_y(ball.origin.y) as i32,
                        ball.radius as f32,
                        CIRCLE_COLOR,
                    );
                    if ball_dropped {
                        if tick.is_multiple_of(3) {
                            ball_path.push((
                                con_x(ball.origin.x, screen_width),
                                con_y(ball.origin.y),
                            ));
                        }
                    }
                }
                Entity::LeftWall(wall) => {
                    d.draw_line(
                        con_x(wall.x, screen_width),
                        -WALL_HEIGHT,
                        con_x(wall.x, screen_width),
                        WALL_HEIGHT,
                        BORDER_COLOR,
                    );
                }
                Entity::RightWall(wall) => {
                    d.draw_line(
                        con_x(wall.x, screen_width),
                        -WALL_HEIGHT,
                        con_x(wall.x, screen_width),
                        WALL_HEIGHT,
                        BORDER_COLOR,
                    );
                }
                Entity::Floor(floor) => {
                    d.draw_line(
                        -WALL_HEIGHT,
                        con_y(floor.y),
                        WALL_HEIGHT,
                        con_y(floor.y),
                        BORDER_COLOR,
                    );
                }
                Entity::Pin(pin) => {
                    d.draw_circle(
                        con_x(pin.origin.x, screen_width) as i32,
                        con_y(pin.origin.y) as i32,
                        pachinkox::PIN_RADIUS as f32,
                        PIN_COLOR,
                    );
                }
            }
        }

        if ball_dropped {
            for p in &ball_path {
                d.draw_circle(p.0, p.1, 2.0, PATH_COLOR);
            }
        }

        d.draw_text("<space> to drop ball", 10, 10, FONT_SIZE, Color::BLACK);
        d.draw_text("q to quit", 10, 40, FONT_SIZE, Color::BLACK);
        d.draw_text("r to restart", 10, 70, FONT_SIZE, Color::BLACK);

        d.clear_background(Color {
            r: 235,
            g: 235,
            b: 235,
            a: 1,
        });

        if ball_dropped {
            world.next();
        }
        sleep(Duration::from_millis(FRAMETIME));
        tick += 1;
    }
}

fn con_x(x: f64, width: i32) -> i32 {
    (width as f64 / 2f64 + x) as i32
}

fn con_y(y: f64) -> i32 {
    -y as i32
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn switch(&mut self) {
        if *self == Self::Left {
            *self = Self::Right;
        } else if *self == Self::Right {
            *self = Self::Left;
        }
    }
}
