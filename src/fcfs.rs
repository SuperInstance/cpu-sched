//! First Come, First Served CPU scheduling.

use crate::gantt::GanttChart;
use crate::{CpuScheduler, Process};

/// FCFS scheduler — non-preemptive, processes run in arrival order.
#[derive(Debug, Default)]
pub struct FcfsScheduler;

impl CpuScheduler for FcfsScheduler {
    fn schedule(&self, processes: &[Process]) -> GanttChart {
        let mut procs: Vec<&Process> = processes.iter().collect();
        procs.sort_by_key(|p| (p.arrival_time, p.pid));

        let mut chart = GanttChart::new();
        let mut current_time: u32 = 0;

        for proc in &procs {
            if current_time < proc.arrival_time {
                current_time = proc.arrival_time;
            }
            chart.add(proc.pid, current_time, current_time + proc.burst_time);
            current_time += proc.burst_time;
        }

        chart
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CpuScheduler;

    #[test]
    fn empty_processes() {
        let chart = FcfsScheduler.schedule(&[]);
        assert_eq!(chart.segments.len(), 0);
    }

    #[test]
    fn single_process() {
        let chart = FcfsScheduler.schedule(&[Process::new(1, 0, 6)]);
        assert_eq!(chart.segments.len(), 1);
        assert_eq!(chart.total_time(), 6);
    }

    #[test]
    fn textbook_example() {
        // P1: arr=0, burst=6; P2: arr=1, burst=8; P3: arr=2, burst=7
        let procs = vec![
            Process::new(1, 0, 6),
            Process::new(2, 1, 8),
            Process::new(3, 2, 7),
        ];
        let chart = FcfsScheduler.schedule(&procs);
        assert_eq!(chart.segments[0], crate::gantt::GanttSegment { pid: 1, start: 0, end: 6 });
        assert_eq!(chart.segments[1], crate::gantt::GanttSegment { pid: 2, start: 6, end: 14 });
        assert_eq!(chart.segments[2], crate::gantt::GanttSegment { pid: 3, start: 14, end: 21 });
    }

    #[test]
    fn waiting_times() {
        let procs = vec![
            Process::new(1, 0, 6),
            Process::new(2, 1, 8),
            Process::new(3, 2, 7),
        ];
        let chart = FcfsScheduler.schedule(&procs);
        let m = chart.metrics();
        assert_eq!(m.waiting(1, 0, 6), Some(0));
        assert_eq!(m.waiting(2, 1, 8), Some(5));
        assert_eq!(m.waiting(3, 2, 7), Some(12));
    }

    #[test]
    fn gap_in_arrivals() {
        let procs = vec![
            Process::new(1, 0, 3),
            Process::new(2, 10, 5),
        ];
        let chart = FcfsScheduler.schedule(&procs);
        assert_eq!(chart.segments[1].start, 10);
    }
}
