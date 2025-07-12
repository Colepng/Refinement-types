#[const_trait]
pub trait Refined {
    type Input;

    fn new(input: Self::Input) -> Self;

    fn holds(input: &Self::Input) -> bool;
}

// #[const_trait]
// pub trait Restriction<I> {
//     fn holds(input: &I) -> bool;
// }

// #[allow(unused)]
// pub struct PredicateStruct<I, R: Restriction<I>> {
//     input: I,
//     restriction: R,
// }

// pub trait Predicate {
//     type Input;
//     type Restriction: Restriction<Self::Input>;
// }

// impl<I, R: Restriction<I>> Predicate for PredicateStruct<I, R> {
//     type Input = I;
//     type Restriction = R;
// }

// pub struct Not<const PTR: *const T, T> {
//     // phatom_data: PhantomData<T>,
// }

// impl<T, const PTR: *const T> const Restriction<T> for Not<T> {
//     fn holds(input: &T) -> bool {
//         input != T
//     }
// }

// pub struct NotEqual<const NUM: u64>;

// pub struct GreaterThen<const NUM: u64>;

// impl<const NUM: u64> const Restriction<u64> for NotEqual<NUM> {
//     fn holds(input: &u64) -> bool {
//         *input != NUM
//     }
// }

// impl<const NUM: u64> const Restriction<u64> for GreaterThen<NUM> {
//     fn holds(input: &u64) -> bool {
//         *input > NUM
//     }
// }
