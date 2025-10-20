# rr_scheduler_basic.py
# Round-robin per shift, beginner style

import random

DAYS = ["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"]
SHIFTS = ["morning","afternoon","evening"]

MIN_COVER = 2
MAX_CAP   = 3       # max people in a shift (capacity)
MAX_DAYS  = 5       # max days per employee

def parse_rank(s):
    s = s.strip().lower()
    if s == "" or s in ("off","o"):
        return ["off"]
    s = s.replace(",", ">").replace("/", ">").replace("|", ">")
    parts = [p.strip() for p in s.split(">") if p.strip()]
    out = []
    for p in parts:
        if p in ("morning","m"): out.append("morning")
        elif p in ("afternoon","a"): out.append("afternoon")
        elif p in ("evening","e"): out.append("evening")
        elif p in ("off","o"): out.append("off")
        else: return None
    # unique & pad
    u = []
    for x in out:
        if x not in u: u.append(x)
    if "off" in u or not u:
        return ["off"]
    for sh in SHIFTS:
        if sh not in u: u.append(sh)
    return u

def read_names():
    while True:
        raw = input("Enter employee names (comma-separated): ").strip()
        if raw:
            names = [x.strip() for x in raw.split(",") if x.strip()]
            if names: return names
        print("Please enter at least one name.")

def read_prefs(names):
    prefs = {n: {} for n in names}
    print("\nEnter preferences per day (morning/afternoon/evening/off or ranking like m>a>e). Blank = off.\n")
    for n in names:
        print(f"-- {n} --")
        for d in DAYS:
            while True:
                raw = input(f"{d} preference: ")
                if raw.strip() == "":
                    prefs[n][d] = ["off"]; break
                r = parse_rank(raw)
                if r is not None:
                    prefs[n][d] = r; break
                print("  Invalid. Try again.")
    return prefs

def assign_from_queue(day, shift, queue, placed_today, totals, schedule):
    # take from queue in order until capacity, obeying limits
    i = 0
    made_move = False
    while len(schedule[day][shift]) < MAX_CAP and i < len(queue):
        name = queue[i]
        if (name not in placed_today
            and totals[name] < MAX_DAYS):
            schedule[day][shift].append(name)
            placed_today.add(name)
            totals[name] += 1
            queue.pop(i)
            made_move = True
        else:
            i += 1
    return made_move

def main():
    random.seed(1)  # deterministic-ish

    people = read_names()
    prefs = read_prefs(people)

    # schedule structure
    schedule = {d: {s: [] for s in SHIFTS} for d in DAYS}
    totals   = {n: 0 for n in people}
    # simple “rotation pointer” per shift (fairness across days)
    rotation = {s: 0 for s in SHIFTS}

    # for “deferral”: who tried today but didn’t get placed (and wasn’t off)
    carry_over = {i: [] for i in range(len(DAYS))}

    for di, day in enumerate(DAYS):
        placed_today = set()

        # --- build preference buckets per shift: firsts, seconds, thirds ---
        first = {s: [] for s in SHIFTS}
        second = {s: [] for s in SHIFTS}
        third = {s: [] for s in SHIFTS}

        # anyone deferred gets pushed to the front of their first-choice queue
        front = carry_over.get(di, [])

        # Build ordered list (deferred first, then others)
        ordered = front + [n for n in people if n not in front]

        for name in ordered:
            r = prefs[name].get(day, ["off"])
            if r == ["off"]:
                continue
            # r has at least 3 (padded)
            if r[0] in SHIFTS: first[r[0]].append(name)
            if r[1] in SHIFTS: second[r[1]].append(name)
            if r[2] in SHIFTS: third[r[2]].append(name)

        # rotate queues for fairness (round-robin)
        for s in SHIFTS:
            k = rotation[s] % (len(first[s]) if first[s] else 1)
            first[s] = first[s][k:] + first[s][:k]
            k2 = rotation[s] % (len(second[s]) if second[s] else 1)
            second[s] = second[s][k2:] + second[s][:k2]
            k3 = rotation[s] % (len(third[s]) if third[s] else 1)
            third[s] = third[s][k3:] + third[s][:k3]

        # --- 3 passes per shift: first choice, then second, then third ---
        for s in SHIFTS:
            assign_from_queue(day, s, first[s], placed_today, totals, schedule)
        for s in SHIFTS:
            assign_from_queue(day, s, second[s], placed_today, totals, schedule)
        for s in SHIFTS:
            assign_from_queue(day, s, third[s], placed_today, totals, schedule)

        # --- ensure minimum staffing per shift ---
        for s in SHIFTS:
            while len(schedule[day][s]) < MIN_COVER:
                # choose among available people who are not off today
                candidates = []
                for name in people:
                    if name in placed_today: continue
                    if totals[name] >= MAX_DAYS: continue
                    r = prefs[name].get(day, ["off"])
                    if r != ["off"]:
                        candidates.append(name)
                if not candidates:
                    # if still none, choose any available
                    candidates = [n for n in people if n not in placed_today and totals[n] < MAX_DAYS]
                if not candidates:
                    break
                # prefer person with fewest days so far (very simple fairness)
                candidates.sort(key=lambda x: totals[x])
                pick = candidates[0]
                if len(schedule[day][s]) < MAX_CAP:
                    schedule[day][s].append(pick)
                    placed_today.add(pick)
                    totals[pick] += 1
                else:
                    break

        # --- update rotation so next day starts from a different point ---
        for s in SHIFTS:
            rotation[s] = (rotation[s] + 1) % 1000  # any small step

        # --- build carry-over for next day (not off, not placed, not at cap) ---
        if di + 1 < len(DAYS):
            nxt = []
            for name in people:
                r = prefs[name].get(day, ["off"])
                if r != ["off"] and name not in placed_today and totals[name] < MAX_DAYS:
                    nxt.append(name)
            carry_over[di + 1] = nxt

    # ----- print final schedule -----
    print("\n===== FINAL SCHEDULE (Round-Robin) =====\n")
    for d in DAYS:
        print(d)
        for s in SHIFTS:
            line = ", ".join(schedule[d][s]) if schedule[d][s] else "-"
            print(f"  {s:<9}: {line}")
        print()
    print("Totals (days assigned, max 5):")
    for n in sorted(people):
        print(f"  {n:<12} {totals[n]}")

if __name__ == "__main__":
    main()