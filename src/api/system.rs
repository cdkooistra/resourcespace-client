use serde::Serialize;
use validator::Validate;

use crate::client::Client;
use crate::error::RsError;

/// Sub-API for system endpoints.
#[derive(Debug)]
pub struct SystemApi<'a> {
    client: &'a Client,
}

impl<'a> SystemApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get system status - healthcheck information.
    ///
    /// ## Arguments
    /// `None`
    ///
    /// ## Returns
    /// Returns back system status information (configuration dependant - e.g mysql_log_transactions).
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::Client;
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// let status = client.system().get_system_status().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_system_status(&self) -> Result<serde_json::Value, RsError> {
        self.client
            .send_request("get_system_status", reqwest::Method::GET, ())
            .await
    }

    /// Return a summary of daily statistics by activity type.
    ///
    /// Note max 365 days as only the current and previous year's data is accessed.
    ///
    /// ## Arguments
    /// * `request` - Parameters built via [`GetDailyStatSummaryRequest`]
    ///
    /// ## Returns
    ///
    /// Returns an array of daily statistics.
    ///
    /// ## TODO: Errors
    ///
    /// ## Examples
    /// ```no_run
    /// # use resourcespace_client::{Client, api::system::GetDailyStatSummaryRequest};
    /// # async fn example(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// // Default — last 30 days
    /// let stats = client.system()
    ///     .get_daily_stat_summary(GetDailyStatSummaryRequest::new())
    ///     .await?;
    ///
    /// // Last 7 days
    /// let stats = client.system()
    ///     .get_daily_stat_summary(GetDailyStatSummaryRequest::new().days(7))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_daily_stat_summary(
        &self,
        request: GetDailyStatSummaryRequest,
    ) -> Result<serde_json::Value, RsError> {
        request
            .validate()
            .map_err(|e| RsError::Validation(e.to_string()))?;
        self.client
            .send_request("get_daily_stat_summary", reqwest::Method::GET, request)
            .await
    }

    pub async fn get_reports(&self) -> Result<serde_json::Value, RsError> {
        todo!("available from RS v11.0")
    }

    pub async fn do_report(&self) -> Result<serde_json::Value, RsError> {
        todo!("available from RS v11.0")
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Validate)]
pub struct GetDailyStatSummaryRequest {
    /// Number of past days to include in the summary (1–365). Defaults to 30 when omitted.
    #[validate(range(min = 1, max = 365))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<u16>,
}

impl GetDailyStatSummaryRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn days(mut self, days: u16) -> Self {
        self.days = Some(days);
        self
    }
}
