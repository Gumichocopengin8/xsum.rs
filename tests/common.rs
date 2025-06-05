use xsum::{Xsum, XsumAuto, XsumExt, XsumLarge, XsumSmall};

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
    // XsumSmall
    let mut xsumsmall = XsumSmall::new();
    xsumsmall.add_list(vec);
    assert!(is_valid(xsumsmall.sum(), expected));

    // XsumLarge
    let mut xsumlarge = XsumLarge::new();
    xsumlarge.add_list(vec);
    assert!(is_valid(xsumlarge.sum(), expected));

    // XsumAuto
    let mut xsumauto = XsumAuto::new();
    xsumauto.add_list(vec);
    assert!(is_valid(xsumauto.sum(), expected));

    // XsumExt
    assert!(is_valid(vec.xsum(), expected));
}
