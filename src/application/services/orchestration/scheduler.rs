/*
Scheduler Orchestrator

Application-layer orchestrator for job scheduling. Relocated from
`core/platform/manager/scheduler.rs`.

Renamed `Scheduler` → `SchedulerOrchestrator`. A backwards-compatible type alias
`Scheduler = SchedulerOrchestrator` is provided.

`Schedule`, `ScheduledJob`, `ScheduledJobInfo`, and `SchedulerStats` are re-exported
from their canonical location in `paladin-core::platform::container::schedule`.
`SchedulerError` is defined in `super::types`.
*/

use super::types::SchedulerError;
use crate::core::base::component::action::ActionPriority;
use crate::core::platform::container::job::Job;
use crate::core::platform::container::task::{
    ContentIndexingService, DataBackupService, EmailNotificationService, Task, TaskService,
};
use chrono::{DateTime, Datelike, Utc};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::interval;
use uuid::Uuid;

pub use crate::core::platform::container::schedule::{
    Schedule, ScheduledJob, ScheduledJobInfo, SchedulerStats,
};

/// Application-layer job scheduler orchestrator.
///
/// Renamed from `Scheduler`. A backwards-compatible type alias is provided.
pub struct SchedulerOrchestrator {
    scheduled_jobs: HashMap<Uuid, ScheduledJob>,
    services: HashMap<String, Box<dyn TaskService>>,
    running: bool,
    tick_interval: Duration,
}

/// Backwards-compatible alias for `SchedulerOrchestrator`.
pub type Scheduler = SchedulerOrchestrator;

impl SchedulerOrchestrator {
    pub fn new() -> Self {
        let mut scheduler = Self {
            scheduled_jobs: HashMap::new(),
            services: HashMap::new(),
            running: false,
            tick_interval: Duration::from_secs(60), // Check every minute
        };

        // Register default services
        scheduler.register_default_services();
        scheduler
    }

    /// Register a task service.
    pub fn register_service(&mut self, service: Box<dyn TaskService>) {
        let service_name = service.name().to_string();
        self.services.insert(service_name.clone(), service);
        log::info!("Registered service: {}", service_name);
    }

    /// Register default services.
    fn register_default_services(&mut self) {
        self.register_service(Box::new(DataBackupService::new(
            std::env::temp_dir()
                .join("paladin_backups")
                .to_string_lossy()
                .to_string(),
        )));

        self.register_service(Box::new(ContentIndexingService::new("main_index")));

        self.register_service(Box::new(EmailNotificationService::new("localhost:587")));
    }

    /// Add a job to the scheduler.
    pub fn add_job(&mut self, job: Job, schedule: Schedule) -> Result<Uuid, SchedulerError> {
        // Validate that all required services are available
        for task in &job.tasks {
            if !self.services.contains_key(&task.service_name) {
                return Err(SchedulerError::ServiceNotFound(task.service_name.clone()));
            }
        }

        let job_id = job.id();
        let next_run = Self::calculate_next_run(&schedule);

        let scheduled_job = ScheduledJob {
            job,
            schedule,
            enabled: true,
            next_run,
            last_run: None,
            run_count: 0,
        };

        self.scheduled_jobs.insert(job_id, scheduled_job);
        log::info!("Added job {} to scheduler", job_id);
        Ok(job_id)
    }

    /// Remove a job from the scheduler.
    pub fn remove_job(&mut self, job_id: Uuid) -> bool {
        match self.scheduled_jobs.remove(&job_id) {
            Some(scheduled_job) => {
                log::info!("Removed job '{}' from scheduler", scheduled_job.job.name());
                true
            }
            None => false,
        }
    }

    /// Enable a scheduled job.
    pub fn enable_job(&mut self, job_id: Uuid) -> bool {
        if let Some(scheduled_job) = self.scheduled_jobs.get_mut(&job_id) {
            scheduled_job.enabled = true;
            scheduled_job.next_run = Self::calculate_next_run(&scheduled_job.schedule);
            log::info!("Enabled job '{}'", scheduled_job.job.name());
            true
        } else {
            false
        }
    }

