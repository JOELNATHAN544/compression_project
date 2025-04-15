pub mod lz;
pub mod rle;
pub mod do_lz;
pub mod do_rle;

// Re-export main functionality
pub use do_lz::{compress_file, decompress_file, handle_cli};
pub use lz::{compress_lz, decompress_lz};
pub use rle::{compress_rle, decompress_rle};

// Just expose the compression modules
// pub use rle;
// pub use lz;