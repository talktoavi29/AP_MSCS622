# Employee Scheduler 

Employee Scheduler in **Python** and **Rust** that build a weekly employee schedule while demonstrating core control structures (conditionals, loops, branching).

Implements:

1. **Input & Storage** – collect names and per-day shift preferences (optionally ranked `m>a>e`)
2. **Scheduling Logic** – one shift per person per day; **max 5 days/week**; **≥ 2 employees per shift/day**
3. **Shift Conflicts** – if a preferred shift is full, try another shift **same day**; otherwise **defer to next day**
4. **Output** – readable weekly schedule to console (with optional CSV/Markdown export)
5. **Bonus (optional)** – ranking support (first → second → third choice)

## Requirements

- **Python 3.9+**
- **Rust (stable)** via `rustup`  

## Files

- **Python:** `main.py`
- **Rust:** `main.rs`