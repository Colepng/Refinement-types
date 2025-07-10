#![feature(const_trait_impl)]
#![allow(unused)]
use crate::traits::{PredicateStruct, Refined, Restriction};

#[macro_use]
mod macros;
mod traits;

fn main() {
    let number = refine_const!(1, NotZero);
    let not_zero = refine!(number, NotZero);
    let _above_ten = refine!(not_zero, AboveTen);

    let num = 10;
    let _not_null = refine!(std::ptr::from_ref(&num), NotNull<u64>);
}

#[derive(Debug)]
pub struct NotZero {
    num: u64,
}

impl const Restriction<u64> for NotZero {
    fn holds(input: &u64) -> bool {
        *input != 0
    }
}

impl const Refined for NotZero {
    type Input = u64;
    type Predicate = PredicateStruct<Self::Input, NotZero>;

    fn new(input: Self::Input) -> Self {
        Self { num: input }
    }
}

impl From<NotZero> for u64 {
    fn from(value: NotZero) -> Self {
        value.num
    }
}

#[derive(Debug)]
pub struct NotNull<T> {
    ptr: *const T
}

impl<T> const Restriction<*const T> for NotNull<T> {
    fn holds(input: &*const T) -> bool {
        !input.is_null()
    }
}

impl<T> const Refined for NotNull<T> {
    type Input = *const T;

    type Predicate = PredicateStruct<Self::Input, NotNull<T>>;

    fn new(input: <NotNull<T> as Refined>::Input) -> Self {
        Self {
            ptr: input
        }
    }
}

#[derive(Debug)]
pub struct AboveTen {
    num: u64,
}

impl const Restriction<u64> for AboveTen {
    fn holds(input: &u64) -> bool {
        *input > 10
    }
}

impl const Refined for AboveTen {
    type Input = u64;
    type Predicate = PredicateStruct<Self::Input, AboveTen>;

    fn new(input: Self::Input) -> Self {
        Self { num: input }
    }
}
