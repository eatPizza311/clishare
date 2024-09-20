pub mod data;
pub mod domain;
pub mod service;

// Reexport some frequently used type to crate root
pub use data::DataError;
pub use domain::clip::field::ShortCode;
pub use domain::clip::ClipError;
pub use domain::time::Time;
pub use domain::Clip;
pub use service::ServiceError;
