use crate::{constants::XSUM_THRESHOLD, xsum_large, xsum_small};

pub trait Xsum {
    fn new() -> Self;
    fn add_list(&mut self, vec: &[f64]);
    fn add(&mut self, value: f64);
    fn sum(&mut self) -> f64;
}

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
