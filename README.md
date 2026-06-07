# cpu-sched

CPU scheduling algorithm simulator for research and education.

Implements five classic CPU scheduling strategies with Gantt chart output:

- **FCFS** — First Come, First Served
- **SJF** — Shortest Job First (non-preemptive)
- **Round Robin** — Time quantum based preemption
- **Priority** — Priority-based scheduling (preemptive and non-preemptive)
- **Multilevel Queue** — Multiple priority queues

## Usage

```rust
use cpu_sched::{fcfs::FcfsScheduler, gantt::GanttChart};
use cpu_sched::Process;

let processes = vec![
    Process::new(1, 0, 6),
    Process::new(2, 1, 8),
    Process::new(3, 2, 7),
];

let scheduler = FcfsScheduler;
let gantt = scheduler.schedule(&processes);
println!("{}", gantt.display());

let metrics = gantt.metrics();
println!("Avg turnaround: {:.2}", metrics.avg_turnaround());
println!("Avg waiting: {:.2}", metrics.avg_waiting());
```

## Gantt Chart

The `gantt` module produces:
- Visual Gantt chart display
- Per-process turnaround and waiting times
- Averages across all processes

No external dependencies — pure `std`.

## License

MIT
