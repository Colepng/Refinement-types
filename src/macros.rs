#[allow(unused)]
macro_rules! refine_into_const {
    ($value:expr, $input:ty, $restriction:ty, $output:ty) => {{
        use crate::traits::Restriction;
        #[derive(Debug)]
        #[repr(transparent)]
        struct RefinedMacro {
            value: $input,
        }

        type Predicate = PredicateStruct<$input, $restriction>;

        const RESULT: RefinedMacro = const {
            if <$restriction>::holds(&$value) {
                RefinedMacro { value: $value }
            } else {
                panic!("predicate does not hold at compile time");
            }
        };

        const fn eq<A: Into<B>, B: Into<A>>() -> bool {
            true
        }

        let result = if eq::<Predicate, <$output as Refined>::Predicate>() {
            unsafe { std::mem::transmute::<RefinedMacro, $output>(RESULT) }
        } else {
            panic!("failed to cast to output type");
        };

        result
    }};
}

#[allow(unused)]
macro_rules! refine_into {
    ($value:expr, $input:ty, $restriction:ty, $output:ty) => {{
        #[derive(Debug)]
        #[repr(transparent)]
        struct RefinedMacro {
            value: $input,
        }

        type Predicate = PredicateStruct<$input, $restriction>;

        let result = if <$restriction>::holds(&$value) {
            RefinedMacro { value: $value }
        } else {
            panic!("predicate does not hold at run time");
        };

        const fn eq<A: Into<B>, B: Into<A>>() -> bool {
            true
        }

        let result = if eq::<Predicate, <$output as Refined>::Predicate>() {
            unsafe { std::mem::transmute::<RefinedMacro, $output>(result) }
        } else {
            panic!("failed to cast to output type");
        };

        result
    }};
}

#[allow(unused)]
macro_rules! refine_const {
    ($value:expr, $output:ty) => {{
        use crate::traits::Predicate;
        use crate::traits::Restriction;

        type RestrictionMacro = <<$output as Refined>::Predicate as Predicate>::Restriction;

        const RESULT: $output = {
            if <RestrictionMacro>::holds(&$value) {
                <$output>::new($value)
            } else {
                panic!("predicate does not hold at run time");
            }
        };

        RESULT
    }};
}

#[allow(unused)]
macro_rules! refine_v1 {
    ($value:expr, $output:ty) => {{
        use crate::traits::Predicate;
        use crate::traits::Restriction;

        type RestrictionMacro = <<$output as Refined>::Predicate as Predicate>::Restriction;

        if <RestrictionMacro>::holds(&$value) {
            <$output>::new($value)
        } else {
            panic!("predicate does not hold at run time");
        }
    }};
}

#[allow(unused)]
macro_rules! refine {
    ($value:expr, $output:ty) => {{
        use crate::traits::Predicate;
        use crate::traits::Restriction;

        type RestrictionMacro = <<$output as Refined>::Predicate as Predicate>::Restriction;
        type Input = <<$output as Refined>::Predicate as Predicate>::Input;

        let value: Input = $value.into();

        if <RestrictionMacro>::holds(&value) {
            <$output>::new(value)
        } else {
            panic!("predicate does not hold at run time");
        }
    }};
}

#[allow(unused)]
macro_rules! refine_into_v2 {
    ($value:expr, $input:ty, $restriction:ty, $output:ty) => {{
        #[derive(Debug)]
        #[repr(transparent)]
        struct RefinedMacro {
            value: $input,
        }

        type Predicate = PredicateStruct<$input, $restriction>;

        let result = if <$restriction>::holds(&$value) {
            RefinedMacro { value: $value }
        } else {
            panic!("predicate does not hold at run time");
        };

        const fn eq<A: Into<B>, B: Into<A>>() -> bool {
            true
        }

        let result = if eq::<Predicate, <$output as Refined>::Predicate>() {
            unsafe { std::mem::transmute::<RefinedMacro, $output>(result) }
        } else {
            panic!("failed to cast to output type");
        };

        result
    }};
}