    /// Disable a scheduled job.
    pub fn disable_job(&mut self, job_id: Uuid) -> bool {
        if let Some(scheduled_job) = self.scheduled_jobs.get_mut(&job_id) {
            scheduled_job.enabled = false;
            scheduled_job.next_run = None;
            log::info!("Disabled job '{}'", scheduled_job.job.name());
            true
        } else {
            false
        }
    }

    /// Start the scheduler.
    pub async fn start(&mut self) {
        if self.running {
            log::info!("Scheduler is already running");
            return;
        }

        self.running = true;
        log::info!(
            "Starting scheduler with {} jobs and {} services",
            self.scheduled_jobs.len(),
            self.services.len()
        );

        // Add default jobs if none exist
        if self.scheduled_jobs.is_empty() {
            self.add_default_jobs();
        }

        // Execute startup jobs
        self.execute_startup_jobs().await;

        let mut interval = interval(self.tick_interval);

        while self.running {
            interval.tick().await;
            self.check_and_execute_jobs().await;
        }
    }

    /// Stop the scheduler.
    pub fn stop(&mut self) {
        self.running = false;
        log::info!("Scheduler stopped");
    }

    /// Execute jobs that are scheduled to run on startup.
    async fn execute_startup_jobs(&mut self) {
        let startup_job_ids: Vec<_> = self
            .scheduled_jobs
            .iter()
            .filter(|(_, scheduled_job)| {
                scheduled_job.enabled && matches!(scheduled_job.schedule, Schedule::OnStartup)
            })
            .map(|(id, _)| *id)
            .collect();

        for job_id in startup_job_ids {
            self.execute_job(job_id).await;
        }
    }

    /// Check for jobs that need to be executed and execute them.
    async fn check_and_execute_jobs(&mut self) {
        let now = Utc::now();
        let mut jobs_to_execute = Vec::new();

        // Collect jobs that need to be executed
        for (job_id, scheduled_job) in &self.scheduled_jobs {
            if scheduled_job.enabled
                && let Some(next_run) = scheduled_job.next_run
                && now >= next_run
            {
                jobs_to_execute.push(*job_id);
            }
        }

        // Execute collected jobs
        for job_id in jobs_to_execute {
            self.execute_job(job_id).await;
        }
    }

    /// Execute a specific job.
    async fn execute_job(&mut self, job_id: Uuid) {
        if let Some(scheduled_job) = self.scheduled_jobs.get_mut(&job_id) {
            log::info!("Executing scheduled job: '{}'", scheduled_job.job.name());

            let start_time = Utc::now();

            // Execute the job
            match scheduled_job.job.execute(&self.services).await {
                Ok(_) => {
                    let stats = scheduled_job.job.job_stats();
                    log::info!(
                        "Job '{}' completed successfully: {} of {} tasks completed ({}% success rate)",
                        scheduled_job.job.name(),
                        stats.completed_tasks,
                        stats.total_tasks,
                        stats.success_rate
                    );
                }
                Err(e) => {
                    log::info!("Job '{}' failed: {}", scheduled_job.job.name(), e);

                    // Log partial completion info if available
                    let stats = scheduled_job.job.job_stats();
                    if stats.completed_tasks > 0 {
                        log::info!(
                            "  Partial completion: {} of {} tasks completed",
                            stats.completed_tasks,
                            stats.total_tasks
                        );
                    }
                }
            }

            // Update scheduling info
            scheduled_job.last_run = Some(start_time);
            scheduled_job.run_count += 1;

            // Calculate next run time
            scheduled_job.next_run = match &scheduled_job.schedule {
                Schedule::Once(_) => None,   // One-time jobs don't repeat
                Schedule::OnStartup => None, // Startup jobs only run once
                _ => Self::calculate_next_run(&scheduled_job.schedule),
            };

            if let Some(next_run) = scheduled_job.next_run {
                log::info!(
                    "Next run for job '{}': {}",
                    scheduled_job.job.name(),
                    next_run
                );
            } else {
                log::info!("Job '{}' will not run again", scheduled_job.job.name());
            }
        }
    }

