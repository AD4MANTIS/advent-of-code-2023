/**
Can be used to write the boilerplate for a test part binary.<br>
Generates a main function that measures the duration it takes to get the answer, and prints it to std-out.<br>
Also generates a 1-n test with this format, separated by `,`:<br>
`(test_name ([raw("test data")] | [("./input-file.txt")]) => expected_result)+`<br>
For the following `test_name`s the following default `input-file` is assumed, otherwise you have to specify it:<br>

| test_name | input_file          |
|-----------|---------------------|
| test      | "test-input.txt"    |
| example   | "example-input.txt" |
| answer    | "input.txt"         |

# Examples

```ignore
# #[macro_use] extern crate lib;
lib::day!(09, part2, test => 3, answer => 42);
```

<br>

```ignore
# #[macro_use] extern crate lib;
lib::day!(
    09,
    part2,
    test raw("3") => 3,
    another_test raw("4 2") => 42
);
```

<br>

```
# #[macro_use] extern crate lib;
# fn part2(input: &str) -> usize {
#   input.replace(' ', "").parse().expect("should only contain numbers and space")
# }
lib::day_test!(
    09,
    part2,
    test raw("3") => 3,
    another_test raw("4 2") => 42
);
```
*/
#[macro_export]
macro_rules! day {
    ($day: literal, $part: expr, $($answers:tt)*) => {
        $crate::day_main!($day, $part);

        $crate::day_test!($day, $part, $($answers)*);
    };
}

#[macro_export]
macro_rules! day_main {
    ($day: literal, $part: expr) => {
        fn main() {
            let _timer = lib::PrintTimer::new(&("day-".to_owned() + stringify!($day)));

            let input = include_str!("./input.txt");
            let output = $part(input);
            dbg!(output);
        }
    };
}

#[macro_export]
macro_rules! day_test {
    (
        $day: literal,
        $part: expr,
        $($name: ident $($raw: ident)?$(($test_file: literal))? => $result: literal),+
    ) => {
        #[cfg(test)]
        $crate::paste::item! {
            mod [< $part _tests >] {
                use super::*;

                $(
                    #[test]
                    fn [< day_ $day _ $name _works >]() {
                        let result = $part($crate::get_test_file!($name $($raw)?$( $test_file)?));
                        assert_eq!(result, $result);
                    }
                )+
            }
        }
    };
}

#[macro_export]
macro_rules! get_test_file {
    (test) => {
        include_str!("test-input.txt")
    };
    (example) => {
        include_str!("example-input.txt")
    };
    (answer) => {
        include_str!("input.txt")
    };
    ($_:ident raw $raw_test_content:literal) => {
        $raw_test_content
    };
    ($_:ident $test_file:literal) => {
        include_str!($test_file)
    };
}
