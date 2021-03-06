#![deny(missing_docs)]

//! # Higher Order Core
//!
//! This crate contains core structs and traits for programming with higher order data structures.
//!
//! ### Introduction to higher order data structures
//!
//! A higher order data structure is a generalization of an ordinary data structure.
//!
//! In ordinary data structures, the default way of programming is:
//!
//! - Use data structures for data
//! - Use methods/functions for operations on data
//!
//! In a higher order data structure, data and functions become the same thing.
//!
//! The central idea of a higher order data structure,
//! is that properties can be functions of the same type.
//!
//! For example, a `Point` has an `x`, `y` and `z` property.
//! In ordinary programming, `x`, `y` and `z` might have the type `f64`.
//!
//! If `x`, `y` and `z` are functions from `T -> f64`,
//! then the point type is `Point<T>`.
//!
//! A higher order `Point<T>` can be called, just like a function.
//! When called as a function, `Point<T>` returns `Point`.
//!
//! However, unlike functions, you can still access properties of `Point<T>`.
//! You can also define methods and overload operators for `Point<T>`.
//! This means that in a higher order data structure, data and functions become the same thing.
//!
//! ### Motivation of programming with higher order data structures
//!
//! The major application of higher order data structures is geometry.
//!
//! A typical usage is e.g. to create procedurally generated content for games.
//!
//! Higher order data structures is about finding the right balance between
//! hiding implementation details and exposing them for various generic algorithms.
//!
//! For example, a circle can be thought of as having the type `Point<f64>`.
//! The argument can be an angle in radians, or a value in the unit interval `[0, 1]`.
//!
//! Another example, a line can be thought of as having the type `Point<f64>`.
//! The argument is a value in the unit interval `[0, 1]`.
//! When called with `0`, you get the start point of the line.
//! When called with `1`, you get the end point of the line.
//!
//! Instead of declaring a `Circle` type, a `Line` type and so on,
//! one can use `Point<f64>` to represent both of them.
//!
//! Higher order data structures makes easier to write generic algorithms for geometry.
//! Although it seems abstract at first, it is also practically useful in unexpected cases.
//!
//! For example, an animated point can be thought of as having the type `Point<(&[Frame], f64)>`.
//! The first argument contains the animation data and the second argument is time in seconds.
//! Properties `x`, `y` and `z` of an animated point determines how the animated point is computed.
//! The details of the implementation can be hidden from the algorithm that uses animated points.
//!
//! Sometimes you need to work with complex geometry.
//! In these cases, it is easier to work with higher order data structures.
//!
//! For example, a planet might have a center, equator, poles, surface etc.
//! A planet orbits around a star, which orbits around the center of a galaxy.
//! This means that the properties of a planet, viewed from different reference frames,
//! are functions of the arguments that determine the reference frame.
//! You can create a "higher order planet" to reason about a planet's properties
//! under various reference frames.
//!
//! ### Design
//!
//! Here is an example of how to declare a new higher order data structure:
//!
//! ```rust
//! use ha::{Ho, Call, Arg, Fun, Func};
//! use std::sync::Arc;
//!
//! /// Higher order 3D point.
//! #[derive(Clone)]
//! pub struct Point<T = ()> where f64: Ho<T> {
//!     /// Function for x-coordinates.
//!     pub x: Fun<T, f64>,
//!     /// Function for y-coordinates.
//!     pub y: Fun<T, f64>,
//!     /// Function for z-coordinates.
//!     pub z: Fun<T, f64>,
//! }
//!
//! // It is common to declare a type alias for functions, e.g:
//! pub type PointFunc<T> = Point<Arg<T>>;
//!
//! // Implement `Ho<Arg<T>>` to allow higher order data structures
//! // using properties `Fun<T, Point>` (`<Point as Ho<T>>::Fun`).
//! impl<T: Clone> Ho<Arg<T>> for Point {
//!    type Fun = PointFunc<T>;
//! }
//!
//! // Implement `Call<T>` to allow higher order calls.
//! impl<T: Copy> Call<T> for Point
//!     where f64: Ho<Arg<T>> + Call<T>
//! {
//!     fn call(f: &Self::Fun, val: T) -> Point {
//!         Point::<()> {
//!             x: <f64 as Call<T>>::call(&f.x, val),
//!             y: <f64 as Call<T>>::call(&f.y, val),
//!             z: <f64 as Call<T>>::call(&f.z, val),
//!         }
//!     }
//! }
//!
//! impl<T> PointFunc<T> {
//!     /// Helper method for calling value.
//!    pub fn call(&self, val: T) -> Point where T: Copy {
//!        <Point as Call<T>>::call(self, val)
//!    }
//! }
//!
//! // Operations are usually defined as simple traits.
//! // They look exactly the same as for normal generic programming.
//! /// Dot operator.
//! pub trait Dot<Rhs = Self> {
//!     /// The output type.
//!     type Output;
//!
//!     /// Returns the dot product.
//!     fn dot(self, other: Rhs) -> Self::Output;
//! }
//!
//! // Implement operator once for the ordinary case.
//! impl Dot for Point {
//!     type Output = f64;
//!     fn dot(self, other: Self) -> f64 {
//!         self.x * other.x +
//!         self.y * other.y +
//!         self.z * other.z
//!     }
//! }
//!
//! // Implement operator once for the higher order case.
//! impl<T: 'static + Copy> Dot for PointFunc<T> {
//!     type Output = Func<T, f64>;
//!     fn dot(self, other: Self) -> Func<T, f64> {
//!         let ax = self.x;
//!         let ay = self.y;
//!         let az = self.z;
//!         let bx = other.x;
//!         let by = other.y;
//!         let bz = other.z;
//!         Arc::new(move |a| ax(a) * bx(a) + ay(a) * by(a) + az(a) * bz(a))
//!     }
//! }
//! ```
//!
//! To disambiguate impls of e.g. `Point<()>` from `Point<T>`,
//! an argument type `Arg<T>` is used for point functions: `Point<Arg<T>>`.
//!
//! For every higher order type `U` and and argument type `T`,
//! there is an associated function type `T -> U`.
//!
//! For primitive types, e.g. `f64`, the function type is `Func<T, f64>`.
//!
//! For higher order structs, e.g. `X<()>`, the function type is `X<Arg<T>>`.
//!
//! The code for operators on higher order data structures must be written twice:
//!
//! - Once for the ordinary case `X<()>`
//! - Once for the higher order case `X<Arg<T>>`
//!
//! ### Higher Order Maps
//!
//! Sometimes it is useful to construct arbitrary data of the kind:
//!
//! - Vectors of primitives
//! - Vectors of vectors, etc.
//!
//! For example, if a higher order point maps from angles to a circle,
//! then complex geometry primitives might be defined onto the circle using angles:
//!
//! - Edge, e.g. `[a, b]`
//! - Triangle, e.g. `[a, b, c]`
//! - Square, e.g. `[[a, b], [c, d]]`
//!
//! The `HMap::hmap` method can be used to work with such structures.
//!
//! For example, if `p` is a higher order point of type `Point<Arg<f64>>`,
//! then the following code maps two points at the same time:
//!
//! ```ignore
//! let q: [Point; 2] = [0.0, 1.0].hmap(&p);
//! ```
//!
//! For binary higher order maps of type `f : (T, T) -> U`,
//! the `HPair::hpair` method can be used before using `HMap::hmap`.
//!
//! For example:
//!
//! ```ignore
//! let in_between: Func<f64, f64> = Arc::new(move |(a, b)| {
//!     if b < a {b += 1.0};
//!     (a + (b - a) * 0.5) % 1.0
//! });
//! // Pair up.
//! let args: [(f64, f64); 2] = ([0.7, 0.9], [0.9, 0.1]).hpair();
//! // `[0.8, 0.0]`
//! let q: [f64; 2] = args.hmap(&in_between);
//! ```

