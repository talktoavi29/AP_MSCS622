import random

DAYS = ["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"]
SHIFTS = ["morning","afternoon","evening"]

MIN_PER_SHIFT = 2
MAX_PER_SHIFT = 3
MAX_DAYS_PER_EMPLOYEE = 5

def parse_ranking(s):
    s = s.strip().lower()
    if s == "" or s == "off" or s == "o":
        return ["off"]
    s = s.replace(",", ">").replace("/", ">").replace("|", ">")
    parts = [p.strip() for p in s.split(">") if p.strip()]

    out = []
    for p in parts:
        if p in ["morning","m"]:
            val = "morning"
        elif p in ["afternoon","a"]:
            val = "afternoon"
        elif p in ["evening","e"]:
            val = "evening"
        elif p in ["off","o"]:
            val = "off"
        else:
            return None
        if val not in out:
            out.append(val)

    if not out or "off" in out:
        return ["off"]

    for sh in SHIFTS:
        if sh not in out:
            out.append(sh)
    return out

def read_names():
    while True:
        raw = input("Enter employee names (comma-separated): ").strip()
        if raw:
            names = [x.strip() for x in raw.split(",") if x.strip()]
            if names:
                return names
        print("Please enter at least one name.")

def collect_preferences(names):
    prefs = {name: {} for name in names}
    print("\nEnter per-day preference for each person.")
    print("Use morning/afternoon/evening/off or ranking like m>a>e. Blank = off.\n")
    for n in names:
        print(f"-- {n} --")
        for d in DAYS:
            while True:
                raw = input(f"{d} preference: ")
                if raw.strip() == "":
                    prefs[n][d] = ["off"]
                    break
                r = parse_ranking(raw)
                if r is not None:
                    prefs[n][d] = r
                    break
                else:
                    print("  Invalid. Try: morning, afternoon, evening, off, or m>a>e.")
    return prefs

def try_assign(schedule, assigned_today, days_count, day, shift, name):
    if name in assigned_today:
        return False
    if days_count[name] >= MAX_DAYS_PER_EMPLOYEE:
        return False
    if len(schedule[day][shift]) >= MAX_PER_SHIFT:
        return False
    schedule[day][shift].append(name)
    assigned_today.add(name)
    days_count[name] += 1
    return True

def main():
    random.seed(42)

    names = read_names()
    prefs = collect_preferences(names)

    schedule = {d: {s: [] for s in SHIFTS} for d in DAYS}
    days_count = {n: 0 for n in names}
    deferred = {i: [] for i in range(len(DAYS))}

    for di, day in enumerate(DAYS):
        assigned_today = set()

        incoming = list(deferred.get(di, []))
        random.shuffle(incoming)

        rest = [n for n in names if n not in incoming]
        random.shuffle(rest)
        ordered = incoming + rest

        for rank in range(3):
            for n in ordered:
                if n in assigned_today or days_count[n] >= MAX_DAYS_PER_EMPLOYEE:
                    continue
                r = prefs[n].get(day, ["off"])
                if r == ["off"]:
                    continue
                preferred = r[rank]
                if try_assign(schedule, assigned_today, days_count, day, preferred, n):
                    continue
                for alt in SHIFTS:
                    if alt == preferred:
                        continue
                    if try_assign(schedule, assigned_today, days_count, day, alt, n):
                        break

        for sh in SHIFTS:
            while len(schedule[day][sh]) < MIN_PER_SHIFT:
                candidates = []
                for n in names:
                    if n in assigned_today: 
                        continue
                    if days_count[n] >= MAX_DAYS_PER_EMPLOYEE:
                        continue
                    r = prefs[n].get(day, ["off"])
                    if r != ["off"]:
                        candidates.append(n)
                if not candidates:
                    candidates = [n for n in names if n not in assigned_today and days_count[n] < MAX_DAYS_PER_EMPLOYEE]
                if not candidates:
                    break
                pick = random.choice(candidates)
                try_assign(schedule, assigned_today, days_count, day, sh, pick)

        if di + 1 < len(DAYS):
            for n in names:
                if n not in assigned_today and days_count[n] < MAX_DAYS_PER_EMPLOYEE:
                    r = prefs[n].get(day, ["off"])
                    if r != ["off"]:
                        deferred[di + 1].append(n)

    print("\n===== FINAL SCHEDULE =====\n")
    for d in DAYS:
        print(d)
        for s in SHIFTS:
            line = ", ".join(schedule[d][s]) if schedule[d][s] else "-"
            print(f"  {s:<9}: {line}")
        print()
    print("Totals (days assigned, max 5):")
    for n in sorted(names):
        print(f"  {n:<12} {days_count[n]}")

if __name__ == "__main__":
    main()