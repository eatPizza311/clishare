use std::convert::TryFrom;

use chrono::{NaiveDateTime, Utc};

use crate::data::DbId;
use crate::domain::clip::field::Posted;
use crate::{ClipError, ShortCode, Time};

/// Clip that directly converted from sqlx::Row
#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64,
}

impl TryFrom<Clip> for crate::domain::Clip {
    type Error = ClipError;

    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        use std::str::FromStr;

        Ok(Self {
            clip_id: field::ClipId::new(DbId::from_str(clip.clip_id.as_str())?),
            shortcode: field::ShortCode::from(clip.shortcode.as_str()),
            content: field::Content::new(clip.content.as_str())?,
            title: field::Title::new(clip.title),
            posted: field::Posted::new(Time::from_naive_utc(clip.posted)),
            expires: field::Expires::new(clip.expires.map(Time::from_naive_utc)),
            password: field::Password::new(clip.password.unwrap_or_default())?,
            hits: field::Hits::new(u64::try_from(clip.hits)?),
        })
    }
}

/// Use shortcode to query a clip
pub struct GetClip {
    pub(in crate::data) shortcode: String,
}

// Convert ask::GetClip to model::GetClip in order to query the database
// Service layer -> Data layer
impl From<crate::service::ask::GetClip> for GetClip {
    fn from(req: crate::service::ask::GetClip) -> Self {
        Self {
            shortcode: req.shortcode.into_inner(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        GetClip {
            shortcode: shortcode.into_inner(),
        }
    }
}

impl From<String> for GetClip {
    fn from(shortcode: String) -> Self {
        GetClip { shortcode }
    }
}

/// New clip with hits = 0, and other specify by user
pub struct NewClip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    // Store the dates in database as number of seconds instead of doing string convertion
    pub(in crate::data) posted: i64,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

// Service layer -> Data layer
impl From<crate::service::ask::NewClip> for NewClip {
    fn from(req: crate::service::ask::NewClip) -> Self {
        Self {
            clip_id: DbId::new().into(),
            shortcode: ShortCode::default().into(),
            content: req.content.into_inner(),
            title: req.title.into_inner(),
            posted: Utc::now().timestamp(),
            expires: req.expires.into_inner().map(|time| time.timestamp()),
            password: req.password.into_inner(),
        }
    }
}

/// Update clip without modify the clip_id & posted date
pub struct UpdateClip {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

// Service layer -> Data layer
impl From<crate::service::ask::UpdateClip> for UpdateClip {
    fn from(req: crate::service::ask::UpdateClip) -> Self {
        Self {
            shortcode: req.shortcode.as_str().to_string(),
            content: req.content.into_inner(),
            title: req.title.into_inner(),
            expires: req.expires.into_inner().map(|time| time.timestamp()),
            password: req.password.into_inner(),
        }
    }
}
