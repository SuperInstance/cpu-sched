//! Gantt chart and scheduling metrics.

/// A single segment in a Gantt chart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GanttSegment {
    pub pid: u32,
    pub start: u32,
    pub end: u32,
}

/// A Gantt chart representing a CPU schedule.
#[derive(Debug, Clone)]
pub struct GanttChart {
    pub segments: Vec<GanttSegment>,
}

impl GanttChart {
    pub fn new() -> Self {
        Self { segments: vec![] }
    }

    /// Add a segment.
    pub fn add(&mut self, pid: u32, start: u32, end: u32) {
        self.segments.push(GanttSegment { pid, start, end });
    }

    /// Total time of the schedule.
    pub fn total_time(&self) -> u32 {
        self.segments.last().map(|s| s.end).unwrap_or(0)
    }

    /// Display as a text Gantt chart.
    pub fn display(&self) -> String {
        let mut result = String::new();
        for seg in &self.segments {
            result.push_str(&format!(
                "| P{} [{}-{}] ",
                seg.pid, seg.start, seg.end
            ));
        }
        result.push('|');
        result
    }

    /// Calculate metrics: turnaround time, waiting time per process.
    pub fn metrics(&self) -> ScheduleMetrics {
        let mut completion_times: std::collections::HashMap<u32, u32> =
            std::collections::HashMap::new();
        let mut first_run: std::collections::HashMap<u32, u32> =
            std::collections::HashMap::new();

        for seg in &self.segments {
            completion_times.insert(seg.pid, seg.end);
            first_run.entry(seg.pid).or_insert(seg.start);
        }

        ScheduleMetrics {
            completion_times,
            first_run,
        }
    }
}

impl Default for GanttChart {
    fn default() -> Self {
        Self::new()
    }
}

/// Scheduling metrics.
#[derive(Debug)]
pub struct ScheduleMetrics {
    pub completion_times: std::collections::HashMap<u32, u32>,
    pub first_run: std::collections::HashMap<u32, u32>,
}

impl ScheduleMetrics {
    /// Turnaround time = completion - arrival.
    pub fn turnaround(&self, pid: u32, arrival: u32) -> Option<u32> {
        self.completion_times.get(&pid).map(|&c| c - arrival)
    }

    /// Waiting time = turnaround - burst.
    pub fn waiting(&self, pid: u32, arrival: u32, burst: u32) -> Option<u32> {
        self.turnaround(pid, arrival).map(|t| t - burst)
    }

    /// Response time = first_run - arrival.
    pub fn response(&self, pid: u32, arrival: u32) -> Option<u32> {
        self.first_run.get(&pid).map(|&s| s - arrival)
    }

    /// Average turnaround time.
    pub fn avg_turnaround(&self, arrivals: &[(u32, u32)]) -> f64 {
        let total: u32 = arrivals
            .iter()
            .filter_map(|&(pid, arr)| self.turnaround(pid, arr))
            .sum();
        let count = arrivals.len();
        if count == 0 { 0.0 } else { total as f64 / count as f64 }
    }

    /// Average waiting time.
    pub fn avg_waiting(&self, processes: &[(u32, u32, u32)]) -> f64 {
        let total: u32 = processes
            .iter()
            .filter_map(|&(pid, arr, burst)| self.waiting(pid, arr, burst))
            .sum();
        let count = processes.len();
        if count == 0 { 0.0 } else { total as f64 / count as f64 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_chart() {
        let g = GanttChart::new();
        assert_eq!(g.total_time(), 0);
        assert_eq!(g.display(), "|");
    }

    #[test]
    fn single_segment() {
        let mut g = GanttChart::new();
        g.add(1, 0, 6);
        assert_eq!(g.total_time(), 6);
    }

    #[test]
    fn metrics_basic() {
        let mut g = GanttChart::new();
        g.add(1, 0, 6);
        g.add(2, 6, 14);
        let m = g.metrics();
        assert_eq!(m.turnaround(1, 0), Some(6));
        assert_eq!(m.turnaround(2, 1), Some(13));
        assert_eq!(m.waiting(1, 0, 6), Some(0));
        assert_eq!(m.waiting(2, 1, 8), Some(5));
    }

    #[test]
    fn avg_metrics() {
        let mut g = GanttChart::new();
        g.add(1, 0, 6);
        g.add(2, 6, 14);
        let m = g.metrics();
        let avg_t = m.avg_turnaround(&[(1, 0), (2, 1)]);
        assert!((avg_t - 9.5).abs() < f64::EPSILON);
    }

    #[test]
    fn display_format() {
        let mut g = GanttChart::new();
        g.add(1, 0, 3);
        g.add(2, 3, 5);
        let s = g.display();
        assert!(s.contains("P1"));
        assert!(s.contains("P2"));
    }
}
