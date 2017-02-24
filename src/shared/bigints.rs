const LOW_BITS: u64 = ((1 as u64) << 32) - 1;
const HIGH_BITS: u64 = LOW_BITS << 32;
fn big_prod(first: u64, second: u64) -> (u64, u64) {
    let flow:   u64 = first & LOW_BITS;
    let slow:   u64 = second & LOW_BITS;
    let fhigh:  u64 = (first & HIGH_BITS) >> 32;
    let shigh:  u64 = (second & HIGH_BITS) >> 32;
    let reslow: u64 = flow * slow;
    let resm1:  u64 = fhigh * slow;
    let resm2:  u64 = flow * shigh;
    let secdig: u64 = (resm1 & LOW_BITS) + (resm2 & LOW_BITS) + ((reslow & HIGH_BITS) >> 32);
    (((secdig & HIGH_BITS) >> 32) + ((resm1 & HIGH_BITS) >> 32) + ((resm2 & HIGH_BITS) >> 32) + fhigh * shigh,
     ((secdig & LOW_BITS) << 32) | (reslow & LOW_BITS))
}