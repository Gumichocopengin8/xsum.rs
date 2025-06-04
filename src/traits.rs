use crate::{constants::XSUM_THRESHOLD, xsum_large, xsum_small};

pub trait XsumExt {
    fn xsum(&self) -> f64;
}

impl XsumExt for [f64] {
    fn xsum(&self) -> f64 {
        if self.len() > XSUM_THRESHOLD {
            let mut xsumsmall = xsum_small::XsumSmall::new();
            xsumsmall.add_list(self);
            xsumsmall.sum()
        } else {
            let mut xsumlarge = xsum_large::XsumLarge::new();
            xsumlarge.add_list(self);
            xsumlarge.sum()
        }
    }
}