    /// Add default jobs to demonstrate the scheduler.
    fn add_default_jobs(&mut self) {
        // Data backup job - runs every 6 hours
        let backup_task = Task::new(
            "Daily Backup".to_string(),
            "Performs daily data backup".to_string(),
            "DataBackupService".to_string(),
        );

        let backup_job = Job::new(
            "Daily Backup Job".to_string(),
            "Automated daily backup job".to_string(),
            vec![backup_task],
        )
        .with_priority(ActionPriority::High);

        if let Err(e) = self.add_job(
            backup_job,
            Schedule::Interval(Duration::from_secs(6 * 3600)),
        ) {
            log::info!("Failed to add backup job: {}", e);
        }

        // Content indexing job - runs every hour
        let indexing_task = Task::new(
            "Content Indexing".to_string(),
            "Indexes content for search".to_string(),
            "ContentIndexingService".to_string(),
        );

        let indexing_job = Job::new(
            "Content Indexing Job".to_string(),
            "Automated content indexing job".to_string(),
            vec![indexing_task],
        )
        .with_priority(ActionPriority::Normal);

        if let Err(e) = self.add_job(indexing_job, Schedule::Interval(Duration::from_secs(3600))) {
            log::info!("Failed to add indexing job: {}", e);
        }

        // Weekly maintenance job with multiple tasks
        let cleanup_task = Task::new(
            "System Cleanup".to_string(),
            "Cleans up temporary files".to_string(),
            "DataBackupService".to_string(), // Reusing backup service for demo
        );

        let mut email_task = Task::new(
            "Weekly Report".to_string(),
            "Sends weekly status report".to_string(),
            "EmailNotificationService".to_string(),
        );

        // Add email arguments
        email_task
            .action
            .add_argument("to_email".to_string(), "admin@example.com")
            .unwrap();
        email_task
            .action
            .add_argument("subject".to_string(), "Weekly System Report")
            .unwrap();

        let maintenance_job = Job::new(
            "Weekly Maintenance".to_string(),
            "Weekly system maintenance and reporting".to_string(),
            vec![cleanup_task, email_task],
        )
        .with_priority(ActionPriority::Low);

        // Run every Sunday at 2:00 AM
        if let Err(e) = self.add_job(maintenance_job, Schedule::Weekly(0, 2, 0)) {
            log::info!("Failed to add maintenance job: {}", e);
        }

        // Startup job to validate system
        let validation_task = Task::new(
            "System Validation".to_string(),
            "Validates system on startup".to_string(),
            "ContentIndexingService".to_string(),
        );

        let startup_job = Job::new(
            "Startup Validation".to_string(),
            "Validates system health on startup".to_string(),
            vec![validation_task],
        )
        .with_priority(ActionPriority::Critical);

        if let Err(e) = self.add_job(startup_job, Schedule::OnStartup) {
            log::info!("Failed to add startup job: {}", e);
        }
    }

    /// Calculate the next run time for a schedule.
    fn calculate_next_run(schedule: &Schedule) -> Option<DateTime<Utc>> {
        let now = Utc::now();

        match schedule {
            Schedule::Interval(duration) => Some(now + chrono::Duration::from_std(*duration).ok()?),
            Schedule::Daily(hour, minute) => {
                let mut next = now.date_naive().and_hms_opt(*hour, *minute, 0)?;
                if next <= now.naive_utc() {
                    next += chrono::Duration::days(1);
                }
                Some(DateTime::from_naive_utc_and_offset(next, Utc))
            }
            Schedule::Weekly(day_of_week, hour, minute) => {
                let current_weekday = now.weekday().number_from_sunday() as u8;
                let days_until_target = if *day_of_week >= current_weekday {
                    *day_of_week - current_weekday
                } else {
                    7 - current_weekday + *day_of_week
                };

                let target_date =
                    now.date_naive() + chrono::Duration::days(days_until_target as i64);
                let next = target_date.and_hms_opt(*hour, *minute, 0)?;

                // If it's the same day but time has passed, schedule for next week
                if days_until_target == 0 && next <= now.naive_utc() {
                    let next_week = target_date + chrono::Duration::days(7);
                    let next = next_week.and_hms_opt(*hour, *minute, 0)?;
                    Some(DateTime::from_naive_utc_and_offset(next, Utc))
                } else {
                    Some(DateTime::from_naive_utc_and_offset(next, Utc))
                }
            }
            Schedule::Monthly(day, hour, minute) => {
                let mut target_date = now.date_naive();

                // Try to set the day for current month
                if let Some(date_with_day) = target_date.with_day(*day) {
                    let next = date_with_day.and_hms_opt(*hour, *minute, 0)?;
                    if next > now.naive_utc() {
                        return Some(DateTime::from_naive_utc_and_offset(next, Utc));
                    }
                }

                // Move to next month - set to day 1 first to avoid issues with month boundaries
                target_date = target_date.with_day(1)?;
                target_date = if target_date.month() == 12 {
                    target_date
                        .with_year(target_date.year() + 1)?
                        .with_month(1)?
                } else {
                    target_date.with_month(target_date.month() + 1)?
                };

                let next = target_date.with_day(*day)?.and_hms_opt(*hour, *minute, 0)?;
                Some(DateTime::from_naive_utc_and_offset(next, Utc))
            }
            Schedule::Once(datetime) => {
                if *datetime > now {
                    Some(*datetime)
                } else {
                    None // One-time job already passed
                }
            }
            Schedule::OnStartup => {
                None // Startup jobs don't have a next run time
            }
        }
    }

