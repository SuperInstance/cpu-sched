//! Shortest Job First (non-preemptive) CPU scheduling.

use crate::gantt::GanttChart;
use crate::{CpuScheduler, Process};

/// SJF scheduler — non-preemptive, picks shortest burst at each decision point.
#[derive(Debug, Default)]
pub struct SjfScheduler;

impl CpuScheduler for SjfScheduler {
    fn schedule(&self, processes: &[Process]) -> GanttChart {
        let mut remaining: Vec<Process> = processes.to_vec();
        let mut chart = GanttChart::new();
        let mut current_time: u32 = 0;

        while !remaining.is_empty() {
            // Find available processes (arrived by current_time)
            let mut available: Vec<&Process> = remaining
                .iter()
                .filter(|p| p.arrival_time <= current_time)
                .collect();

            if available.is_empty() {
                // Jump to next arrival
                let next_arrival = remaining.iter().map(|p| p.arrival_time).min().unwrap();
                current_time = next_arrival;
                continue;
            }

            // Pick shortest burst time (ties by arrival, then pid)
            available.sort_by(|a, b| {
                a.burst_time.cmp(&b.burst_time)
                    .then(a.arrival_time.cmp(&b.arrival_time))
                    .then(a.pid.cmp(&b.pid))
            });

            let chosen = available[0];
            let pid = chosen.pid;
            let burst = chosen.burst_time;

            chart.add(pid, current_time, current_time + burst);
            current_time += burst;

            remaining.retain(|p| p.pid != pid);
        }

        chart
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CpuScheduler;

    #[test]
    fn empty() {
        let chart = SjfScheduler.schedule(&[]);
        assert_eq!(chart.segments.len(), 0);
    }

    #[test]
    fn single() {
        let chart = SjfScheduler.schedule(&[Process::new(1, 0, 5)]);
        assert_eq!(chart.total_time(), 5);
    }

    #[test]
    fn picks_shortest() {
        // All arrive at 0: P1(burst=8), P2(burst=4), P3(burst=2)
        let procs = vec![
            Process::new(1, 0, 8),
            Process::new(2, 0, 4),
            Process::new(3, 0, 2),
        ];
        let chart = SjfScheduler.schedule(&procs);
        // Order: P3(2), P2(4), P1(8)
        assert_eq!(chart.segments[0].pid, 3);
        assert_eq!(chart.segments[1].pid, 2);
        assert_eq!(chart.segments[2].pid, 1);
    }

    #[test]
    fn arrival_matters() {
        // P1: arr=0,burst=6; P2: arr=2,burst=2
        // At t=0, only P1 available → runs P1 first
        let procs = vec![
            Process::new(1, 0, 6),
            Process::new(2, 2, 2),
        ];
        let chart = SjfScheduler.schedule(&procs);
        assert_eq!(chart.segments[0].pid, 1);
        assert_eq!(chart.segments[1].pid, 2);
    }

    #[test]
    fn metrics() {
        let procs = vec![
            Process::new(1, 0, 8),
            Process::new(2, 0, 4),
            Process::new(3, 0, 2),
        ];
        let chart = SjfScheduler.schedule(&procs);
        let m = chart.metrics();
        // P3: completes at 2, turnaround=2, waiting=0
        assert_eq!(m.turnaround(3, 0), Some(2));
        // P2: completes at 6, turnaround=6, waiting=2
        assert_eq!(m.waiting(2, 0, 4), Some(2));
    }
}
