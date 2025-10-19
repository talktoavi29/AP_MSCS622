# scheduler-python/main.py
from __future__ import annotations
import json, random, csv, os
from typing import Dict, List, Tuple

# ----------------- CONFIG -----------------
DAYS = ["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"]
SHIFTS = ["morning","afternoon","evening"]
MIN_PER_SHIFT = 2
MAX_PER_SHIFT = 3              # capacity that can trigger conflicts
MAX_DAYS_PER_EMPLOYEE = 5
RANDOM_SEED = 42               # set to None for non-deterministic
# -----------------------------------------

VALID_TOKENS = {
    "morning":"morning", "m":"morning",
    "afternoon":"afternoon", "a":"afternoon",
    "evening":"evening", "e":"evening",
    "off":"off", "o":"off",
}

def ask(prompt: str) -> str:
    try:
        return input(prompt)
    except EOFError:
        return ""

def parse_ranking(raw: str) -> List[str] | None:
    s = raw.strip().lower()
    if not s:
        return ["off"]
    # support separators
    for sep in [",","/","|"]:
        s = s.replace(sep, ">")
    parts = [p.strip() for p in s.split(">") if p.strip()]
    out: List[str] = []
    for p in parts:
        if p in VALID_TOKENS:
            out.append(VALID_TOKENS[p])
        else:
            return None  # invalid token -> reject
    # uniq and expand
    uniq: List[str] = []
    for v in out:
        if v not in uniq:
            uniq.append(v)
    if not uniq or "off" in uniq:
        return ["off"]
    # fill missing in default order
    for sh in SHIFTS:
        if sh not in uniq:
            uniq.append(sh)
    return uniq

def collect_preferences() -> Dict[str, Dict[str, List[str]]]:
    print("Enter employee names separated by commas (e.g., Alice,Bob,Charlie)")
    raw_names = ask("> ").strip()
    while not raw_names:
        print("Please enter at least one name.")
        raw_names = ask("> ").strip()
    names = [n.strip() for n in raw_names.split(",") if n.strip()]
    prefs: Dict[str, Dict[str, List[str]]] = {name: {} for name in names}
    print("\nEnter a preferred shift *per day* for each employee.")
    print("Accepted: morning/afternoon/evening/off or ranking like 'm>a>e'. Blank = off.\n")
    for name in names:
        print(f"\n-- {name} --")
        for day in DAYS:
            while True:
                raw = ask(f"{day} preference: ")
                ranked = parse_ranking(raw)
                if ranked is not None:
                    prefs[name][day] = ranked
                    break
                print("  Invalid. Use morning/afternoon/evening/off or ranking like m>a>e.")
    return prefs

def try_assign(day: str, shift: str, name: str,
               schedule, assigned_days_count, daily_assigned) -> bool:
    """Try assigning (day, shift) if capacity & per-day constraints allow."""
    if name in daily_assigned[day]:
        return False
    if assigned_days_count[name] >= MAX_DAYS_PER_EMPLOYEE:
        return False
    if len(schedule[day][shift]) >= MAX_PER_SHIFT:
        return False
    schedule[day][shift].append(name)
    daily_assigned[day].add(name)
    assigned_days_count[name] += 1
    return True

