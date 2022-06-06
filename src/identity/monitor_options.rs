#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum MonitorOptions {
    None = 0,
    AcceptNotFoundAsNotAnError = 1,
}
