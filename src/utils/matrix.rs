use {
    crate::utils::Vector3,
    std::{
        fmt,
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    },
};

#[derive(Clone, Copy)]
pub struct Matrix {
    a: Vector3,
    b: Vector3,
    c: Vector3,
}

impl Matrix {
    pub const fn new(a: Vector3, b: Vector3, c: Vector3) -> Self {
        Self { a, b, c }
    }

    pub fn a(&self) -> &Vector3 {
        &self.a
    }

    pub fn b(&self) -> &Vector3 {
        &self.b
    }

    pub fn c(&self) -> &Vector3 {
        &self.c
    }

    pub fn transpose(&self) -> Self {
        Self {
            a: Vector3::new(self.a.x(), self.b.x(), self.c.x()),
            b: Vector3::new(self.a.y(), self.b.y(), self.c.y()),
            c: Vector3::new(self.a.z(), self.b.z(), self.c.z()),
        }
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            a: self.a + other.a,
            b: self.b + other.b,
            c: self.c + other.c,
        }
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            a: self.a + other.a,
            b: self.b + other.b,
            c: self.c + other.c,
        };
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            a: self.a - other.a,
            b: self.b - other.b,
            c: self.c - other.c,
        }
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            a: self.a - other.a,
            b: self.b - other.b,
            c: self.c - other.c,
        };
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let a = Vector3::new(
            self.a.x() * other.a.x() + self.a.y() * other.b.x() + self.a.z() * other.c.x(),
            self.a.x() * other.a.y() + self.a.y() * other.b.y() + self.a.z() * other.c.y(),
            self.a.x() * other.a.z() + self.a.y() * other.b.z() + self.a.z() * other.c.z(),
        );
        let b = Vector3::new(
            self.b.x() * other.a.x() + self.b.y() * other.b.x() + self.b.z() * other.c.x(),
            self.b.x() * other.a.y() + self.b.y() * other.b.y() + self.b.z() * other.c.y(),
            self.b.x() * other.a.z() + self.b.y() * other.b.z() + self.b.z() * other.c.z(),
        );
        let c = Vector3::new(
            self.c.x() * other.a.x() + self.c.y() * other.b.x() + self.c.z() * other.c.x(),
            self.c.x() * other.a.y() + self.c.y() * other.b.y() + self.c.z() * other.c.y(),
            self.c.x() * other.a.z() + self.c.y() * other.b.z() + self.c.z() * other.c.z(),
        );
        Self { a, b, c }
    }
}

impl Mul<f64> for Matrix {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            a: self.a * scalar,
            b: self.b * scalar,
            c: self.c * scalar,
        }
    }
}

impl Mul<Vector3> for Matrix {
    type Output = Vector3;

    fn mul(self, vector: Vector3) -> Vector3 {
        Vector3::new(
            self.a.x() * vector.x() + self.a.y() * vector.y() + self.a.z() * vector.z(),
            self.b.x() * vector.x() + self.b.y() * vector.y() + self.b.z() * vector.z(),
            self.c.x() * vector.x() + self.c.y() * vector.y() + self.c.z() * vector.z(),
        )
    }
}

impl MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, scalar: f64) {
        *self = Self {
            a: self.a * scalar,
            b: self.b * scalar,
            c: self.c * scalar,
        };
    }
}

impl Div<f64> for Matrix {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            a: self.a / scalar,
            b: self.b / scalar,
            c: self.c / scalar,
        }
    }
}

impl DivAssign<f64> for Matrix {
    fn div_assign(&mut self, scalar: f64) {
        *self = Self {
            a: self.a / scalar,
            b: self.b / scalar,
            c: self.c / scalar,
        };
    }
}

impl Neg for Matrix {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            a: -self.a,
            b: -self.b,
            c: -self.c,
        }
    }
}

pub fn calculate_rotation_matrix(rotation: Vector3) -> Matrix {
    let x = rotation.x();
    let y = rotation.y();
    let z = rotation.z();
    let x_rotation_matrix = Matrix::new(
        Vector3::new(1., 0., 0.),
        Vector3::new(0., x.cos(), -x.sin()),
        Vector3::new(0., x.sin(), x.cos()),
    );
    let y_rotation_matrix = Matrix::new(
        Vector3::new(y.cos(), 0., y.sin()),
        Vector3::new(0., 1., 0.),
        Vector3::new(-y.sin(), 0., y.cos()),
    );
    let z_rotation_matrix = Matrix::new(
        Vector3::new(z.cos(), -z.sin(), 0.),
        Vector3::new(z.sin(), z.cos(), 0.),
        Vector3::new(0., 0., 1.),
    );
    z_rotation_matrix * y_rotation_matrix * x_rotation_matrix
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix {{\n  a: {},\n  b: {},\n  c: {}\n}}",
            self.a, self.b, self.c
        )
    }
}
