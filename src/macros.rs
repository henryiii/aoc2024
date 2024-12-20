#[macro_export]
macro_rules! make_test {
    ($part:literal, $input:literal, $answer:literal) => {
        paste::paste! {
            #[test]
            fn [<test_part $part>]() {
                assert_eq!(super::[<solution_ $part>](include_str!(concat!("../../../samples/2024/", $input))), $answer);
            }
        }
    };
}