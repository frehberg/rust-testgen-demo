#![cfg(test)]
extern crate test_case_derive;

use test_case_derive::test_case;

#[test_case( 2,  4 :: "when both operands are possitive")]
#[test_case( 4,  2 :: "when operands are swapped")]
#[test_case(-2, -4 :: "when both operands are negative")]
fn multiplication_tests(x: i8, y: i8) {
    let actual = (x * y).abs();

    assert_eq!(8, actual)
}