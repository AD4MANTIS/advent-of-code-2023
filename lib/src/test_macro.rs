#[macro_export]
macro_rules! tests {
    {
        $tests_name: ident

        $(init:
            $init: item
        )?

        test:
        $($test: item)*
    } => {
        #[cfg(test)]
        $crate::paste::item! {
            mod [<$tests_name>] {
                $($init)?

                $(
                    #[test]
                    $test
                )*
            }
        }
    };
}
