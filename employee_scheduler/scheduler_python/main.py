from __future__ import annotations
import json
from typing import Dict, List

DAYS = ["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"]
VALID_SHIFTS = {"morning","afternoon","evening","off"}

def parse_shift_input(raw: str) -> List[str]:

    s = raw.strip().lower()
    if not s:
        return ["off"]
    for separator in [",", "/", "|"]:
        s = s.replace(separator, ">")
    parts = [p.strip() for p in s.split(">") if p.strip()]
    res: List[str] = []
    for p in parts:
        if p in VALID_SHIFTS:
            res.append(p)
        else:
            m = {"m":"morning","a":"afternoon","e":"evening","o":"off"}.get(p[:1], None)
            if m:
                res.append(m)
    if not res:
        return ["off"]
    seen = set()
    uniq: List[str] = []
    for r in res:
        if r not in seen:
            seen.add(r); uniq.append(r)
    if "off" in uniq:
        return ["off"]
    for sh in ["morning","afternoon","evening"]:
        if sh not in uniq:
            uniq.append(sh)
    return uniq

def ask(prompt: str) -> str:
    try:
        return input(prompt)
    except EOFError:
        return ""

def collect_preferences() -> Dict[str, Dict[str, List[str]]]:
    print("Enter employee names separated by commas (e.g., Alice,Bob,Charlie)")
    raw_names = ask("> ").strip()
    while not raw_names:
        print("Please enter at least one name.")
        raw_names = ask("> ").strip()
    names = [n.strip() for n in raw_names.split(",") if n.strip()]
    preferences: Dict[str, Dict[str, List[str]]] = {name: {} for name in names}
    print("\nEnter a preferred shift *per day* for each employee.")
    print("Accepted: morning/afternoon/evening/off or a ranking like 'm>a>e'. Blank = off.\n")
    for name in names:
        print(f"\n-- {name} --")
        for day in DAYS:
            raw = ask(f"{day} preference: ").strip()
            prefs = parse_shift_input(raw)
            preferences[name][day] = prefs
    return preferences

def main():
    prefs = collect_preferences()
    print("\nCollected preferences (JSON):")
    print(json.dumps(prefs, indent=2))
    save = ask("\nSave to preferences.json? [y/N] ").strip().lower()
    if save.startswith("y"):
        with open("preferences.json","w", encoding="utf-8") as f:
            json.dump(prefs, f, indent=2)
        print("Wrote preferences.json")

if __name__ == "__main__":
    main()