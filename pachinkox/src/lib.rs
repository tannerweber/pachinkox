//! This library crate can perform a basic simulation like a pachink machine.
//! A ball can bounce of pins as it falls down.
//!
//! # Quick Start
//!
//! ```
//! use std::thread::sleep;
//! use std::time::Duration;
//! use pachinkox::*;
//!
//! let mut world = World::builder().pin_density(10.0).build();
//! loop {
//!
//!     for entity in &mut world.entities {
//!         match entity {
//!             Entity::Ball(ball) => {
//!                 // Draw the ball
//!             }
//!             Entity::Pin(pin) => {
//!                 // Draw the pin
//!             }
//!             _ => (),
//!         }
//!     }
//!
//!     world.next();
//!     sleep(Duration::from_millis(16));
//!     break; // Only for example
//! }
//! ```

const RANGE: f64 = 1.0;
const RESTITUTION: f64 = 0.7;
const WIDTH: f64 = 600.0;
const HEIGHT: f64 = 600.0;
const LEFT_WALL: f64 = -WIDTH / 2.0;
const RIGHT_WALL: f64 = WIDTH / 2.0;
const GRAVITY_PER_TICK: f64 = -0.25;
const DEFAULT_BALL_RADIUS: f64 = 10.0;
const DEFAULT_PIN_DENSITY: f64 = 10.0;
const MARGIN_FROM_TOP: f64 = 50.0;
pub const PIN_RADIUS: f64 = 5.0;

/// The world containing all entities to be simulated.
///
/// Coordinates
///
/// ```md
/// +--------------------+
/// | (-1,0) (0,0) (1,0) |
/// |        (0,-1)      |
/// |        (0,-2)      |
/// +--------------------+
/// ```
#[derive(Debug, Default)]
pub struct World {
    pub entities: Vec<Entity>,
}

impl World {
    /// Returns a world with no entities. Use the `builder()` method to
    /// construct a `World` with a variety of default entities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder for the `World` that provides default entities and the ability
    /// to change them.
    ///
    /// # Examples
    ///
    /// ```
    /// use pachinkox::*;
    ///
    /// let world = World::builder().pin_density(10.0).build();
    /// ```
    #[must_use]
    pub fn builder() -> WorldBuilder {
        WorldBuilder::default()
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
    /// let mut world = World::builder().pin_density(10.0).build();
    /// loop {
    ///     world.next();
    ///     sleep(Duration::from_millis(16));
    ///     break; // Only for example
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
            #[allow(clippy::single_match)]
            match entity {
                Entity::Ball(ball) => {
                    ball.next();
                }
                _ => (),
            }
        }
    }

    fn get_colliding_entities(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::new();

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
        #[allow(clippy::single_match)]
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
    fn bp_phy(b: &mut Ball) {
        b.velocity.x = -b.velocity.x * RESTITUTION;
        b.velocity.y = -b.velocity.y * RESTITUTION;
    }
}

/// Construct a `World` with some default entities.
#[derive(Debug, Clone)]
pub struct WorldBuilder {
    pin_density: f64,
    ball: Ball,
    left_wall: Wall,
    right_wall: Wall,
    floor: Floor,
}

impl WorldBuilder {
    #[must_use]
    pub fn build(self) -> World {
        let mut entities = Vec::<Entity>::new();

        for p in &Self::get_random_pins(
            &self.floor,
            &self.left_wall,
            &self.right_wall,
            self.pin_density,
        ) {
            entities.push(Entity::Pin(p.clone()));
        }

        entities.push(Entity::Ball(self.ball));
        entities.push(Entity::LeftWall(self.left_wall));
        entities.push(Entity::RightWall(self.right_wall));
        entities.push(Entity::Floor(self.floor));

        World { entities }
    }

    /// Set the density of pins. A higher value leads to more pins.
    #[must_use]
    pub const fn pin_density(mut self, density: f64) -> Self {
        self.pin_density = density;
        self
    }

    /// Set the floor position.
    #[must_use]
    pub const fn floor(mut self, y: f64) -> Self {
        self.floor = Floor { y };
        self
    }

    #[must_use]
    /// Set the left wall position.
    pub const fn left_wall(mut self, x: f64) -> Self {
        self.left_wall = Wall { x };
        self
    }

    #[must_use]
    /// Set the right wall position.
    pub const fn right_wall(mut self, x: f64) -> Self {
        self.right_wall = Wall { x };
        self
    }

    #[must_use]
    fn get_random_pins(
        floor: &Floor,
        left_wall: &Wall,
        right_wall: &Wall,
        mut density: f64,
    ) -> Vec<Pin> {
        if density < 1.0 {
            density = 1.0;
        }
        let mut pins = Vec::<Pin>::new();
        let mut rows = Vec::<((f64, f64), Vec<(f64, f64)>)>::new();
        let width = (left_wall.x - right_wall.x).abs();
        let height = floor.y.abs();
        let cell_width = width / density;
        let cell_height = height / density;

        for n in 0..((height / cell_height).round() as usize) {
            let min_y = -(n as f64) * cell_height;
            let max_y = (-(n as f64) + 1.0) * cell_height;
            rows.push(((min_y, max_y), vec![]));
        }

        for r in &mut rows {
            for n in 0..((width / cell_width).round() as usize) {
                let min_x = (n as f64 * cell_width) - width / 2.0;
                let max_x = ((n as f64 + 1.0) * cell_width) - width / 2.0;
                r.1.push((min_x, max_x));
            }
        }

        for r in &rows {
            for c in &r.1 {
                let x = rand::random_range(c.0..c.1);
                let y = rand::random_range(r.0.0..r.0.1);
                if y > -MARGIN_FROM_TOP {
                    continue;
                }
                pins.push(Pin {
                    origin: Point { x, y },
                });
            }
        }

        pins
    }
}

impl std::default::Default for WorldBuilder {
    fn default() -> Self {
        Self {
            pin_density: DEFAULT_PIN_DENSITY,
            ball: Ball::default(),
            left_wall: Wall { x: LEFT_WALL },
            right_wall: Wall { x: RIGHT_WALL },
            floor: Floor { y: -HEIGHT },
        }
    }
}

/// An entity that the `World` can contain.
#[derive(Debug)]
pub enum Entity {
    Ball(Ball),
    LeftWall(Wall),
    RightWall(Wall),
    Floor(Floor),
    Pin(Pin),
}

/// A 2D ball that bounces around.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
        self.velocity.y += GRAVITY_PER_TICK;
        self.origin.x += self.velocity.x;
        self.origin.y += self.velocity.y;
    }
}

impl std::default::Default for Ball {
    fn default() -> Self {
        Self {
            origin: Point::default(),
            radius: DEFAULT_BALL_RADIUS,
            velocity: Velocity::default(),
        }
    }
}

/// An infinitely tall vertical wall.
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Wall {
    pub x: f64,
}

/// The floor of the simlulation.
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Floor {
    pub y: f64,
}

/// A fixed position circular pin.
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Pin {
    pub origin: Point,
}

/// A point in 2D coordinates space.
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

/// A Velocity in 2D coordinates.
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
        let _ = World::new();
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
