//! Scheduling adapters for the infrastructure layer.
//!
//! Provides concrete implementations of the [`SchedulerPort`] trait using
//! external scheduling libraries.
//!
//! [`SchedulerPort`]: paladin_ports::output::scheduler_port::SchedulerPort

pub mod tokio_cron_adapter;
