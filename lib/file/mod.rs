mod load;
mod save;

pub use load::*;
pub use save::*;

// 10 bits needed per number
// (10 * 100 / 8 = 125 bytes)
/// The maximum size of saved memory in bytes
pub const MAX_FILE_SIZE: usize = 125;
