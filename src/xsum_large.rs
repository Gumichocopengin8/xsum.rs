use crate::{
    accumulators::large_accumulator::LargeAccumulator, constants::XSUM_MANTISSA_BITS,
    xsum_small::XsumSmall,
};

pub struct XsumLarge {
    m_lacc: LargeAccumulator,
}

impl Default for XsumLarge {
    fn default() -> Self {
        Self::new()
    }
}

impl XsumLarge {
    pub fn new() -> Self {
        Self {
            m_lacc: LargeAccumulator::new(),
        }
    }

    pub fn from_xsum_small(xsmall: XsumSmall) -> Self {
        let mut lacc = LargeAccumulator::new();
        lacc.m_sacc = xsmall.transfer_accumulator();
        Self { m_lacc: lacc }
    }

    pub fn add_list(&mut self, vec: &[f64]) {
        for value in vec {
            self.add(*value);
        }
    }

    pub fn add(&mut self, value: f64) {
        // increment
        self.m_lacc.m_sacc.increment_when_value_added(value);

        // Convert to integer form in uintv
        let uintv: u64 = value.to_bits();

        // Isolate the upper sign+exponent bits that index the chunk.
        let ix: usize = (uintv >> XSUM_MANTISSA_BITS) as usize;

        // Find the count for this chunk, and subtract one.
        let count: i32 = self.m_lacc.m_count[ix] - 1;

        if count < 0 {
            // If the decremented count is negative, it's either a special
            // Inf/NaN chunk (in which case count will stay at -1), or one that
            // needs to be transferred to the small accumulator, or one that
            // has never been used before and needs to be initialized.
            self.m_lacc.large_add_value_inf_nan(ix, uintv);
        } else {
            // Store the decremented count of additions allowed before transfer,
            // and add this value to the chunk.
            self.m_lacc.m_count[ix] = count;
            self.m_lacc.m_chunk[ix] = self.m_lacc.m_chunk[ix].wrapping_add(uintv);
        }
    }

    pub fn sum(&mut self) -> f64 {
        self.m_lacc.transfer_to_small();
        let mut xsum_smal = XsumSmall::new_with(&self.m_lacc.m_sacc);
        xsum_smal.sum()
    }
}
