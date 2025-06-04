use xsum::{XsumExt, XsumLarge, XsumSmall};

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
    let mut xsumsmall = XsumSmall::new();
    xsumsmall.add_list(vec);
    assert!(is_valid(xsumsmall.sum(), expected));

    let mut xsumlarge = XsumLarge::new();
    xsumlarge.add_list(vec);
    assert!(is_valid(xsumlarge.sum(), expected));

    assert!(is_valid(vec.xsum(), expected));
}
