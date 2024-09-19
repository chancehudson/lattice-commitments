pub trait Norm {
    fn norm_l1(&self) -> u64;

    /// Calculate the l2 norm for this polynomial. That is
    /// the square root of the summation of each coefficient squared
    fn norm_l2(&self) -> u64;

    /// Calculate the l-infinity norm for this polynomial. That is
    /// the largest coefficient
    fn norm_max(&self) -> u64;
}