def schedule_week(preferences: Dict[str, Dict[str, List[str]]],
                  seed: int | None = RANDOM_SEED
) -> Tuple[Dict[str, Dict[str, List[str]]], Dict[str,int]]:
    rng = random.Random(seed)
    employees = list(preferences.keys())

    # Initialize structures
    schedule = {day: {shift: [] for shift in SHIFTS} for day in DAYS}
    assigned_days_count = {name: 0 for name in employees}
    # track who is assigned on a given day
    daily_assigned = {day: set() for day in DAYS}

    # Deferred (couldn't place today): map of day_index+1 -> list of employees
    deferred_next_day = {d: [] for d in range(len(DAYS))}

    # iterate days
    for di, day in enumerate(DAYS):
        # 1) bring in deferrals from previous day
        incoming = deferred_next_day.get(di, [])
        rng.shuffle(incoming)

        # 2) build candidate order for today: incoming first (fairness), then others
        todays_names = [n for n in employees if n not in incoming]
        rng.shuffle(todays_names)
        ordered = incoming + todays_names

        # 3) ranking-aware fill: pass for rank 1, then 2, then 3
        for rank in range(3):
            for name in ordered:
                if name in daily_assigned[day]:        # already got a shift today
                    continue
                if assigned_days_count[name] >= MAX_DAYS_PER_EMPLOYEE:
                    continue
                ranking = preferences.get(name, {}).get(day, ["off"])
                if ranking == ["off"]:
                    continue
                preferred = ranking[rank]  # will exist because we pad to 3
                # try preferred first
                if try_assign(day, preferred, name, schedule, assigned_days_count, daily_assigned):
                    continue
                # conflict: try other shifts same day
                for alt in SHIFTS:
                    if alt == preferred:
                        continue
                    if try_assign(day, alt, name, schedule, assigned_days_count, daily_assigned):
                        break
                else:
                    # still unassigned at this rank; we'll try next rank round
                    pass

        # 4) ensure minimum staffing per shift by random fill among available
        for shift in SHIFTS:
            while len(schedule[day][shift]) < MIN_PER_SHIFT:
                candidates = [
                    n for n in employees
                    if (n not in daily_assigned[day]
                        and assigned_days_count[n] < MAX_DAYS_PER_EMPLOYEE
                        and preferences.get(n, {}).get(day, ["off"]) != ["off"])
                ]
                if not candidates:
                    candidates = [
                        n for n in employees
                        if (n not in daily_assigned[day]
                            and assigned_days_count[n] < MAX_DAYS_PER_EMPLOYEE)
                    ]
                if not candidates:
                    break
                pick = rng.choice(candidates)
                try_assign(day, shift, pick, schedule, assigned_days_count, daily_assigned)

        # 5) Any still-unassigned with non-off preferences? Defer to next day
        if di + 1 < len(DAYS):
            for name in employees:
                if (name not in daily_assigned[day]
                    and assigned_days_count[name] < MAX_DAYS_PER_EMPLOYEE
                    and preferences.get(name, {}).get(day, ["off"]) != ["off"]):
                    # defer; they will be considered first tomorrow
                    deferred_next_day[di + 1].append(name)

    return schedule, assigned_days_count

def print_schedule(schedule: Dict[str, Dict[str, List[str]]], totals: Dict[str,int]):
    print("\n================ WEEKLY SCHEDULE ================\n")
    for day in DAYS:
        print(day)
        for shift in SHIFTS:
            names = schedule[day][shift]
            show = ", ".join(names) if names else "-"
            print(f"  {shift:<9}: {show}")
        print()
    print("Totals (days assigned per employee, max 5):")
    for name, cnt in sorted(totals.items()):
        print(f"  {name:<15} {cnt}")
    print()

def write_csv(schedule, path="schedule.csv"):
    with open(path, "w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow(["Day","Shift","Employees"])
        for day in DAYS:
            for shift in SHIFTS:
                w.writerow([day, shift, "; ".join(schedule[day][shift])])

def write_md(schedule, path="schedule.md"):
    with open(path, "w", encoding="utf-8") as f:
        f.write("| Day | Morning | Afternoon | Evening |\n")
        f.write("|---|---|---|---|\n")
        for day in DAYS:
            row = [day]
            for shift in SHIFTS:
                row.append(", ".join(schedule[day][shift]) if schedule[day][shift] else "-")
            f.write("| " + " | ".join(row) + " |\n")

def write_png(schedule, path="schedule.png"):
    try:
        import matplotlib.pyplot as plt
        # Build a table-like image
        cell_text = []
        for day in DAYS:
            row = []
            for shift in SHIFTS:
                row.append("\n".join(schedule[day][shift]) if schedule[day][shift] else "-")
            cell_text.append(row)
        fig, ax = plt.subplots(figsize=(10, 6))
        ax.axis("off")
        tbl = ax.table(
            cellText=cell_text,
            rowLabels=DAYS,
            colLabels=[s.title() for s in SHIFTS],
            loc="center",
        )
        tbl.scale(1, 2)
        plt.tight_layout()
        fig.savefig(path, dpi=200, bbox_inches="tight")
        plt.close(fig)
        print(f"Saved {path}")
    except Exception as e:
        print(f"(Skipping PNG export) {e}")

def main():
    print("== Employee Scheduler (Python) ==")
    # Collect
    prefs = collect_preferences()

    # Capacity sanity check
    cap = len(prefs) * MAX_DAYS_PER_EMPLOYEE
    need = len(DAYS) * len(SHIFTS) * MIN_PER_SHIFT
    if cap < need:
        print(f"\n[Warning] Weekly capacity {cap} < required minimum {need}. "
              f"Some shifts may be under-staffed.\n")

    # Schedule
    schedule, totals = schedule_week(prefs, seed=RANDOM_SEED)

    # Output
    print_schedule(schedule, totals)
    with open("preferences.json","w", encoding="utf-8") as f:
        json.dump(prefs, f, indent=2)
    with open("schedule.json","w", encoding="utf-8") as f:
        json.dump(schedule, f, indent=2)
    write_csv(schedule, "schedule.csv")
    write_md(schedule, "schedule.md")
    write_png(schedule, "schedule.png")

    print("Wrote: preferences.json, schedule.json, schedule.csv, schedule.md, schedule.png (if matplotlib available)")

if __name__ == "__main__":
    main()