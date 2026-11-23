use std::vec;

const RANGE: f64 = 1.0;
const RESTITUTION: f64 = 0.7;
const WIDTH: f64 = 600.0;
const HEIGHT: f64 = 600.0;
const LEFT_WALL: f64 = -WIDTH / 2.0;
const RIGHT_WALL: f64 = WIDTH / 2.0;
const GRAVITY: f64 = 1.0;
const DEFAULT_BALL_RADIUS: f64 = 10.0;
pub const PIN_RADIUS: f64 = 5.0;

/// The world containing all entities to be simulated.
///
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
            // Entity::Ball(Ball::new(90.0, 0.0, 10.0)),
            // Entity::Ball(Ball::new(80.0, 0.0, 10.0)),
            // Entity::Ball(Ball::new(50.0, 0.0, 10.0)),
            // Entity::Ball(Ball::new(30.0, 0.0, 10.0)),
            // Entity::Ball(Ball::new(10.0, 0.0, 10.0)),
            // Entity::Ball(Ball::new(-30.0, 0.0, 10.0)),
            // Entity::Ball(Ball::new(-50.0, 0.0, 10.0)),
            // Entity::Ball(Ball::new(-70.0, 0.0, 10.0)),
            // Entity::Ball(Ball::new(-90.0, 0.0, 10.0)),
            Entity::LeftWall(Wall { x: LEFT_WALL }),
            Entity::RightWall(Wall { x: RIGHT_WALL }),
            Entity::Floor(Floor { y: -HEIGHT }),
        ];
        for p in &Self::get_random_pins(200) {
            entities.push(Entity::Pin(p.clone()));
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
        for (a, b) in &self.get_colliding_entities() {
            if a > b {
                let (left, right) = self.entities.split_at_mut(*a);
                Self::apply_collision(&mut left[*b], &mut right[0]);
            } else if a < b {
                let (left, right) = self.entities.split_at_mut(*b);
                Self::apply_collision(&mut left[*a], &mut right[0]);
            }
        }
        for entity in &mut self.entities {
            match entity {
                Entity::Ball(ball) => {
                    ball.next();
                }
                _ => (),
            }
        }
    }

    #[must_use]
    pub fn get_pin_diamond() -> vec::Vec<Pin> {
        let mut pins = vec::Vec::new();
        let levels: u32 = 11;
        let max_width: f64 = WIDTH;
        let y_gap: f64 = -50.0;

        for i in 1..=levels {
            for j in 1..=i {
                let distance: f64 = max_width / (f64::from(i) + 1.0);
                let y: f64 = f64::from(i) * y_gap;
                let x: f64 = (f64::from(j) * distance) - max_width / 2.0;
                pins.push(Pin {
                    origin: Point { x, y },
                });
            }
        }

        pins
    }

    #[must_use]
    pub fn get_random_pins(n: u32) -> vec::Vec<Pin> {
        let mut pins = vec::Vec::new();

        for _ in 0..n {
            let x = rand::random_range(LEFT_WALL..RIGHT_WALL);
            let y = rand::random_range(-HEIGHT..0.0);
            pins.push(Pin {
                origin: Point { x, y },
            });
        }

        pins
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
                E::Pin(pin) => Self::bp_phy(ball, pin),
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
            E::Pin(pin) => match b {
                E::Ball(ball) => Self::bp_phy(ball, pin),
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
        if b.origin.distance(&p.origin) <= PIN_RADIUS + b.radius {
            let x_diff = b.origin.x - p.origin.x;
            let y_diff = b.origin.y - p.origin.y;
            if x_diff.signum() != b.velocity.x.signum()
                && y_diff.signum() != b.velocity.y.signum()
            {
                return true;
            }
            return false;
        }
        false
    }

    // Ball - Pin
    fn bp_phy(b: &mut Ball, p: &Pin) {
        let distance = b.origin.distance(&p.origin);
        let t = b.radius / distance;
        let collision_point_x = b.origin.x + t * (p.origin.x - b.origin.x);
        let collision_point_y = b.origin.y + t * (p.origin.y - b.origin.y);
        let col_point = Point {
            x: collision_point_x,
            y: collision_point_y,
        };
        let tangent_line: Point = col_point
            + (t * Point {
                x: -(p.origin.y - b.origin.y),
                y: p.origin.x - b.origin.x,
            });

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
    #[must_use]
    pub const fn new(x: f64, y: f64, velocity_x: f64) -> Self {
        Self {
            origin: Point { x, y },
            radius: DEFAULT_BALL_RADIUS,
            velocity: Velocity {
                x: velocity_x,
                y: 0.0,
            },
        }
    }

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

impl Point {
    #[must_use]
    pub fn distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        f64::sqrt(dx.powi(2) + dy.powi(2))
    }
}

impl std::ops::Mul<Self> for Point {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
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

    #[test]
    fn test_point_distance() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 0.0, y: 0.0 };
        assert_eq!(0.0, p1.distance(&p2));

        let p1 = Point { x: 0.0, y: 1.0 };
        let p2 = Point { x: 0.0, y: 0.0 };
        assert_eq!(1.0, p1.distance(&p2));

        let p1 = Point { x: 1.0, y: 0.0 };
        let p2 = Point { x: 0.0, y: 0.0 };
        assert_eq!(1.0, p1.distance(&p2));

        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 0.0, y: 1.0 };
        assert_eq!(1.0, p1.distance(&p2));

        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 1.0, y: 0.0 };
        assert_eq!(1.0, p1.distance(&p2));
    }

    #[test]
    fn test_point_mul() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 0.0, y: 0.0 };
        assert_eq!(0.0, p1 * p2);

        let p1 = Point { x: 1.0, y: 1.0 };
        let p2 = Point { x: 0.0, y: 0.0 };
        assert_eq!(0.0, p1 * p2);

        let p1 = Point { x: 1.0, y: 1.0 };
        let p2 = Point { x: 1.0, y: 1.0 };
        assert_eq!(2.0, p1 * p2);

        let p1 = Point { x: 100.0, y: 1.0 };
        let p2 = Point { x: 2.0, y: 99.0 };
        assert_eq!(299.0, p1 * p2);
    }
}
