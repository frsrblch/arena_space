vector_and_scalar! {
    struct TestVector([struct TestScalar(f64); 2]) {
        fn in_test(test) -> Self;
    }
}

#[test]
fn scalar_add_tests() {
    let a = TestScalar::in_test(2.0);
    let b = TestScalar::in_test(3.0);

    let expected = TestScalar::in_test(5.0);

    assert_eq!(expected, a + b);
    assert_eq!(expected, &a + b);
    assert_eq!(expected, a + &b);
    assert_eq!(expected, &a + &b);
}

#[test]
fn scalar_add_assign_tests() {
    let a_0 = TestScalar::in_test(2.0);
    let b = TestScalar::in_test(3.0);

    let expected = TestScalar::in_test(5.0);

    let mut a = a_0;
    a += b;
    assert_eq!(expected, a);

    let mut a = a_0;
    a += &b;
    assert_eq!(expected, a);
}

#[test]
fn scalar_sub_tests() {
    let a = TestScalar::in_test(2.0);
    let b = TestScalar::in_test(3.0);

    let expected = TestScalar::in_test(-1.0);

    assert_eq!(expected, a - b);
    assert_eq!(expected, &a - b);
    assert_eq!(expected, a - &b);
    assert_eq!(expected, &a - &b);
}

#[test]
fn scalar_sub_assign_tests() {
    let a_0 = TestScalar::in_test(2.0);
    let b = TestScalar::in_test(3.0);

    let expected = TestScalar::in_test(-1.0);

    let mut a = a_0;
    a -= b;
    assert_eq!(expected, a);

    let mut a = a_0;
    a -= &b;
    assert_eq!(expected, a);
}

#[test]
fn scalar_mul_test() {
    let a = TestScalar::in_test(2.0);
    let b = 3.0f64;

    let expected = TestScalar::in_test(6.0);

    assert_eq!(expected, a * b);
    assert_eq!(expected, &a * b);
    assert_eq!(expected, a * &b);
    assert_eq!(expected, &a * &b);

    assert_eq!(expected, b * a);
    assert_eq!(expected, &b * a);
    assert_eq!(expected, b * &a);
    assert_eq!(expected, &b * &a);
}

#[test]
fn scalar_mul_assign_test() {
    let a_0 = TestScalar::in_test(2.0);
    let b = 3.0f64;

    let expected = TestScalar::in_test(6.0);

    let mut a = a_0;
    a *= b;
    assert_eq!(expected, a);

    let mut a = a_0;
    a *= &b;
    assert_eq!(expected, a);
}

#[test]
fn scalar_div_test() {
    let a = TestScalar::in_test(2.0);
    let b = 3.0f64;

    let expected = TestScalar::in_test(2.0 / 3.0);

    assert_eq!(expected, a / b);
    assert_eq!(expected, &a / b);
    assert_eq!(expected, a / &b);
    assert_eq!(expected, &a / &b);
}

#[test]
fn scalar_div_assign_test() {
    let a_0 = TestScalar::in_test(6.0);
    let b = 3.0f64;

    let expected = TestScalar::in_test(2.0);

    let mut a = a_0;
    a /= b;
    assert_eq!(expected, a);

    let mut a = a_0;
    a /= &b;
    assert_eq!(expected, a);
}

scalar! {
    struct Num(f64) {
        fn in_test(test) -> Self;
    }
}

scalar! {
    struct Den(f64) {
        fn in_test(test) -> Self;
    }
}

scalar! {
    struct Res(f64) {
        fn in_test(test) -> Self;
    }
}

scalar_div! { Num | Den = Res}

#[test]
fn scalar_div_conversion_test() {
    let num = Num::in_test(6.0);
    let den = Den::in_test(2.0);
    let res = Res::in_test(3.0);

    assert_eq!(res, num / den);
    assert_eq!(res, &num / den);
    assert_eq!(res, num / &den);
    assert_eq!(res, &num / &den);
}

#[test]
fn scalar_mul_conversion_test() {
    let num = Num::in_test(6.0);
    let den = Den::in_test(2.0);
    let res = Res::in_test(3.0);

    assert_eq!(num, res * den);
    assert_eq!(num, &res * den);
    assert_eq!(num, res * &den);
    assert_eq!(num, &res * &den);

    assert_eq!(num, den * res);
    assert_eq!(num, &den * res);
    assert_eq!(num, den * &res);
    assert_eq!(num, &den * &res);
}

