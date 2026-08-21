use serde::Serialize;
use serde_with::skip_serializing_none;
use validator::Validate;

use crate::api::shared::opt_bool_as_u8;

// Referenced only from doc links below; the imports keep them resolvable.
#[allow(unused_imports)]
use super::{SystemApi, SystemStatus};

/// Parameters for [`SystemApi::get_system_status`].
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct GetSystemStatusRequest {
    /// If true, ResourceSpace checks database connectivity only and returns
    /// early — [`SystemStatus::results`] will be empty.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub basic: Option<bool>,
}

impl GetSystemStatusRequest {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn basic(mut self, basic: bool) -> Self {
        self.basic = Some(basic);
        self
    }
}

/// Parameters for [`SystemApi::do_report`].
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DoReportRequest {
    /// The ID of the report to run, as returned by
    /// [`SystemApi::get_reports`].
    pub report_ref: u32,
    /// Start of the range, as `YYYY-MM-DD`. Defaults to seven days ago when
    /// omitted.
    pub from_date: Option<String>,
    /// End of the range, as `YYYY-MM-DD`. Defaults to today when omitted.
    pub to_date: Option<String>,
}

impl DoReportRequest {
    #[must_use]
    pub fn new(report_ref: u32) -> Self {
        Self {
            report_ref,
            from_date: None,
            to_date: None,
        }
    }

    #[must_use]
    pub fn from_date(mut self, from_date: impl Into<String>) -> Self {
        self.from_date = Some(from_date.into());
        self
    }

    #[must_use]
    pub fn to_date(mut self, to_date: impl Into<String>) -> Self {
        self.to_date = Some(to_date.into());
        self
    }
}

/// Parameters for [`SystemApi::get_daily_stat_summary`].
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Validate)]
pub struct GetDailyStatSummaryRequest {
    /// Number of past days to include in the summary (1–365). Defaults to 30 when omitted.
    #[validate(range(min = 1, max = 365))]
    pub days: Option<u16>,
}

impl GetDailyStatSummaryRequest {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn days(mut self, days: u16) -> Self {
        self.days = Some(days);
        self
    }
}
