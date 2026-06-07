//! Round Robin CPU scheduling.

use crate::gantt::GanttChart;
use crate::{CpuScheduler, Process};
use std::collections::VecDeque;

/// Round Robin scheduler with configurable time quantum.
#[derive(Debug)]
pub struct RoundRobinScheduler {
    pub quantum: u32,
}

impl RoundRobinScheduler {
    pub fn new(quantum: u32) -> Self {
        Self { quantum }
    }
}

impl Default for RoundRobinScheduler {
    fn default() -> Self {
        Self::new(2)
    }
}

impl CpuScheduler for RoundRobinScheduler {
    fn schedule(&self, processes: &[Process]) -> GanttChart {
        if processes.is_empty() {
            return GanttChart::new();
        }

        let mut procs: Vec<Process> = processes.to_vec();
        procs.sort_by_key(|p| (p.arrival_time, p.pid));

        // Reset remaining time
        for p in &mut procs {
            p.reset_remaining();
        }

        let mut chart = GanttChart::new();
        let mut ready_queue: VecDeque<usize> = VecDeque::new();
        let mut current_time: u32 = 0;
        let mut next_to_enqueue: usize = 0;
        let n = procs.len();

        // Enqueue initial arrivals
        while next_to_enqueue < n && procs[next_to_enqueue].arrival_time <= current_time {
            ready_queue.push_back(next_to_enqueue);
            next_to_enqueue += 1;
        }

        while !ready_queue.is_empty() || next_to_enqueue < n {
            if ready_queue.is_empty() {
                // Jump to next arrival
                current_time = procs[next_to_enqueue].arrival_time;
                while next_to_enqueue < n && procs[next_to_enqueue].arrival_time <= current_time {
                    ready_queue.push_back(next_to_enqueue);
                    next_to_enqueue += 1;
                }
            }

            let idx = ready_queue.pop_front().unwrap();
            let exec_time = self.quantum.min(procs[idx].remaining_time);

            chart.add(procs[idx].pid, current_time, current_time + exec_time);
            current_time += exec_time;
            procs[idx].remaining_time -= exec_time;

            // Enqueue new arrivals
            while next_to_enqueue < n && procs[next_to_enqueue].arrival_time <= current_time {
                ready_queue.push_back(next_to_enqueue);
                next_to_enqueue += 1;
            }

            // If not finished, put back in queue
            if procs[idx].remaining_time > 0 {
                ready_queue.push_back(idx);
            }
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
        let chart = RoundRobinScheduler::new(2).schedule(&[]);
        assert_eq!(chart.segments.len(), 0);
    }

    #[test]
    fn single_no_preempt() {
        let chart = RoundRobinScheduler::new(4).schedule(&[Process::new(1, 0, 3)]);
        assert_eq!(chart.segments.len(), 1);
        assert_eq!(chart.total_time(), 3);
    }

    #[test]
    fn round_robin_basic() {
        // P1: burst=4, P2: burst=3, quantum=2
        let procs = vec![
            Process::new(1, 0, 4),
            Process::new(2, 0, 3),
        ];
        let chart = RoundRobinScheduler::new(2).schedule(&procs);
        // P1(0-2), P2(2-4), P1(4-6), P2(6-7)
        assert_eq!(chart.segments[0], crate::gantt::GanttSegment { pid: 1, start: 0, end: 2 });
        assert_eq!(chart.segments[1], crate::gantt::GanttSegment { pid: 2, start: 2, end: 4 });
        assert_eq!(chart.segments[2], crate::gantt::GanttSegment { pid: 1, start: 4, end: 6 });
        assert_eq!(chart.segments[3], crate::gantt::GanttSegment { pid: 2, start: 6, end: 7 });
    }

    #[test]
    fn staggered_arrivals() {
        let procs = vec![
            Process::new(1, 0, 4),
            Process::new(2, 3, 3),
        ];
        let chart = RoundRobinScheduler::new(2).schedule(&procs);
        assert_eq!(chart.segments[0].start, 0);
        assert!(chart.segments.iter().any(|s| s.pid == 2));
    }

    #[test]
    fn metrics() {
        let procs = vec![
            Process::new(1, 0, 4),
            Process::new(2, 0, 3),
        ];
        let chart = RoundRobinScheduler::new(2).schedule(&procs);
        let m = chart.metrics();
        // P2 completes at 7, arrival=0 → turnaround=7
        assert_eq!(m.turnaround(2, 0), Some(7));
    }
}
