use std::{
    fmt::Debug,
    ops,
};
use serde::{Serialize, Deserialize};

pub trait Float : Copy + Clone + Default + Debug +
    PartialEq + PartialOrd +
    Sync + Send +
    Serialize + for<'d> Deserialize<'d> +
    ops::Add<Output=Self> + ops::AddAssign +
    ops::Sub<Output=Self> + ops::SubAssign + 
    ops::Mul<Output=Self> + ops::MulAssign + 
    ops::Div<Output=Self> + ops::DivAssign +
    ops::Neg<Output=Self> {
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const HALF: Self;
    fn of(value: f64) -> Self;
    fn to(self) -> f64;
    fn sqrt(self) -> Self;
    fn powi(self, pow: i32) -> Self;
    fn min(self, other: Self) -> Self;
}

impl Float for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const HALF: Self = 0.5;

    fn of(value: f64) -> Self {
        value
    }

    fn to(self) -> f64 {
        self
    }

    fn sqrt(self) -> Self {
        Self::sqrt(self)
    }

    fn powi(self, pow: i32) -> Self {
        Self::powi(self, pow)
    }

    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }
}

impl Float for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const HALF: Self = 0.5;

    fn of(value: f64) -> Self {
        value as f32
    }

    fn to(self) -> f64 {
        self as f64
    }

    fn sqrt(self) -> Self {
        Self::sqrt(self)
    }
    
    fn powi(self, pow: i32) -> Self {
        Self::powi(self, pow)
    }

    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vector2<F> {
    pub x: F,
    pub y: F,
}

impl<F> Vector2<F> {
    pub const fn new(x: F, y: F) -> Self {
        Self { x, y, }
    }
}

impl<F: Float> Vector2<F> {
    pub fn length(self) -> F {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn length_squared(self) -> F {
        self.x * self.x + self.y * self.y
    }
}

impl<F: Float> ops::Mul<F> for Vector2<F> {
    type Output = Self;

    fn mul(self, other: F) -> Self::Output {
        Self::new(self.x * other, self.y * other)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[derive(Deserialize)]
pub struct Vector3<F> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F> Vector3<F> {
    pub const fn new(x: F, y: F, z: F) -> Self {
        Self { x, y, z }
    }
}

impl<F: Float> Vector3<F> {
    pub const ZERO: Self = Self::new(F::ZERO, F::ZERO, F::ZERO);

    pub fn normalized(self) -> Self {
        let scale = F::ONE / self.length();
        Self::new(self.x * scale, self.y * scale, self.z * scale)
    }

    pub fn length(self) -> F {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(self) -> F {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn distance(self, other: Self) -> F {
        (self - other).length()
    }

    pub fn dot(self, other: Self) -> F {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<F: Float> From<[F; 3]> for Vector3<F> {
    fn from(array: [F; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl<F: Float> ops::Add for Vector3<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<F: Float> ops::Sub for Vector3<F> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<F: Float> ops::Mul<F> for Vector3<F> {
    type Output = Self;

    fn mul(self, other: F) -> Self::Output {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}

impl<F: Float> ops::Neg for Vector3<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Ray<F: Float> {
    pub origin: Vector3<F>,
    pub direction: Vector3<F>,
}

impl<F: Float> Ray<F> {
    pub fn at(&self, t: F) -> Vector3<F> {
        self.origin + self.direction * t
    }
}
