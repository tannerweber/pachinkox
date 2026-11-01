use std::{thread::sleep, time::Duration};

use pachinkox::{Entity, Point, Velocity};
use raylib::prelude::*;

const END: f64 = -600.0;

fn main() {
    let (mut rl, thread) =
        raylib::init().size(1024, 768).title("Pachinkox").build();

    let ball = pachinkox::Ball {
        velocity: Velocity { x: -5.0, y: 0.0 },
        origin: Point::default(),
        radius: 10.0,
    };
    let mut world = pachinkox::World::new(&ball);
    let mut restart = false;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let screen_width = d.get_screen_width();

        if restart {
            world = pachinkox::World::new(&ball);
            restart = false;
        }

        for entity in world.entities.iter() {
            match entity {
                Entity::Ball(ball) => {
                    if ball.origin.y <= END {
                        restart = true;
                    }
                    d.draw_circle(
                        con_x(ball.origin.x, screen_width) as i32,
                        con_y(ball.origin.y) as i32,
                        ball.radius as f32,
                        Color::RED,
                    );
                }
                Entity::LeftWall(wall) => {
                    d.draw_line(
                        con_x(wall.x, screen_width),
                        -999,
                        con_x(wall.x, screen_width),
                        999,
                        Color::BLACK,
                    );
                }
                Entity::RightWall(wall) => {
                    d.draw_line(
                        con_x(wall.x, screen_width),
                        -999,
                        con_x(wall.x, screen_width),
                        999,
                        Color::BLACK,
                    );
                }
                Entity::Floor(floor) => {
                    d.draw_line(
                        -999,
                        con_y(floor.y),
                        999,
                        con_y(floor.y),
                        Color::BLACK,
                    );
                }
                Entity::Pin(pin) => {
                    d.draw_circle(
                        con_x(pin.origin.x, screen_width) as i32,
                        con_y(pin.origin.y) as i32,
                        pachinkox::PIN_RADIUS as f32,
                        Color::BLACK,
                    );
                }
            }
        }

        d.clear_background(Color {
            r: 235,
            g: 235,
            b: 235,
            a: 1,
        });

        world.next();
        sleep(Duration::from_millis(100));
    }
}

fn con_x(x: f64, width: i32) -> i32 {
    (width as f64 / 2f64 + x) as i32
}

fn con_y(y: f64) -> i32 {
    -y as i32
}
