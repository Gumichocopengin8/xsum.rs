use xsum::{Xsum, XsumAuto, XsumExt, XsumLarge, XsumSmall};

fn is_valid(actual: f64, expected: f64) -> bool {
    // check NaN, 0, -0, infinity, -infinity, finite values
    if actual.to_bits() == expected.to_bits() {
        return true;
    }

    // check NaN with no payload
    if actual.is_nan() && expected.is_nan() {
        return true;
    }
    false
}

pub(crate) fn same_value(vec: &[f64], expected: f64) {
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
