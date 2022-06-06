#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum RetryDelayType {
    Linear = 0,
    SlowingDown20Percent = 1,
    SlowingDown50Percent = 2,
}
