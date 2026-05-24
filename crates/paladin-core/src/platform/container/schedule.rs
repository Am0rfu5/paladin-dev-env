//! Schedule and job scheduling domain types.
//!
//! These are plain data types with no port references, suitable for the core
//! container layer. They are used by the `SchedulerOrchestrator` in the
//! application layer.

use crate::base::component::action::ActionStatus;
use crate::platform::container::job::Job;
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid::Uuid;

/// Defines when a scheduled job should execute.
#[derive(Debug, Clone)]
pub enum Schedule {
    /// Run at regular intervals
    Interval(Duration),
    /// Run daily at specific time (hour, minute)
    Daily(u32, u32),
    /// Run weekly on specific day and time (day of week 0=Sunday, hour, minute)
    Weekly(u8, u32, u32),
    /// Run monthly on specific day and time (day of month, hour, minute)
    Monthly(u32, u32, u32),
    /// Run once at a specific time
    Once(DateTime<Utc>),
    /// Run on scheduler startup
    OnStartup,
}

/// A job paired with its recurrence schedule.
#[derive(Debug, Clone)]
pub struct ScheduledJob {
    /// The job to execute
    pub job: Job,
    /// When to execute the job
    pub schedule: Schedule,
    /// Whether this scheduled job is currently active
    pub enabled: bool,
    /// Next scheduled execution time
    pub next_run: Option<DateTime<Utc>>,
    /// Last execution time
    pub last_run: Option<DateTime<Utc>>,
    /// Total number of executions completed
    pub run_count: u32,
}

/// Display information for a scheduled job (monitoring / CLI output).
#[derive(Debug, Clone)]
pub struct ScheduledJobInfo {
    /// Job identifier
    pub id: Uuid,
    /// Human-readable job name
    pub name: String,
    /// Whether this job is enabled
    pub enabled: bool,
    /// Next scheduled run time
    pub next_run: Option<DateTime<Utc>>,
    /// Last run time
    pub last_run: Option<DateTime<Utc>>,
    /// Execution count
    pub run_count: u32,
    /// Number of tasks in the job
    pub task_count: usize,
    /// Current action status
    pub status: ActionStatus,
    /// Schedule definition
    pub schedule: Schedule,
}

/// Aggregate statistics for the scheduler.
///
/// Plain data type with no port references \u2014 suitable for core container layer.
#[derive(Debug, Clone)]
pub struct SchedulerStats {
    /// Total scheduled jobs registered
    pub total_jobs: usize,
    /// Number of enabled jobs
    pub enabled_jobs: usize,
    /// Total job executions completed across all jobs
    pub total_runs: u32,
    /// Number of registered task services
    pub total_services: usize,
    /// Name of the next job scheduled to run
    pub next_job_name: Option<String>,
    /// Time the next job is scheduled to run
    pub next_job_time: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_stats_fields() {
        let stats = SchedulerStats {
            total_jobs: 5,
            enabled_jobs: 3,
            total_runs: 42,
            total_services: 2,
            next_job_name: Some("backup".to_string()),
            next_job_time: None,
        };
        assert_eq!(stats.total_jobs, 5);
        assert_eq!(stats.enabled_jobs, 3);
        assert_eq!(stats.total_runs, 42);
        assert_eq!(stats.next_job_name.as_deref(), Some("backup"));
    }

    #[test]
    fn test_schedule_variants() {
        let _interval = Schedule::Interval(Duration::from_secs(3600));
        let _daily = Schedule::Daily(9, 30);
        let _weekly = Schedule::Weekly(1, 9, 0);
        let _monthly = Schedule::Monthly(1, 9, 0);
        let _once = Schedule::Once(Utc::now());
        let _startup = Schedule::OnStartup;
    }
}
