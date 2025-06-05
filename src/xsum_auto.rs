use crate::{XsumLarge, XsumSmall, constants::XSUM_THRESHOLD};

enum Xsum {
    XSmall(XsumSmall),
    XLarge(XsumLarge),
}

pub struct XsumAuto {
    m_xsum: Xsum,
}

impl Default for XsumAuto {
    fn default() -> Self {
        Self::new()
    }
}

impl XsumAuto {
    pub fn new() -> Self {
        Self {
            m_xsum: Xsum::XSmall(XsumSmall::new()),
        }
    }

    pub fn add_list(&mut self, vec: &[f64]) {
        match &mut self.m_xsum {
            Xsum::XSmall(xsmal) => {
                xsmal.add_list(vec);
                self.transform_to_large();
            }
            Xsum::XLarge(xlarge) => {
                xlarge.add_list(vec);
            }
        }
    }

    pub fn add(&mut self, value: f64) {
        match &mut self.m_xsum {
            Xsum::XSmall(xsmal) => {
                xsmal.add(value);
                self.transform_to_large();
            }
            Xsum::XLarge(xlarge) => {
                xlarge.add(value);
            }
        }
    }

    pub fn sum(&mut self) -> f64 {
        match &mut self.m_xsum {
            Xsum::XSmall(xsmall) => xsmall.sum(),
            Xsum::XLarge(xlarge) => xlarge.sum(),
        }
    }

    #[inline(always)]
    fn transform_to_large(&mut self) {
        let should_transform = match &self.m_xsum {
            Xsum::XSmall(xsmall) => xsmall.get_size_count() > XSUM_THRESHOLD,
            _ => false,
        };
        if !should_transform {
            return;
        }

        let old_xsum = std::mem::replace(&mut self.m_xsum, Xsum::XSmall(XsumSmall::default()));

        self.m_xsum = match old_xsum {
            Xsum::XSmall(xsmall) => Xsum::XLarge(XsumLarge::from_xsum_small(xsmall)),
            other => other,
        };
    }
}
