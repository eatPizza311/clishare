// declare submodule
mod clip_id;
// export the field structure to field module so it can be direct access from outside
pub use clip_id::ClipId;

mod content;
pub use content::Content;

mod expires;
pub use expires::Expires;

mod hits;
pub use hits::Hits;

mod password;
pub use password::Password;

mod posted;
pub use posted::Posted;

mod shortcode;
pub use shortcode::ShortCode;

mod title;
pub use title::Title;