use std::sync::Arc;

/// Standard function type.
pub type Func<T, U> = Arc<dyn Fn(T) -> U + Send + Sync>;

/// Used to disambiguate impls for Rust's type checker.
#[derive(Copy, Clone)]
pub struct Arg<T>(pub T);

/// Implemented by higher order types.
///
/// A higher order type might be a concrete value,
/// or it might be a function of some input type `T`.
///
/// Each higher order type has an associated function type
/// for any argument of type `T`.
///
/// This makes it possible to e.g. associate `PointFunc<T>` with `Point`.
pub trait Ho<T>: Sized {
    /// The function type.
    type Fun: Clone;
}

/// Implemented by higher order calls.
pub trait Call<T>: Ho<Arg<T>> {
    /// Calls function with some value.
    fn call(f: &Self::Fun, val: T) -> Self;
}

impl<T, U> Call<T> for U
where U: Ho<Arg<T>, Fun = Func<T, Self>> {
    fn call(f: &Self::Fun, val: T) -> Self {f(val)}
}

impl<T: Clone> Ho<()> for T {type Fun = T;}

/// Used to declare functions in a more readable way.
pub type Fun<T, U> = <U as Ho<T>>::Fun;

impl<T> Ho<Arg<T>> for f64 {type Fun = Func<T, f64>;}
impl<T> Ho<Arg<T>> for f32 {type Fun = Func<T, f32>;}
impl<T> Ho<Arg<T>> for u8 {type Fun = Func<T, u8>;}
impl<T> Ho<Arg<T>> for u16 {type Fun = Func<T, u16>;}
impl<T> Ho<Arg<T>> for u32 {type Fun = Func<T, u32>;}
impl<T> Ho<Arg<T>> for u64 {type Fun = Func<T, u64>;}
impl<T> Ho<Arg<T>> for usize {type Fun = Func<T, usize>;}
impl<T> Ho<Arg<T>> for i8 {type Fun = Func<T, i8>;}
impl<T> Ho<Arg<T>> for i16 {type Fun = Func<T, i16>;}
impl<T> Ho<Arg<T>> for i32 {type Fun = Func<T, i32>;}
impl<T> Ho<Arg<T>> for i64 {type Fun = Func<T, i64>;}
impl<T> Ho<Arg<T>> for isize {type Fun = Func<T, isize>;}

