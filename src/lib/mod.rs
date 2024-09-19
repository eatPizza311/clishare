pub mod data;
pub mod domain;

// Reexport some frequently used type to crate root
pub use domain::clip::field::ShortCode;
pub use domain::clip::ClipError;
pub use domain::time::Time;
