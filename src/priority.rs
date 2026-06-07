//! Priority-based CPU scheduling.

use crate::gantt::GanttChart;
use crate::{CpuScheduler, Process};

/// Priority scheduler — lower number = higher priority.
#[derive(Debug)]
pub struct PriorityScheduler {
    pub preemptive: bool,
}

impl PriorityScheduler {
    pub fn new(preemptive: bool) -> Self {
        Self { preemptive }
    }

    pub fn non_preemptive() -> Self {
        Self::new(false)
    }

    pub fn preemptive() -> Self {
        Self::new(true)
    }
}

impl Default for PriorityScheduler {
    fn default() -> Self {
        Self::non_preemptive()
    }
}

impl CpuScheduler for PriorityScheduler {
    fn schedule(&self, processes: &[Process]) -> GanttChart {
        if processes.is_empty() {
            return GanttChart::new();
        }

        let mut procs: Vec<Process> = processes.to_vec();
        for p in &mut procs {
            p.reset_remaining();
        }

        let mut chart = GanttChart::new();
        let mut current_time: u32 = 0;
        let total_burst: u32 = procs.iter().map(|p| p.burst_time).sum();

        while chart.total_time() < total_burst {
            // Find highest priority available process
            let available: Vec<&Process> = procs
                .iter()
                .filter(|p| p.arrival_time <= current_time && p.remaining_time > 0)
                .collect();

            if available.is_empty() {
                current_time += 1;
                continue;
            }

            let chosen = available
                .iter()
                .min_by(|a, b| {
                    a.priority.cmp(&b.priority)
                        .then(a.arrival_time.cmp(&b.arrival_time))
                        .then(a.pid.cmp(&b.pid))
                })
                .unwrap();

            let pid = chosen.pid;

            if self.preemptive {
                // Run for 1 unit
                for p in &mut procs {
                    if p.pid == pid {
                        p.remaining_time -= 1;
                        break;
                    }
                }
                // Merge with previous segment if same process
                if let Some(last) = chart.segments.last_mut() {
                    if last.pid == pid && last.end == current_time {
                        last.end = current_time + 1;
                    } else {
                        chart.add(pid, current_time, current_time + 1);
                    }
                } else {
                    chart.add(pid, current_time, current_time + 1);
                }
                current_time += 1;
            } else {
                // Non-preemptive: run to completion
                let burst = chosen.remaining_time;
                for p in &mut procs {
                    if p.pid == pid {
                        p.remaining_time = 0;
                        break;
                    }
                }
                chart.add(pid, current_time, current_time + burst);
                current_time += burst;
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
        let chart = PriorityScheduler::default().schedule(&[]);
        assert_eq!(chart.segments.len(), 0);
    }

    #[test]
    fn non_preemptive_basic() {
        let procs = vec![
            Process::with_priority(1, 0, 6, 2),
            Process::with_priority(2, 0, 4, 1),
            Process::with_priority(3, 0, 2, 3),
        ];
        let chart = PriorityScheduler::non_preemptive().schedule(&procs);
        // P2 (pri=1) first, then P1 (pri=2), then P3 (pri=3)
        assert_eq!(chart.segments[0].pid, 2);
        assert_eq!(chart.segments[1].pid, 1);
        assert_eq!(chart.segments[2].pid, 3);
    }

    #[test]
    fn preemptive_basic() {
        let procs = vec![
            Process::with_priority(1, 0, 4, 2),
            Process::with_priority(2, 2, 3, 1), // arrives at 2, higher priority
        ];
        let chart = PriorityScheduler::preemptive().schedule(&procs);
        // P1 runs 0-2, then P2 (higher pri) preempts: 2-5, then P1 resumes: 5-7
        assert_eq!(chart.segments[0].pid, 1);
        assert_eq!(chart.segments[0].end, 2);
    }

    #[test]
    fn single_process() {
        let chart = PriorityScheduler::default().schedule(&[
            Process::with_priority(1, 0, 5, 1),
        ]);
        assert_eq!(chart.total_time(), 5);
    }

    #[test]
    fn same_priority_fcfs() {
        let procs = vec![
            Process::with_priority(1, 0, 3, 1),
            Process::with_priority(2, 0, 3, 1),
        ];
        let chart = PriorityScheduler::non_preemptive().schedule(&procs);
        assert_eq!(chart.segments[0].pid, 1);
        assert_eq!(chart.segments[1].pid, 2);
    }
}
