use std::cmp::Ordering;
use std::ops::{Mul, Sub, Add};

const LOW_BITS: u64 = ((1 as u64) << 32) - 1;
const HIGH_BITS: u64 = LOW_BITS << 32;
fn big_prod(first: u64, second: u64) -> (u64, u64) {
    let flow:   u64 = first & LOW_BITS;
    let slow:   u64 = second & LOW_BITS;
    let fhigh:  u64 = (first & HIGH_BITS) >> 32;
    let shigh:  u64 = (second & HIGH_BITS) >> 32;
    let reslow: u64 = flow * slow;
    let resm1:  u64 = fhigh * slow;
    let resm2:  u64 = flow * shigh;
    let secdig: u64 = (resm1 & LOW_BITS) + (resm2 & LOW_BITS) + ((reslow & HIGH_BITS) >> 32);
    (((secdig & HIGH_BITS) >> 32) + ((resm1 & HIGH_BITS) >> 32) + ((resm2 & HIGH_BITS) >> 32) + fhigh * shigh,
     ((secdig & LOW_BITS) << 32) | (reslow & LOW_BITS))
}

/// Radius-vector from the origin
#[derive(Debug, Clone)]
struct Point<T> {
    x: T,
    y: T,
    norm_sq: T,
}

impl<T: Copy + Mul<Output=T> + Add<Output=T>> Point<T> {
    #[inline]
    fn new(x: T, y: T) -> Point<T> {
        Point{x: x, y: y, norm_sq: x * x + y * y}
    }
}

impl<T: PartialEq + PartialOrd + Default> Point<T> {
    #[inline]
    fn bottom(&self) -> bool{
        self.y < T::default() || (self.y == T::default() && self.x < T::default())
    }
}

impl<'a, T: 'a + Copy + Mul<Output=T> + Add<Output=T>> Mul for &'a Point<T> {
    type Output = T;
    #[inline]
    fn mul(self, rhs: &Point<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: Copy + PartialEq + PartialOrd + Default + Mul<Output=T> + Sub<Output=T>> PartialEq for Point<T> {
    #[inline]
    fn eq(&self, other: &Point<T>) -> bool {
        (self.bottom(), self.y * other.x - self.x * other.y) == (other.bottom(), T::default())
    }
}

impl<T> Eq for Point<T> where Point<T>: PartialEq {}

impl<T: Copy + PartialEq + PartialOrd + Default + Mul<Output=T> + Sub<Output=T>> Ord for Point<T> {
    #[inline]
    fn cmp(&self, other: &Point<T>) -> Ordering {
        (self.bottom(), self.y * other.x - self.x * other.y)
            .partial_cmp(&(other.bottom(), T::default()))
            .unwrap_or(Ordering::Equal)
    }
}

impl<T> PartialOrd for Point<T> where Point<T>: Ord, {
    #[inline]
    fn partial_cmp(&self, other: &Point<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Angle<'a, T: 'a> {
    v: &'a Point<T>,
    w: &'a Point<T>,
    dot_product: T,
    norm_prod_sq: T,
}

impl<'a, T: Copy + Mul<Output=T>> Angle<'a, T> where &'a Point<T>: Mul<Output=T> {
    #[inline]
    fn new(v: &'a Point<T>, w: &'a Point<T>) -> Angle<'a, T> {
        Angle{v: v, w: w, dot_product: v * w, norm_prod_sq: v.norm_sq * w.norm_sq}
    }
}

/*impl<T: Ord + Default + Mul<Output=T>> PartialEq for Angle<T> {
    #[inline]
    fn eq(&self, other: &Angle<T>) -> bool {
        self.dot_product.cmp(&T::default()) == other.dot_product.cmp(&T::default()) &&
            other.dot_product * other.dot_product * self.norm_prod_sq ==
            self.dot_product * self.dot_product * other.norm_prod_sq
    }
}*/

impl<'a> PartialEq for Angle<'a, i64> {
    #[inline]
    fn eq(&self, other: &Angle<'a, i64>) -> bool {
        self.dot_product.signum() == other.dot_product.signum() &&
            big_prod((other.dot_product * other.dot_product) as u64, self.norm_prod_sq as u64) ==
            big_prod((self.dot_product * self.dot_product) as u64, other.norm_prod_sq as u64)
    }
}

impl<'a, T> Eq for Angle<'a, T> where Angle<'a, T>: PartialEq {}

/*impl<T: Ord + Default + Mul<Output=T>> Ord for Angle<T> {
    fn cmp(&self, other: &Angle<T>) -> Ordering {
        if self.dot_product < T::default() && other.dot_product >= T::default() {
            return Ordering::Greater;
        }
        if self.dot_product > T::default() && other.dot_product <= T::default() {
            return Ordering::Less;
        }
        (other.dot_product * other.dot_product * self.norm_prod_sq)
            .cmp(&(self.dot_product * self.dot_product * other.norm_prod_sq))
    }
}*/

impl<'a> Ord for Angle<'a, i64> {
    fn cmp(&self, other: &Angle<'a, i64>) -> Ordering {
        let sign_ord = other.dot_product.signum().cmp(&self.dot_product.signum());
        if sign_ord != Ordering::Equal {
            return sign_ord;
        }
        if self.dot_product.signum() == -1 {
            return big_prod(other.norm_prod_sq as u64, (self.dot_product * self.dot_product) as u64)
            .cmp(&big_prod(self.norm_prod_sq as u64, (other.dot_product * other.dot_product) as u64));
        }
        big_prod(self.norm_prod_sq as u64, (other.dot_product * other.dot_product) as u64)
            .cmp(&big_prod(other.norm_prod_sq as u64, (self.dot_product * self.dot_product) as u64))
    }
}

impl<'a, T> PartialOrd for Angle<'a, T> where Angle<'a, T>: Ord {
    #[inline]
    fn partial_cmp(&self, other: &Angle<'a, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}