/// Higher order pairing.
///
/// A higher order pairing is used pair up components of a pair of data structures.
/// This is used before binary higher order maps of the type `f : (T, T) -> U`.
pub trait HPair {
    /// Output type.
    type Out;
    /// Returns the higher order transposed value.
    fn hpair(self) -> Self::Out;
}

impl HPair for (f64, f64) {type Out = Self; fn hpair(self) -> Self {self}}
impl HPair for (f32, f32) {type Out = Self; fn hpair(self) -> Self {self}}
impl HPair for (u8, u8) {type Out = Self; fn hpair(self) -> Self {self}}
impl HPair for (u16, u16) {type Out = Self; fn hpair(self) -> Self {self}}
impl HPair for (u32, u32) {type Out = Self; fn hpair(self) -> Self {self}}
impl HPair for (u64, u64) {type Out = Self; fn hpair(self) -> Self {self}}
impl HPair for (usize, usize) {type Out = Self; fn hpair(self) -> Self {self}}
impl HPair for (i8, i8) {type Out = Self; fn hpair(self) -> Self {self}}
impl HPair for (i16, i16) {type Out = Self; fn hpair(self) -> Self {self}}
impl HPair for (i32, i32) {type Out = Self; fn hpair(self) -> Self {self}}
impl HPair for (i64, i64) {type Out = Self; fn hpair(self) -> Self {self}}
impl HPair for (isize, isize) {type Out = Self; fn hpair(self) -> Self {self}}

impl<T> HPair for ([T; 2], [T; 2]) where (T, T): HPair {
    type Out = [<(T, T) as HPair>::Out; 2];
    fn hpair(self) -> Self::Out {
        let ([a, b], [c, d]) = self;
        [(a, c).hpair(), (b, d).hpair()]
    }
}

impl<T> HPair for ([T; 3], [T; 3]) where (T, T): HPair {
    type Out = [<(T, T) as HPair>::Out; 3];
    fn hpair(self) -> Self::Out {
        let ([a, b, c], [d, e, f]) = self;
        [(a, d).hpair(), (b, e).hpair(), (c, f).hpair()]
    }
}

