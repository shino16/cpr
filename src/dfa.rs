use lib2::fxhash::*;
// (otherwise) use std::collections::HashMap;
use crate::int::*;
use std::cmp::Ordering;
use std::hash::Hash;

pub const DIGITS: &[u8] = b"0123456789";

pub trait Dfa {
    type Alphabet;
    type State;
    fn init(&self) -> Self::State;
    fn next(&self, s: Self::State, a: Self::Alphabet, i: usize) -> Self::State;
    fn accept(&self, s: Self::State) -> bool;
    fn successful(&self, _: Self::State) -> bool { false }
    fn unsuccessful(&self, _: Self::State) -> bool { false }
    fn count<I: Num>(&self, n: usize, alphabet: &[Self::Alphabet]) -> I
    where
        Self::Alphabet: Copy,
        Self::State: Eq + Hash + Copy,
    {
        count(self, n, alphabet)
    }
}

pub struct And<X, Y>(pub X, pub Y);

impl<X: Dfa<Alphabet = A>, Y: Dfa<Alphabet = A>, A: Copy> Dfa for And<X, Y> {
    type Alphabet = A;
    type State = (X::State, Y::State);
    fn init(&self) -> Self::State { (self.0.init(), self.1.init()) }
    fn next(&self, (s0, s1): Self::State, a: Self::Alphabet, i: usize) -> Self::State {
        (self.0.next(s0, a, i), self.1.next(s1, a, i))
    }
    fn accept(&self, (s0, s1): Self::State) -> bool {
        self.0.accept(s0) && self.1.accept(s1)
    }
    fn successful(&self, (s0, s1): Self::State) -> bool {
        self.0.successful(s0) && self.1.successful(s1)
    }
    fn unsuccessful(&self, (s0, s1): Self::State) -> bool {
        self.0.unsuccessful(s0) || self.1.unsuccessful(s1)
    }
}

pub struct Prod<X, Y>(pub X, pub Y);

impl<X: Dfa, Y: Dfa> Dfa for Prod<X, Y> {
    type Alphabet = (X::Alphabet, Y::Alphabet);
    type State = (X::State, Y::State);
    fn init(&self) -> Self::State { (self.0.init(), self.1.init()) }
    fn next(
        &self,
        (s0, s1): Self::State,
        (a0, a1): Self::Alphabet,
        i: usize,
    ) -> Self::State {
        (self.0.next(s0, a0, i), self.1.next(s1, a1, i))
    }
    fn accept(&self, (s0, s1): Self::State) -> bool {
        self.0.accept(s0) && self.1.accept(s1)
    }
    fn successful(&self, (s0, s1): Self::State) -> bool {
        self.0.successful(s0) && self.1.successful(s1)
    }
    fn unsuccessful(&self, (s0, s1): Self::State) -> bool {
        self.0.unsuccessful(s0) || self.1.unsuccessful(s1)
    }
}

pub struct Not<X>(pub X);

impl<X: Dfa> Dfa for Not<X> {
    type Alphabet = X::Alphabet;
    type State = X::State;
    fn init(&self) -> Self::State { self.0.init() }
    fn next(&self, s: Self::State, a: Self::Alphabet, i: usize) -> Self::State {
        self.0.next(s, a, i)
    }
    fn accept(&self, s: Self::State) -> bool { !self.0.accept(s) }
    fn successful(&self, s: Self::State) -> bool { self.0.unsuccessful(s) }
    fn unsuccessful(&self, s: Self::State) -> bool { self.0.successful(s) }
}

pub struct Lt<'a>(pub &'a [u8]);

impl Dfa for Lt<'_> {
    type Alphabet = u8;
    type State = Ordering;
    fn init(&self) -> Self::State { Ordering::Equal }
    fn next(&self, s: Self::State, a: Self::Alphabet, i: usize) -> Self::State {
        s.then(a.cmp(&self.0[i]))
    }
    fn accept(&self, s: Self::State) -> bool { s == Ordering::Less }
    fn successful(&self, s: Self::State) -> bool { s == Ordering::Less }
    fn unsuccessful(&self, s: Self::State) -> bool { s == Ordering::Greater }
}

pub struct Leq<'a>(pub &'a [u8]);

impl Dfa for Leq<'_> {
    type Alphabet = u8;
    type State = Ordering;
    fn init(&self) -> Self::State { Ordering::Equal }
    fn next(&self, s: Self::State, a: Self::Alphabet, i: usize) -> Self::State {
        s.then(a.cmp(&self.0[i]))
    }
    fn accept(&self, s: Self::State) -> bool { s != Ordering::Greater }
    fn successful(&self, s: Self::State) -> bool { s == Ordering::Less }
    fn unsuccessful(&self, s: Self::State) -> bool { s == Ordering::Greater }
}

pub struct MultipleOf(pub u32);

impl Dfa for MultipleOf {
    type Alphabet = u8;
    type State = u32;
    fn init(&self) -> Self::State { 0 }
    fn next(&self, s: Self::State, a: Self::Alphabet, _: usize) -> Self::State {
        (s * 10 + (a - b'0') as u32) % self.0
    }
    fn accept(&self, s: Self::State) -> bool { s == 0 }
}

fn count<I: Num, X: Dfa + ?Sized>(dfa: &X, n: usize, alphabet: &[X::Alphabet]) -> I
where
    X::Alphabet: Copy,
    X::State: Eq + Hash + Copy,
{
    let mut dp = HashMap::<X::State, I>::default();
    let mut dp2 = HashMap::<X::State, I>::default();
    dp.insert(dfa.init(), I::one());
    for i in 0..n {
        dp2.clear();
        for (s, k) in dp.drain() {
            for &a in alphabet {
                let s1 = dfa.next(s, a, i);
                if dfa.unsuccessful(s1) {
                    continue;
                }
                *dp2.entry(s1).or_insert(I::zero()) += k;
            }
        }
        std::mem::swap(&mut dp, &mut dp2);
    }
    let mut sum = I::zero();
    for (s, k) in dp {
        if dfa.accept(s) {
            sum += k;
        }
    }
    sum
}