#[test]
fn scalar_rem_test() {
    let a = TestScalar::in_test(5.0);
    let b = TestScalar::in_test(3.0);

    let rem = TestScalar::in_test(2.0);

    assert_eq!(rem, a % b);
    assert_eq!(rem, a % &b);
    assert_eq!(rem, &a % b);
    assert_eq!(rem, &a % &b);
}

#[test]
fn scalar_neg_test() {
    let a = TestScalar::in_test(2.0);

    let neg = TestScalar::in_test(-2.0);

    assert_eq!(neg, -&a);
    assert_eq!(neg, -a);
}

#[test]
fn vector_add_test() {
    let a = TestVector::in_test(2.0, 3.0);
    let b = TestVector::in_test(5.0, 7.0);

    let expected = TestVector::in_test(7.0, 10.0);

    assert_eq!(expected, a + b);
    assert_eq!(expected, &a + b);
    assert_eq!(expected, a + &b);
    assert_eq!(expected, &a + &b);
}

#[test]
fn vector_add_assign_test() {
    let a_0 = TestVector::in_test(2.0, 3.0);
    let b = TestVector::in_test(5.0, 7.0);

    let expected = TestVector::in_test(7.0, 10.0);

    let mut a = a_0;
    a += b;
    assert_eq!(expected, a);

    let mut a = a_0;
    a += &b;
    assert_eq!(expected, a);
}

#[test]
fn vector_sub_test() {
    let a = TestVector::in_test(2.0, 3.0);
    let b = TestVector::in_test(5.0, 7.0);

    let expected = TestVector::in_test(-3.0, -4.0);

    assert_eq!(expected, a - b);
    assert_eq!(expected, &a - b);
    assert_eq!(expected, a - &b);
    assert_eq!(expected, &a - &b);
}

#[test]
fn vector_sub_assign_test() {
    let a_0 = TestVector::in_test(2.0, 3.0);
    let b = TestVector::in_test(5.0, 7.0);

    let expected = TestVector::in_test(-3.0, -4.0);

    let mut a = a_0;
    a -= b;
    assert_eq!(expected, a);

    let mut a = a_0;
    a -= &b;
    assert_eq!(expected, a);
}

#[test]
fn vector_mul_test() {
    let a = TestVector::in_test(2.0, 3.0);
    let b = 5.0f64;

    let expected = TestVector::in_test(10.0, 15.0);

    assert_eq!(expected, a * b);
    assert_eq!(expected, &a * b);
    assert_eq!(expected, a * &b);
    assert_eq!(expected, &a * &b);

    assert_eq!(expected, b * a);
    assert_eq!(expected, &b * a);
    assert_eq!(expected, b * &a);
    assert_eq!(expected, &b * &a);
}

#[test]
fn vector_mul_assign_test() {
    let a_0 = TestVector::in_test(2.0, 3.0);
    let b = 5.0f64;

    let expected = TestVector::in_test(10.0, 15.0);

    let mut a = a_0;
    a *= b;
    assert_eq!(expected, a);

    let mut a = a_0;
    a *= &b;
    assert_eq!(expected, a);
}

#[test]
fn vector_div_test() {
    let a = TestVector::in_test(10.0, 15.0);
    let b = 5.0f64;

    let expected = TestVector::in_test(2.0, 3.0);

    assert_eq!(expected, a / b);
    assert_eq!(expected, &a / b);
    assert_eq!(expected, a / &b);
    assert_eq!(expected, &a / &b);
}

#[test]
fn vector_div_assign_test() {
    let a_0 = TestVector::in_test(10.0, 15.0);
    let b = 5.0f64;

    let expected = TestVector::in_test(2.0, 3.0);

    let mut a = a_0;
    a /= b;
    assert_eq!(expected, a);

    let mut a = a_0;
    a /= &b;
    assert_eq!(expected, a);
}

#[test]
fn vector_neg_test() {
    let a = TestVector::in_test(2.0, 3.0);

    let neg = TestVector::in_test(-2.0, -3.0);

    assert_eq!(neg, -a);
    assert_eq!(neg, -&a);
}

#[cfg(debug_assertions)]
mod debug_tests {
    use super::TestScalar;

    #[test]
    #[should_panic]
    fn nan_is_invalid() {
        TestScalar::new(f64::NAN);
    }

    #[test]
    #[should_panic]
    fn infinite_is_invalid() {
        TestScalar::new(f64::INFINITY);
    }

    #[test]
    #[should_panic]
    fn negative_infinite_is_invalid() {
        TestScalar::new(f64::NEG_INFINITY);
    }

    #[test]
    #[should_panic]
    fn overflow_panics() {
        TestScalar::new(f64::MAX) * 2.0;
    }
}