    /// List all scheduled jobs.
    pub fn list_jobs(&self) -> Vec<ScheduledJobInfo> {
        self.scheduled_jobs
            .iter()
            .map(|(id, scheduled_job)| ScheduledJobInfo {
                id: *id,
                name: scheduled_job.job.name().to_string(),
                enabled: scheduled_job.enabled,
                next_run: scheduled_job.next_run,
                last_run: scheduled_job.last_run,
                run_count: scheduled_job.run_count,
                task_count: scheduled_job.job.tasks.len(),
                status: scheduled_job.job.status().clone(),
                schedule: scheduled_job.schedule.clone(),
            })
            .collect()
    }

    /// Get job details.
    pub fn get_job(&self, job_id: Uuid) -> Option<&ScheduledJob> {
        self.scheduled_jobs.get(&job_id)
    }

    /// Get service names.
    pub fn list_services(&self) -> Vec<String> {
        self.services.keys().cloned().collect()
    }

    /// Get scheduler statistics.
    pub fn stats(&self) -> SchedulerStats {
        let total_jobs = self.scheduled_jobs.len();
        let enabled_jobs = self.scheduled_jobs.values().filter(|j| j.enabled).count();
        let total_runs = self.scheduled_jobs.values().map(|j| j.run_count).sum();

        let next_job = self
            .scheduled_jobs
            .values()
            .filter(|j| j.enabled && j.next_run.is_some())
            .min_by_key(|j| j.next_run.unwrap());

        SchedulerStats {
            total_jobs,
            enabled_jobs,
            total_runs,
            total_services: self.services.len(),
            next_job_name: next_job.map(|j| j.job.name().to_string()),
            next_job_time: next_job.and_then(|j| j.next_run),
        }
    }
}

impl Default for SchedulerOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to start the scheduler.
pub async fn start_scheduler() {
    let mut scheduler = SchedulerOrchestrator::new();
    scheduler.start().await;
}

