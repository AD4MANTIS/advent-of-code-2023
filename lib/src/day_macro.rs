#[macro_export]
macro_rules! day {
    ($day: literal, $part: expr, $($answers:tt)*) => {
        $crate::day_main!($day, $part);

        $crate::day_test!($part, $($answers)*);
    };
}

#[macro_export]
macro_rules! day_main {
    ($day: literal, $part: expr) => {
        fn main() {
            let _timer = lib::PrintTimer::new(&$day.to_string());

            let input = include_str!("./input.txt");
            let output = $part(input);
            dbg!(output);
        }
    };
}

#[macro_export]
macro_rules! day_test {
    (
        $part: expr,
        $($name: ident $(($test_file: literal))? => $result: literal),+
    ) => {
        #[cfg(test)]
        $crate::paste::item! {
            mod [< $part _tests >] {
                use super::*;

                $(
                    $crate::paste::item! {
                        #[test]
                        fn [< $name _works >]() {
                            let result = $part(include_str!($crate::get_test_file!($name $(, $test_file)?)));
                            assert_eq!(result, $result);
                        }
                    }
                )+
            }
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! get_test_file {
    (answer) => {
        "input.txt"
    };
    (test) => {
        "test-input.txt"
    };
    ($_:ident, $test_file:literal) => {
        $test_file
    };
}
