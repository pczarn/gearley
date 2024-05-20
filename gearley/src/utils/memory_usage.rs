use memusage::MemoryReport;
use bit_vec::BitVec;

impl MemoryReport for BitVec {
    fn indirect(&self) -> usize {
        (self.capacity() + 31) / 32 * 4
    }
}
