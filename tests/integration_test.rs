
use rust_learning::unit_test::subtract;
#[test]
fn it_adds_two() {
    let result = subtract(2, 1);
    assert_eq!(result, 1);
}