/// Create a new scheduler instance with custom services.
pub fn create_scheduler_with_services(
    services: Vec<Box<dyn TaskService>>,
) -> SchedulerOrchestrator {
    let mut scheduler = SchedulerOrchestrator::new();

    // Clear default services if custom ones are provided
    if !services.is_empty() {
        scheduler.services.clear();
        for service in services {
            scheduler.register_service(service);
        }
    }

    scheduler
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    #[tokio::test]
    async fn test_scheduler_creation() {
        let scheduler = SchedulerOrchestrator::new();
        assert!(!scheduler.running);
        assert_eq!(scheduler.scheduled_jobs.len(), 0);
        assert!(!scheduler.services.is_empty()); // Should have default services
    }

    #[tokio::test]
    async fn test_add_job() {
        let mut scheduler = SchedulerOrchestrator::new();

        let task = Task::new(
            "Test Task".to_string(),
            "A test task".to_string(),
            "DataBackupService".to_string(),
        );

        let job = Job::new("Test Job".to_string(), "A test job".to_string(), vec![task]);

        let job_id = scheduler
            .add_job(job, Schedule::Interval(Duration::from_secs(3600)))
            .unwrap();

        assert_eq!(scheduler.scheduled_jobs.len(), 1);
        assert!(scheduler.scheduled_jobs.contains_key(&job_id));
    }

    #[tokio::test]
    async fn test_service_registration() {
        let mut scheduler = SchedulerOrchestrator::new();
        let initial_service_count = scheduler.services.len();

        // Register a service with a different name to avoid replacement
        let custom_service = DataBackupService {
            backup_path: "/custom/backup".to_string(),
        };

        // Since we're using the same service type, it will replace the existing one
        // Let's test this behavior explicitly
        scheduler.register_service(Box::new(custom_service));

        // The count should remain the same because we replaced an existing service
        assert_eq!(scheduler.services.len(), initial_service_count);

        // Verify the service was actually replaced by checking the backup path
        let service = scheduler.services.get("DataBackupService").unwrap();
        // We can't directly access the backup_path from the trait object,
        // but we can verify the service exists and has the right name
        assert_eq!(service.name(), "DataBackupService");
    }

    #[tokio::test]
    async fn test_service_registration_new_service() {
        let mut scheduler = SchedulerOrchestrator::new();
        let initial_service_count = scheduler.services.len();

        // Create a new service with a unique name
        #[derive(Debug)]
        struct CustomService;

        #[async_trait]
        impl crate::core::platform::container::task::TaskService for CustomService {
            fn name(&self) -> &str {
                "CustomTestService"
            }

            async fn execute(
                &self,
                _action: &crate::core::base::component::action::Action,
            ) -> Result<Option<serde_json::Value>, crate::core::platform::container::task::TaskError>
            {
                Ok(Some(serde_json::json!({"test": "success"})))
            }

            fn clone_service(&self) -> Box<dyn TaskService> {
                Box::new(CustomService)
            }
        }

        scheduler.register_service(Box::new(CustomService));

        // Now the count should increase by 1
        assert_eq!(scheduler.services.len(), initial_service_count + 1);
        assert!(scheduler.services.contains_key("CustomTestService"));
    }

    #[tokio::test]
    async fn test_service_replacement() {
        let mut scheduler = SchedulerOrchestrator::new();
        let initial_service_count = scheduler.services.len();

        // Register a service with the same name as an existing one
        let replacement_service = DataBackupService {
            backup_path: "/replacement/backup".to_string(),
        };

        scheduler.register_service(Box::new(replacement_service));

        // Count should remain the same (replacement, not addition)
        assert_eq!(scheduler.services.len(), initial_service_count);

        // Service should still exist with the same name
        assert!(scheduler.services.contains_key("DataBackupService"));
    }

    #[tokio::test]
    async fn test_schedule_calculation() {
        // Test interval schedule
        let interval_schedule = Schedule::Interval(Duration::from_secs(3600));
        let next_run = SchedulerOrchestrator::calculate_next_run(&interval_schedule);
        assert!(next_run.is_some());

        // Test one-time schedule in the future
        let future_time = Utc::now() + chrono::Duration::hours(1);
        let once_schedule = Schedule::Once(future_time);
        let next_run = SchedulerOrchestrator::calculate_next_run(&once_schedule);
        assert_eq!(next_run, Some(future_time));

        // Test one-time schedule in the past
        let past_time = Utc::now() - chrono::Duration::hours(1);
        let past_schedule = Schedule::Once(past_time);
        let next_run = SchedulerOrchestrator::calculate_next_run(&past_schedule);
        assert_eq!(next_run, None);
    }

    #[tokio::test]
    async fn test_job_enable_disable() {
        let mut scheduler = SchedulerOrchestrator::new();

        let task = Task::new(
            "Test Task".to_string(),
            "A test task".to_string(),
            "DataBackupService".to_string(),
        );

        let job = Job::new("Test Job".to_string(), "A test job".to_string(), vec![task]);

        let job_id = scheduler
            .add_job(job, Schedule::Interval(Duration::from_secs(3600)))
            .unwrap();

        // Test disable
        assert!(scheduler.disable_job(job_id));
        let scheduled_job = scheduler.scheduled_jobs.get(&job_id).unwrap();
        assert!(!scheduled_job.enabled);
        assert!(scheduled_job.next_run.is_none());

        // Test enable
        assert!(scheduler.enable_job(job_id));
        let scheduled_job = scheduler.scheduled_jobs.get(&job_id).unwrap();
        assert!(scheduled_job.enabled);
        assert!(scheduled_job.next_run.is_some());
    }

    #[tokio::test]
    async fn test_error_variants_display() {
        let service_not_found = SchedulerError::ServiceNotFound("TestService".to_string());
        assert_eq!(
            service_not_found.to_string(),
            "Service not found: TestService"
        );

        let job_not_found = SchedulerError::JobNotFound(Uuid::new_v4());
        assert!(job_not_found.to_string().contains("Job not found"));

        let invalid_schedule = SchedulerError::InvalidSchedule("bad schedule".to_string());
        assert_eq!(
            invalid_schedule.to_string(),
            "Invalid schedule: bad schedule"
        );
    }

    #[tokio::test]
    async fn test_remove_job() {
        let mut scheduler = SchedulerOrchestrator::new();

        let task = Task::new(
            "Test Task".to_string(),
            "A test task".to_string(),
            "DataBackupService".to_string(),
        );

        let job = Job::new("Test Job".to_string(), "A test job".to_string(), vec![task]);

        let job_id = scheduler
            .add_job(job, Schedule::Interval(Duration::from_secs(3600)))
            .unwrap();

        assert_eq!(scheduler.scheduled_jobs.len(), 1);

        // Remove the job
        assert!(scheduler.remove_job(job_id));
        assert_eq!(scheduler.scheduled_jobs.len(), 0);

        // Try to remove again - should return false
        assert!(!scheduler.remove_job(job_id));
    }

    #[tokio::test]
    async fn test_enable_nonexistent_job() {
        let mut scheduler = SchedulerOrchestrator::new();
        let fake_id = Uuid::new_v4();

        assert!(!scheduler.enable_job(fake_id));
    }

    #[tokio::test]
    async fn test_disable_nonexistent_job() {
        let mut scheduler = SchedulerOrchestrator::new();
        let fake_id = Uuid::new_v4();

        assert!(!scheduler.disable_job(fake_id));
    }

    #[tokio::test]
    async fn test_add_job_with_missing_service() {
        let mut scheduler = SchedulerOrchestrator::new();

        let task = Task::new(
            "Test Task".to_string(),
            "A test task".to_string(),
            "NonExistentService".to_string(), // This service doesn't exist
        );

        let job = Job::new("Test Job".to_string(), "A test job".to_string(), vec![task]);

        let result = scheduler.add_job(job, Schedule::Interval(Duration::from_secs(3600)));

        assert!(result.is_err());
        match result {
            Err(SchedulerError::ServiceNotFound(name)) => {
                assert_eq!(name, "NonExistentService");
            }
            _ => panic!("Expected ServiceNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_scheduler_stats() {
        let scheduler = SchedulerOrchestrator::new();
        let stats = scheduler.stats();

        assert_eq!(stats.total_jobs, 0);
        assert_eq!(stats.enabled_jobs, 0);
        assert_eq!(stats.total_runs, 0);
        assert!(stats.total_services > 0); // Should have default services
        assert!(stats.next_job_name.is_none());
        assert!(stats.next_job_time.is_none());
    }

    #[tokio::test]
    async fn test_scheduler_stats_with_jobs() {
        let mut scheduler = SchedulerOrchestrator::new();

        let task = Task::new(
            "Test Task".to_string(),
            "A test task".to_string(),
            "DataBackupService".to_string(),
        );

        let job = Job::new("Test Job".to_string(), "A test job".to_string(), vec![task]);

        scheduler
            .add_job(job, Schedule::Interval(Duration::from_secs(3600)))
            .unwrap();

        let stats = scheduler.stats();
        assert_eq!(stats.total_jobs, 1);
        assert_eq!(stats.enabled_jobs, 1);
        assert!(stats.next_job_name.is_some());
        assert!(stats.next_job_time.is_some());
    }

    #[tokio::test]
    async fn test_scheduled_job_structure() {
        let task = Task::new(
            "Test Task".to_string(),
            "A test task".to_string(),
            "DataBackupService".to_string(),
        );

        let job = Job::new("Test Job".to_string(), "A test job".to_string(), vec![task]);
        let schedule = Schedule::Interval(Duration::from_secs(3600));

        let scheduled_job = ScheduledJob {
            job: job.clone(),
            schedule: schedule.clone(),
            enabled: true,
            next_run: Some(Utc::now()),
            last_run: None,
            run_count: 0,
        };

        assert!(scheduled_job.enabled);
        assert!(scheduled_job.next_run.is_some());
        assert!(scheduled_job.last_run.is_none());
        assert_eq!(scheduled_job.run_count, 0);
    }

    #[tokio::test]
    async fn test_schedule_variants() {
        // Test Interval
        let interval = Schedule::Interval(Duration::from_secs(3600));
        assert!(matches!(interval, Schedule::Interval(_)));

        // Test Daily
        let daily = Schedule::Daily(9, 30); // 9:30 AM
        assert!(matches!(daily, Schedule::Daily(9, 30)));

        // Test Weekly
        let weekly = Schedule::Weekly(1, 10, 0); // Monday 10:00 AM
        assert!(matches!(weekly, Schedule::Weekly(1, 10, 0)));

        // Test Monthly
        let monthly = Schedule::Monthly(15, 14, 30); // 15th of month, 2:30 PM
        assert!(matches!(monthly, Schedule::Monthly(15, 14, 30)));

        // Test Once
        let once_time = Utc::now() + chrono::Duration::hours(1);
        let once = Schedule::Once(once_time);
        assert!(matches!(once, Schedule::Once(_)));

        // Test OnStartup
        let startup = Schedule::OnStartup;
        assert!(matches!(startup, Schedule::OnStartup));
    }

    #[tokio::test]
    async fn test_calculate_next_run_daily() {
        // Daily at 9:00 AM
        let daily_schedule = Schedule::Daily(9, 0);
        let next_run = SchedulerOrchestrator::calculate_next_run(&daily_schedule);
        assert!(next_run.is_some());
    }

    // ───────────────────────── Epic 2 (Task 2.1) validation ─────────────────────────

    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Test double `TaskService` that records how many times it is executed.
    #[derive(Debug, Clone)]
    struct RecordingService {
        service_name: String,
        calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl TaskService for RecordingService {
        fn name(&self) -> &str {
            &self.service_name
        }

        async fn execute(
            &self,
            _action: &crate::core::base::component::action::Action,
        ) -> Result<Option<serde_json::Value>, crate::core::platform::container::task::TaskError>
        {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(Some(serde_json::json!({ "executed": true })))
        }

        fn clone_service(&self) -> Box<dyn TaskService> {
            Box::new(self.clone())
        }
    }

    /// Build a scheduler whose only service is a `RecordingService`, returning the
    /// scheduler and a shared call counter.
    fn scheduler_with_recording_service() -> (SchedulerOrchestrator, Arc<AtomicUsize>) {
        let calls = Arc::new(AtomicUsize::new(0));
        let mut scheduler = SchedulerOrchestrator::new();
        scheduler.services.clear();
        scheduler.register_service(Box::new(RecordingService {
            service_name: "RecordingService".to_string(),
            calls: calls.clone(),
        }));
        (scheduler, calls)
    }

    fn recording_job() -> Job {
        let task = Task::new(
            "Recording Task".to_string(),
            "Records execution".to_string(),
            "RecordingService".to_string(),
        );
        Job::new(
            "Recording Job".to_string(),
            "A job that records execution".to_string(),
            vec![task],
        )
    }

    #[tokio::test]
    async fn test_calculate_next_run_all_variants() {
        // Interval → some
        assert!(
            SchedulerOrchestrator::calculate_next_run(&Schedule::Interval(Duration::from_secs(60)))
                .is_some()
        );
        // Daily → some
        assert!(SchedulerOrchestrator::calculate_next_run(&Schedule::Daily(9, 0)).is_some());
        // Weekly → some
        assert!(SchedulerOrchestrator::calculate_next_run(&Schedule::Weekly(1, 10, 0)).is_some());
        // Monthly → some
        assert!(
            SchedulerOrchestrator::calculate_next_run(&Schedule::Monthly(15, 14, 30)).is_some()
        );
        // Once in the future → Some(exact)
        let future = Utc::now() + chrono::Duration::hours(1);
        assert_eq!(
            SchedulerOrchestrator::calculate_next_run(&Schedule::Once(future)),
            Some(future)
        );
        // Once in the past → None
        let past = Utc::now() - chrono::Duration::hours(1);
        assert_eq!(
            SchedulerOrchestrator::calculate_next_run(&Schedule::Once(past)),
            None
        );
        // OnStartup → None
        assert_eq!(
            SchedulerOrchestrator::calculate_next_run(&Schedule::OnStartup),
            None
        );
    }

    #[tokio::test]
    async fn test_tick_dispatches_due_job_and_updates_bookkeeping() {
        let (mut scheduler, calls) = scheduler_with_recording_service();
        let job_id = scheduler
            .add_job(
                recording_job(),
                Schedule::Interval(Duration::from_secs(3600)),
            )
            .unwrap();

        // Force the job to be due in the past so a single tick dispatches it.
        scheduler.scheduled_jobs.get_mut(&job_id).unwrap().next_run =
            Some(Utc::now() - chrono::Duration::seconds(1));

        scheduler.check_and_execute_jobs().await;

        assert_eq!(calls.load(Ordering::SeqCst), 1, "service should run once");
        let sj = scheduler.scheduled_jobs.get(&job_id).unwrap();
        assert!(sj.last_run.is_some(), "last_run should be set");
        assert_eq!(sj.run_count, 1, "run_count should increment");
        assert!(
            sj.next_run.is_some(),
            "recurring job should have a recomputed next_run"
        );
    }

    #[tokio::test]
    async fn test_tick_skips_disabled_job() {
        let (mut scheduler, calls) = scheduler_with_recording_service();
        let job_id = scheduler
            .add_job(
                recording_job(),
                Schedule::Interval(Duration::from_secs(3600)),
            )
            .unwrap();

        // Make the job due but disabled — it must be skipped.
        {
            let sj = scheduler.scheduled_jobs.get_mut(&job_id).unwrap();
            sj.enabled = false;
            sj.next_run = Some(Utc::now() - chrono::Duration::seconds(1));
        }

        scheduler.check_and_execute_jobs().await;

        assert_eq!(
            calls.load(Ordering::SeqCst),
            0,
            "disabled job must not execute"
        );
        let sj = scheduler.scheduled_jobs.get(&job_id).unwrap();
        assert_eq!(sj.run_count, 0, "disabled job must not advance run_count");
    }

    #[tokio::test]
    async fn test_once_job_fires_once_then_not_again() {
        let (mut scheduler, calls) = scheduler_with_recording_service();
        let future = Utc::now() + chrono::Duration::hours(1);
        let job_id = scheduler
            .add_job(recording_job(), Schedule::Once(future))
            .unwrap();

        // Force it due now.
        scheduler.scheduled_jobs.get_mut(&job_id).unwrap().next_run =
            Some(Utc::now() - chrono::Duration::seconds(1));

        // First tick fires it once.
        scheduler.check_and_execute_jobs().await;
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        let sj = scheduler.scheduled_jobs.get(&job_id).unwrap();
        assert_eq!(sj.run_count, 1);
        assert!(
            sj.next_run.is_none(),
            "Once job must not have a next_run after firing"
        );

        // Second tick must not fire it again.
        scheduler.check_and_execute_jobs().await;
        assert_eq!(
            calls.load(Ordering::SeqCst),
            1,
            "Once job must fire exactly once"
        );
        assert_eq!(scheduler.scheduled_jobs.get(&job_id).unwrap().run_count, 1);
    }

    #[tokio::test]
    async fn test_scheduler_fires_job_scheduled_in_near_future() {
        let (mut scheduler, calls) = scheduler_with_recording_service();
        // Drive ticks quickly so the test stays fast and deterministic.
        scheduler.tick_interval = Duration::from_millis(25);

        let job_id = scheduler
            .add_job(
                recording_job(),
                Schedule::Interval(Duration::from_secs(3600)),
            )
            .unwrap();
        // Due shortly after start.
        scheduler.scheduled_jobs.get_mut(&job_id).unwrap().next_run =
            Some(Utc::now() + chrono::Duration::milliseconds(150));

        tokio::spawn(async move {
            scheduler.start().await;
        });

        // Poll the shared counter with a generous bound to avoid flakiness.
        let mut fired = false;
        for _ in 0..80 {
            tokio::time::sleep(Duration::from_millis(50)).await;
            if calls.load(Ordering::SeqCst) >= 1 {
                fired = true;
                break;
            }
        }
        assert!(
            fired,
            "scheduled job did not fire within the timeout window"
        );
    }
}
