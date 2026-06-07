//! CPU scheduling algorithms.
//!
//! Provides FCFS, SJF, Round Robin, Priority, and Multilevel Queue
//! scheduling with Gantt chart output.

pub mod fcfs;
pub mod sjf;
pub mod round_robin;
pub mod priority;
pub mod gantt;

pub use gantt::{GanttChart, ScheduleMetrics};

/// A process for scheduling.
#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    pub arrival_time: u32,
    pub burst_time: u32,
    pub priority: u32,
    pub remaining_time: u32,
}

impl Process {
    pub fn new(pid: u32, arrival_time: u32, burst_time: u32) -> Self {
        Self {
            pid,
            arrival_time,
            burst_time,
            priority: 0,
            remaining_time: burst_time,
        }
    }

    pub fn with_priority(pid: u32, arrival_time: u32, burst_time: u32, priority: u32) -> Self {
        Self {
            pid,
            arrival_time,
            burst_time,
            priority,
            remaining_time: burst_time,
        }
    }

    pub fn reset_remaining(&mut self) {
        self.remaining_time = self.burst_time;
    }
}

/// Trait for CPU schedulers.
pub trait CpuScheduler {
    /// Schedule the given processes and return a Gantt chart.
    fn schedule(&self, processes: &[Process]) -> GanttChart;
}
