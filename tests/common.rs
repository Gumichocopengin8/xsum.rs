use xsum::{xsum_large, xsum_small};

fn is_valid(actual: f64, expected: f64) -> bool {
    if !actual.is_nan()
        && (actual != expected || actual.is_sign_negative() != expected.is_sign_negative())
    {
        return false;
    }
    if !actual.is_nan() && expected.is_nan() {
        return false;
    }
    true
}

pub fn same_value(vec: &[f64], expected: f64) {
    let mut xsumsmall = xsum_small::XsumSmall::new();
    xsumsmall.addv(vec);
    assert!(is_valid(xsumsmall.sum(), expected));

    let mut xsumlarge = xsum_large::XsumLarge::new();
    xsumlarge.addv(vec);
    assert!(is_valid(xsumlarge.sum(), expected));
}
