//! nshuffle
//!
//! # Abstract
//!
//! This library provides specialized shuffling functions intended for use as with a sort of
//! Secret Santa toolkit, but the shuffle can be used with a slice of any type.
//!
//! # Examples
//!
//! Here we demonstrate that the shuffled version of source contains no values remaining
//! in their original position. This is equired to prevent anyone getting their own name.
//!
//! ```
//! extern crate nshuffle;
//! extern crate rand;
//!
//! let mut rng = rand::weak_rng();
//! let source = [1, 2, 3, 4];
//! let target = nshuffle::shuffle(&mut rng, &source);
//!
//! assert!(source.iter().zip(target.iter()).all(|(a,b)| a != b));
//! ```
//!
//! Here we demonstrate that the extended-shuffled version of source contains no values
//! remaining in their original position, and that no value is in its position in the
//! second slice, either. This is necessary to prevent anyone getting their own name *and*
//! to stop anyone from getting the same name they had last year.
//!
//! ```
//! extern crate nshuffle;
//! extern crate rand;
//!
//! let mut rng = rand::weak_rng();
//! let source = [1, 2, 3, 4];
//! let prior = [2, 1, 4, 3];
//! let target = nshuffle::extended_shuffle(&mut rng, &source, &prior);
//!
//! assert!(source.iter().zip(target.iter()).all(|(a,b)| a != b));
//! assert!(prior.iter().zip(target.iter()).all(|(a,b)| a != b));
//! ```

#![feature(collections)]

extern crate rand;
use rand::Rng;

pub fn shuffle<R: Rng, T: PartialEq + Clone>(r: &mut R, s: &[T]) -> Vec<T> {
    let mut vec = Vec::new();
    vec.push_all(s);

    while any_eq(&vec, s) {
        r.shuffle(&mut vec);
    }
    vec
}

pub fn extended_shuffle<R: Rng, T: PartialEq + Clone>(r: &mut R, s: &[T], p: &[T]) -> Vec<T> {
    let mut vec = Vec::new();
    vec.push_all(s);

    while extended_any_eq(&vec, s, p) {
        r.shuffle(&mut vec)
    }
    vec
}

fn any_eq<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    for i in 0..a.len() {
        if a[i] == b[i] {
            return true;
        }
    }
    false
}

fn extended_any_eq<T: PartialEq>(a: &[T], b: &[T], c: &[T]) -> bool {
    for i in 0..a.len() {
        if a[i] == b[i] || a[i] == c[i] {
            return true;
        }
    }
    false
}
