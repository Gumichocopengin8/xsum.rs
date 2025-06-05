use crate::{XsumLarge, XsumSmall, constants::XSUM_THRESHOLD, traits::Xsum};

enum XsumKind {
    XSmall(XsumSmall),
    XLarge(XsumLarge),
}

pub struct XsumAuto {
    m_xsum: XsumKind,
}

impl Default for XsumAuto {
    fn default() -> Self {
        Self::new()
    }
}

impl XsumAuto {
    #[inline(always)]
    fn transform_to_large(&mut self) {
        let should_transform = match &self.m_xsum {
            XsumKind::XSmall(xsmall) => xsmall.get_size_count() > XSUM_THRESHOLD,
            _ => false,
        };
        if !should_transform {
            return;
        }

        let old_xsum = std::mem::replace(&mut self.m_xsum, XsumKind::XSmall(XsumSmall::default()));

        self.m_xsum = match old_xsum {
            XsumKind::XSmall(xsmall) => XsumKind::XLarge(XsumLarge::from_xsum_small(xsmall)),
            other => other,
        };
    }
}

impl Xsum for XsumAuto {
    fn new() -> Self {
        Self {
            m_xsum: XsumKind::XSmall(XsumSmall::new()),
        }
    }

    fn add_list(&mut self, vec: &[f64]) {
        match &mut self.m_xsum {
            XsumKind::XSmall(xsmal) => {
                xsmal.add_list(vec);
                self.transform_to_large();
            }
            XsumKind::XLarge(xlarge) => {
                xlarge.add_list(vec);
            }
        }
    }

    fn add(&mut self, value: f64) {
        match &mut self.m_xsum {
            XsumKind::XSmall(xsmal) => {
                xsmal.add(value);
                self.transform_to_large();
            }
            XsumKind::XLarge(xlarge) => {
                xlarge.add(value);
            }
        }
    }

    fn sum(&mut self) -> f64 {
        match &mut self.m_xsum {
            XsumKind::XSmall(xsmall) => xsmall.sum(),
            XsumKind::XLarge(xlarge) => xlarge.sum(),
        }
    }
}
