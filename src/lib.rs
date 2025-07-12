#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::style,
    clippy::todo
)]
#![deny(
    clippy::suspicious,
    clippy::correctness,
    clippy::complexity,
    clippy::missing_const_for_fn,
    unsafe_code
)]
#![feature(const_trait_impl)]
#![cfg_attr(not(test), no_std)]

#[macro_use]
pub mod macros;

#[const_trait]
pub trait Refined {
    type Input;

    fn new(input: Self::Input) -> Self;

    fn holds(input: &Self::Input) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::Refined;

    #[derive(Debug)]
    pub struct NotZero {
        num: u64,
    }

    impl const Refined for NotZero {
        type Input = u64;

        fn new(input: Self::Input) -> Self {
            Self { num: input }
        }

        fn holds(input: &u64) -> bool {
            *input != 0
        }
    }

    impl From<NotZero> for u64 {
        fn from(value: NotZero) -> Self {
            value.num
        }
    }

    #[derive(Debug)]
    pub struct AboveTen {
        num: u64,
    }

    impl const Refined for AboveTen {
        type Input = u64;

        fn new(input: Self::Input) -> Self {
            Self { num: input }
        }

        fn holds(input: &u64) -> bool {
            *input > 10
        }
    }

    impl From<AboveTen> for u64 {
        fn from(value: AboveTen) -> Self {
            value.num
        }
    }

    #[derive(Debug)]
    pub struct NotNull<T> {
        ptr: *const T,
    }

    impl<T> const Refined for NotNull<T> {
        type Input = *const T;

        fn new(input: <Self as Refined>::Input) -> Self {
            Self { ptr: input }
        }

        fn holds(input: &*const T) -> bool {
            !input.is_null()
        }
    }

    impl<T> From<NotNull<T>> for *const T {
        fn from(value: NotNull<T>) -> Self {
            value.ptr
        }
    }

    #[derive(Debug)]
    pub struct Len10 {
        string: String,
    }

    impl const Refined for Len10 {
        type Input = String;

        fn new(input: Self::Input) -> Self {
            Self { string: input }
        }

        fn holds(input: &Self::Input) -> bool {
            input.len() <= 10
        }
    }

    impl From<Len10> for String {
        fn from(value: Len10) -> Self {
            value.string
        }
    }

    #[test]
    #[should_panic]
    fn test_0_to_not_zero() {
        refine!(0u64, NotZero);
    }

    #[test]
    fn test_1_to_not_zero() {
        let refined = refine!(1u64, NotZero);
        assert_eq!(1, refined.num);
    }

    #[test]
    fn test_self_to_not_zero() {
        let refined = refine!(1u64, NotZero);
        let refined = refine!(refined, NotZero);
        assert_eq!(1, refined.num);
    }

    #[test]
    fn test_above_ten_to_not_zero() {
        let refined = refine!(11u64, AboveTen);
        let refined = refine!(refined, NotZero);
        assert_eq!(11, refined.num);
    }

    #[test]
    #[should_panic]
    fn test_10_to_above_ten() {
        refine!(10u64, AboveTen);
    }

    #[test]
    fn test_11_to_above_ten() {
        let refined = refine!(11u64, AboveTen);
        assert_eq!(11, refined.num);
    }

    #[test]
    fn test_not_zero_to_above_ten() {
        let refined = refine!(11u64, NotZero);
        let refined = refine!(refined, AboveTen);
        assert_eq!(11, refined.num);
    }

    #[test]
    #[should_panic]
    fn test_not_zero_to_above_ten_panic() {
        let refined = refine!(4u64, NotZero);
        refine!(refined, AboveTen);
    }

    #[test]
    fn test_self_to_above_ten() {
        let refined = refine!(11u64, AboveTen);
        let refined = refine!(refined, AboveTen);
        assert_eq!(11, refined.num);
    }

    #[test]
    #[should_panic]
    fn test_null_to_not_null() {
        refine!(std::ptr::null::<u64>(), NotNull::<u64>);
    }

    #[test]
    fn test_u64_ptr_to_not_null() {
        let num = 10;
        let refined = refine!(std::ptr::from_ref(&num), NotNull::<u64>);
        assert_eq!(std::ptr::from_ref(&num), refined.ptr);
    }

    #[test]
    fn test_self_to_not_null() {
        let num = 10;
        let refined = refine!(std::ptr::from_ref(&num), NotNull::<u64>);
        let refined = refine!(refined, NotNull::<u64>);
        assert_eq!(std::ptr::from_ref(&num), refined.ptr);
    }

    #[test]
    #[should_panic]
    fn test_string11_to_len10() {
        refine!("1".repeat(11), Len10);
    }

    #[test]
    fn test_string3_to_len10() {
        let refined = refine!("123", Len10);
        assert_eq!("123", refined.string);
    }

    #[test]
    fn test_self_to_len10() {
        let refined = refine!("123", Len10);
        let refined = refine!(refined, Len10);
        assert_eq!("123", refined.string);
    }
}
