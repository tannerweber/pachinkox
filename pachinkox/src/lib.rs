use std::vec;

const RANGE: f64 = 1.0;
const RESTITUTION: f64 = 0.7;
const WIDTH: f64 = 600.0;
const HEIGHT: f64 = 600.0;
const LEFT_WALL: f64 = -WIDTH / 2.0;
const RIGHT_WALL: f64 = WIDTH / 2.0;
const GRAVITY: f64 = 1.0;
pub const PIN_RADIUS: f64 = 5.0;

/// Coordinates
/// +--------------------+
/// | (-1,0) (0,0) (1,0) |
/// |        (0,-1)      |
/// |        (0,-2)      |
/// +--------------------+
pub struct World {
    pub entities: vec::Vec<Entity>,
}

impl World {
    #[must_use]
    pub fn new(ball: &Ball) -> Self {
        let mut entities = vec![
            Entity::Ball(ball.clone()),
            Entity::LeftWall(Wall { x: LEFT_WALL }),
            Entity::RightWall(Wall { x: RIGHT_WALL }),
            Entity::Floor(Floor { y: -HEIGHT }),
        ];
        for _ in 0..50 {
            let x = rand::random_range(LEFT_WALL..RIGHT_WALL);
            let y = rand::random_range(-HEIGHT..0.0);
            entities.push(Entity::Pin(Pin {
                origin: Point { x, y },
            }));
        }
        Self { entities }
    }

    /// Progress the world by 1 tick.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::thread::sleep;
    /// use std::time::Duration;
    /// use pachinkox::*;
    ///
    /// let mut world = World::new(&Ball::default());
    /// loop {
    ///     world.next();
    ///     sleep(Duration::from_millis(16));
    ///     break;
    /// }
    /// ```
    pub fn next(&mut self) {
        for (a, b) in self.get_colliding_entities().iter() {
            if a > b {
                let (left, right) = self.entities.split_at_mut(*a);
                Self::apply_collision(&mut left[*b], &mut right[0]);
            } else if a < b {
                let (left, right) = self.entities.split_at_mut(*b);
                Self::apply_collision(&mut left[*a], &mut right[0]);
            }
        }
        for entity in self.entities.iter_mut() {
            match entity {
                Entity::Ball(ball) => {
                    ball.next();
                }
                _ => (),
            }
        }
    }

    fn get_colliding_entities(&self) -> vec::Vec<(usize, usize)> {
        let mut v = vec::Vec::new();

        for (i, entity) in self.entities.iter().enumerate() {
            for j in i + 1..self.entities.len() {
                let other = &self.entities[j];
                if Self::is_collision(entity, other) {
                    v.push((i, j));
                }
            }
        }

        v
    }

    fn apply_collision(a: &mut Entity, b: &mut Entity) {
        use Entity as E;
        match a {
            E::Ball(ball) => match b {
                E::Ball(_) => (),
                E::LeftWall(_) => Self::blw_phy(ball),
                E::RightWall(_) => Self::brw_phy(ball),
                E::Floor(_) => Self::bf_phy(ball),
                E::Pin(_) => Self::bp_phy(ball),
            },
            E::LeftWall(_) => match b {
                E::Ball(ball) => Self::blw_phy(ball),
                _ => (),
            },
            E::RightWall(_) => match b {
                E::Ball(ball) => Self::brw_phy(ball),
                _ => (),
            },
            E::Floor(_) => match b {
                E::Ball(ball) => Self::bf_phy(ball),
                _ => (),
            },
            E::Pin(_) => match b {
                E::Ball(ball) => Self::bp_phy(ball),
                _ => (),
            },
        }
    }

    fn is_collision(a: &Entity, b: &Entity) -> bool {
        use Entity as E;
        match a {
            E::Ball(ball) => match b {
                E::Ball(_) => false,
                E::LeftWall(wall) => Self::blw_col(ball, wall),
                E::RightWall(wall) => Self::brw_col(ball, wall),
                E::Floor(floor) => Self::bf_col(ball, floor),
                E::Pin(pin) => Self::bp_col(ball, pin),
            },
            E::LeftWall(wall) => match b {
                E::Ball(ball) => Self::blw_col(ball, wall),
                _ => false,
            },
            E::RightWall(wall) => match b {
                E::Ball(ball) => Self::brw_col(ball, wall),
                _ => false,
            },
            E::Floor(floor) => match b {
                E::Ball(ball) => Self::bf_col(ball, floor),
                _ => false,
            },
            E::Pin(pin) => match b {
                E::Ball(ball) => Self::bp_col(ball, pin),
                _ => false,
            },
        }
    }

    // Ball - LeftWall
    fn blw_col(b: &Ball, w: &Wall) -> bool {
        if b.velocity.x < 0.0
            && b.origin.x > w.x - RANGE
            && b.origin.x < w.x + RANGE
        {
            return true;
        }
        false
    }

    // Ball - LeftWall
    fn blw_phy(b: &mut Ball) {
        b.velocity.x = -b.velocity.x * RESTITUTION;
    }

    // Ball - RightWall
    fn brw_col(b: &Ball, w: &Wall) -> bool {
        if b.velocity.x > 0.0
            && b.origin.x > w.x - RANGE
            && b.origin.x < w.x + RANGE
        {
            return true;
        }
        false
    }

    // Ball - LeftWall
    fn brw_phy(b: &mut Ball) {
        b.velocity.x = -b.velocity.x * RESTITUTION;
    }

    // Ball - Floor
    fn bf_col(b: &Ball, f: &Floor) -> bool {
        if b.velocity.y < 0.0
            && b.origin.y > f.y - RANGE
            && b.origin.y < f.y + RANGE
        {
            return true;
        }
        false
    }

    // Ball - Floor
    fn bf_phy(b: &mut Ball) {
        b.velocity.y = -b.velocity.y * RESTITUTION;
    }

    // Ball - Pin
    fn bp_col(b: &Ball, p: &Pin) -> bool {
        if b.origin.x > p.origin.x - PIN_RADIUS
            && b.origin.x < p.origin.x + PIN_RADIUS
            && b.origin.y > p.origin.y - PIN_RADIUS
            && b.origin.y < p.origin.y + PIN_RADIUS
        {
            return true;
        }
        false
    }

    // Ball - Pin
    fn bp_phy(b: &mut Ball) {
        b.velocity.x = -b.velocity.x * RESTITUTION;
        b.velocity.y = -b.velocity.y * RESTITUTION;
    }
}

#[derive(Debug)]
pub enum Entity {
    Ball(Ball),
    LeftWall(Wall),
    RightWall(Wall),
    Floor(Floor),
    Pin(Pin),
}

/// A 2D ball that bounces around.
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Ball {
    pub origin: Point,
    pub radius: f64,
    pub velocity: Velocity,
}

impl Ball {
    pub fn next(&mut self) {
        self.velocity.y -= GRAVITY;
        self.origin.x += self.velocity.x;
        self.origin.y += self.velocity.y;
    }
}

/// An infinitely tall vertical wall.
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Wall {
    pub x: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Floor {
    pub y: f64,
}

/// An infinitely small pin.
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Pin {
    pub origin: Point,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Point {
    /// Horizontal
    pub x: f64,
    /// Vertical
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Velocity {
    /// Horizontal
    pub x: f64,
    /// Vertical
    pub y: f64,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_constructor() {
        let ball = Ball::default();
        let _ = World::new(&ball);
    }
}