impl<T> HPair for ([T; 4], [T; 4]) where (T, T): HPair {
    type Out = [<(T, T) as HPair>::Out; 4];
    fn hpair(self) -> Self::Out {
        let ([a, b, c, d], [e, f, g, h]) = self;
        [(a, e).hpair(), (b, f).hpair(), (c, g).hpair(), (d, h).hpair()]
    }
}

impl<T> HPair for ([T; 5], [T; 5]) where (T, T): HPair {
    type Out = [<(T, T) as HPair>::Out; 5];
    fn hpair(self) -> Self::Out {
        let ([a, b, c, d, e], [f, g, h, i, j]) = self;
        [
            (a, f).hpair(),
            (b, g).hpair(),
            (c, h).hpair(),
            (d, i).hpair(),
            (e, j).hpair()
        ]
    }
}

impl<T> HPair for ([T; 6], [T; 6]) where (T, T): HPair {
    type Out = [<(T, T) as HPair>::Out; 6];
    fn hpair(self) -> Self::Out {
        let ([a, b, c, d, e, f], [g, h, i, j, k, l]) = self;
        [
            (a, g).hpair(),
            (b, h).hpair(),
            (c, i).hpair(),
            (d, j).hpair(),
            (e, k).hpair(),
            (f, l).hpair()
        ]
    }
}

impl<T> HPair for (Vec<T>, Vec<T>) where (T, T): HPair {
    type Out = Vec<<(T, T) as HPair>::Out>;
    fn hpair(self) -> Self::Out {
        let (a, b) = self;
        a.into_iter().zip(b.into_iter()).map(|n| n.hpair()).collect()
    }
}

/// Implemented by higher order maps.
///
/// A higher order map takes common data structures such as
/// vectors and lists and applies a function to every element.
///
/// This is implemented recursively, hence higher order maps.
pub trait HMap<Out> {
    /// The out type.
    type Fun;
    /// Maps structure.
    fn hmap(self, f: &Self::Fun) -> Out;
}

impl<T, U> HMap<U> for T
where U: Call<T> {
    type Fun = U::Fun;
    fn hmap(self, f: &Self::Fun) -> U {
        <U as Call<T>>::call(f, self)
    }
}

impl<T, U> HMap<[U; 2]> for [T; 2]
where T: HMap<U> {
    type Fun = T::Fun;
    fn hmap(self, f: &Self::Fun) -> [U; 2] {
        let [a, b] = self;
        [a.hmap(f), b.hmap(f)]
    }
}

impl<T, U> HMap<[U; 3]> for [T; 3]
where T: HMap<U> {
    type Fun = T::Fun;
    fn hmap(self, f: &Self::Fun) -> [U; 3] {
        let [a, b, c] = self;
        [a.hmap(f), b.hmap(f), c.hmap(f)]
    }
}

impl<T, U> HMap<[U; 4]> for [T; 4]
where T: HMap<U> {
    type Fun = T::Fun;
    fn hmap(self, f: &Self::Fun) -> [U; 4] {
        let [a, b, c, d] = self;
        [a.hmap(f), b.hmap(f), c.hmap(f), d.hmap(f)]
    }
}

impl<T, U> HMap<[U; 5]> for [T; 5]
where T: HMap<U> {
    type Fun = T::Fun;
    fn hmap(self, f: &Self::Fun) -> [U; 5] {
        let [a, b, c, d, e] = self;
        [a.hmap(f), b.hmap(f), c.hmap(f), d.hmap(f), e.hmap(f)]
    }
}

impl<T, U> HMap<[U; 6]> for [T; 6]
where T: HMap<U> {
    type Fun = T::Fun;
    fn hmap(self, fx: &Self::Fun) -> [U; 6] {
        let [a, b, c, d, e, f] = self;
        [a.hmap(fx), b.hmap(fx), c.hmap(fx), d.hmap(fx), e.hmap(fx), f.hmap(fx)]
    }
}

impl<T, U> HMap<Vec<U>> for Vec<T>
where T: HMap<U> {
    type Fun = T::Fun;
    fn hmap(self, f: &Self::Fun) -> Vec<U> {
        self.into_iter().map(|n| n.hmap(f)).collect()
    }
}
