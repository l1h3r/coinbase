use chrono::DateTime as Chrono;
use chrono::Utc;
use serde_json::Value;
use std::collections::HashMap;

use crate::error::Error;
use crate::types::Response;

pub type SimpleMap = HashMap<String, String>;

pub type ValueMap = HashMap<String, Value>;

pub type UtcDate = Chrono<Utc>;

pub type CBResult<T> = Result<Response<T>, Error>;
