#[allow(unused)]
#[macro_export]
macro_rules! refine_const {
    ($value:expr, $output:ty) => {{
        type Input = <$output as Refined>::Input;
        
        const VALUE: Input = $value.into();

        const RESULT: $output = {
            if <$output>::holds(&VALUE) {
                <$output>::new(VALUE)
            } else {
                panic!("predicate does not hold at build time");
            }
        };

        RESULT
    }};
}

#[allow(unused)]
#[macro_export]
macro_rules! refine {
    ($value:expr, $output:ty) => {{
        type Input = <$output as Refined>::Input;

        let value: Input = $value.into();

        if <$output>::holds(&value) {
            <$output>::new(value)
        } else {
            panic!("predicate does not hold at run time");
        }
    }};
}
