use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::fs::File;

const DAYS: [&str; 7] = ["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"];
const SHIFTS: [&str; 3] = ["morning","afternoon","evening"];

const MIN_PER_SHIFT: usize = 2;
const MAX_PER_SHIFT: usize = 3;
const MAX_DAYS_PER_EMPLOYEE: usize = 5;
const RANDOM_SEED: u64 = 42;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
enum Shift { Morning, Afternoon, Evening, Off }

fn to_shift(s: &str) -> Option<Shift> {
    match s {
        "morning" | "m" => Some(Shift::Morning),
        "afternoon" | "a" => Some(Shift::Afternoon),
        "evening" | "e" => Some(Shift::Evening),
        "off" | "o" => Some(Shift::Off),
        _ => None
    }
}

fn parse_ranking(raw: &str) -> Option<Vec<Shift>> {
    let mut s = raw.trim().to_lowercase();
    if s.is_empty() { return Some(vec![Shift::Off]); }
    for sep in [",","/","|"] { s = s.replace(sep, ">"); }
    let mut out: Vec<Shift> = Vec::new();
    for p in s.split('>').map(|x| x.trim()).filter(|x| !x.is_empty()) {
        if let Some(sh) = to_shift(p) {
            if !out.contains(&sh) { out.push(sh); }
        } else {
            return None;
        }
    }
    if out.is_empty() || out.contains(&Shift::Off) {
        return Some(vec![Shift::Off]);
    }
    for sh in [Shift::Morning, Shift::Afternoon, Shift::Evening] {
        if !out.contains(&sh) { out.push(sh); }
    }
    Some(out)
}

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    buf.trim_end().to_string()
}

type Prefs = HashMap<String, HashMap<String, Vec<Shift>>>;
type Schedule = HashMap<String, HashMap<String, Vec<String>>>;

fn try_assign(day: &str, shift: &str, name: &str,
              schedule: &mut Schedule,
              assigned_days_count: &mut HashMap<String, usize>,
              daily_assigned: &mut HashMap<String, HashSet<String>>) -> bool {
    if daily_assigned.get(day).unwrap().contains(name) { return false; }
    if *assigned_days_count.get(name).unwrap() >= MAX_DAYS_PER_EMPLOYEE { return false; }
    if schedule.get(day).unwrap().get(shift).unwrap().len() >= MAX_PER_SHIFT { return false; }
    schedule.get_mut(day).unwrap().get_mut(shift).unwrap().push(name.to_string());
    daily_assigned.get_mut(day).unwrap().insert(name.to_string());
    *assigned_days_count.get_mut(name).unwrap() += 1;
    true
}

fn schedule_week(prefs: &Prefs, seed: u64) -> (Schedule, HashMap<String, usize>) {
    let mut rng = StdRng::seed_from_u64(seed);
    let employees: Vec<String> = prefs.keys().cloned().collect();

    let mut schedule: Schedule = DAYS.iter()
        .map(|d| (d.to_string(), SHIFTS.iter()
            .map(|s| (s.to_string(), Vec::<String>::new()))
            .collect::<HashMap<_,_>>()))
        .collect();

    let mut assigned_days_count: HashMap<String, usize> =
        employees.iter().map(|n| (n.clone(), 0usize)).collect();

    let mut daily_assigned: HashMap<String, HashSet<String>> =
        DAYS.iter().map(|d| (d.to_string(), HashSet::new())).collect();

    let mut deferred_next_day: HashMap<usize, Vec<String>> =
        (0..DAYS.len()).map(|i| (i, Vec::new())).collect();

    for (di, day) in DAYS.iter().enumerate() {
        let mut incoming = deferred_next_day.get(&di).cloned().unwrap_or_default();
        incoming.shuffle(&mut rng);
    let mut others: Vec<String> = employees.iter()
        .filter(|n| !incoming.contains(*n))
        .cloned()
        .collect();        
        others.shuffle(&mut rng);
        let mut ordered = incoming;
        ordered.extend(others);

        for rank in 0..3 {
            for name in &ordered {
                if daily_assigned.get(*day).unwrap().contains(name) { continue; }
                if *assigned_days_count.get(name.as_str()).unwrap() >= MAX_DAYS_PER_EMPLOYEE { continue; }
                let ranking = prefs.get(name).and_then(|m| m.get(*day)).cloned().unwrap_or(vec![Shift::Off]);
                if ranking == vec![Shift::Off] { continue; }
                let preferred = &ranking[rank];
                let pref_str = match preferred {
                    Shift::Morning => "morning",
                    Shift::Afternoon => "afternoon",
                    Shift::Evening => "evening",
                    Shift::Off => "off",
                };
                if pref_str != "off" && try_assign(day, pref_str, name, &mut schedule, &mut assigned_days_count, &mut daily_assigned) {
                    continue;
                }
                // try other shifts
                for alt in SHIFTS {
                    if alt == pref_str { continue; }
                    if try_assign(day, alt, name, &mut schedule, &mut assigned_days_count, &mut daily_assigned) {
                        break;
                    }
                }
            }
        }

        for shift in SHIFTS {
            while schedule.get(*day).unwrap().get(shift).unwrap().len() < MIN_PER_SHIFT {
                let mut candidates: Vec<String> = employees.iter().filter(|n| {
                    !daily_assigned.get(*day).unwrap().contains(n.as_str()) &&
                    *assigned_days_count.get(n.as_str()).unwrap() < MAX_DAYS_PER_EMPLOYEE &&
                    prefs.get(*n).and_then(|m| m.get(*day)).map(|v| v != &vec![Shift::Off]).unwrap_or(false)
                }).cloned().collect();

                if candidates.is_empty() {
                    candidates = employees.iter().filter(|n| {
                        !daily_assigned.get(*day).unwrap().contains(n.as_str()) &&
                        *assigned_days_count.get(n.as_str()).unwrap() < MAX_DAYS_PER_EMPLOYEE
                    }).cloned().collect();
                }
                if candidates.is_empty() { break; }
                candidates.shuffle(&mut rng);
                let pick = candidates[0].clone();
                let _ = try_assign(day, shift, &pick, &mut schedule, &mut assigned_days_count, &mut daily_assigned);
            }
        }

        if di + 1 < DAYS.len() {
            for name in &employees {
                if !daily_assigned.get(*day).unwrap().contains(name.as_str())
                    && *assigned_days_count.get(name.as_str()).unwrap() < MAX_DAYS_PER_EMPLOYEE
                    && prefs.get(name).and_then(|m| m.get(*day)).map(|v| v != &vec![Shift::Off]).unwrap_or(false)
                {
                    deferred_next_day.entry(di + 1).or_default().push(name.clone());
                }
            }
        }
    }

    (schedule, assigned_days_count)
}

fn collect_preferences() -> Prefs {
    println!("Enter employee names separated by commas (e.g., Alice,Bob,Charlie)");
    let raw = loop {
        let s = read_line("> ");
        if !s.trim().is_empty() { break s; }
        println!("Please enter at least one name.");
    };
    let names: Vec<String> = raw.split(',')
        .map(|s| s.trim()).filter(|s| !s.is_empty())
        .map(|s| s.to_string()).collect();

    let mut prefs: Prefs = HashMap::new();
    for n in &names {
        prefs.insert(n.clone(), HashMap::new());
    }

    println!("\nEnter a preferred shift *per day* for each employee.");
    println!("Accepted: morning/afternoon/evening/off or ranking like 'm>a>e'. Blank = off.\n");

    for n in &names {
        println!("\n-- {} --", n);
        for d in DAYS {
            loop {
                let raw = read_line(&format!("{d} preference: "));
                if raw.trim().is_empty() {
                    prefs.get_mut(n).unwrap().insert(d.to_string(), vec![Shift::Off]);
                    break;
                }
                if let Some(r) = parse_ranking(&raw) {
                    prefs.get_mut(n).unwrap().insert(d.to_string(), r);
                    break;
                } else {
                    println!("  Invalid. Use morning/afternoon/evening/off or ranking like m>a>e.");
                }
            }
        }
    }
    prefs
}

fn print_schedule(schedule: &Schedule, totals: &HashMap<String, usize>) {
    println!("\n================ WEEKLY SCHEDULE ================\n");
    for day in DAYS {
        println!("{day}");
        for shift in SHIFTS {
            let names = &schedule[day][shift];
            let show = if names.is_empty() { "-".to_string() } else { names.join(", ") };
            println!("  {:<9}: {}", shift, show);
        }
        println!();
    }
    println!("Totals (days assigned per employee, max 5):");
    let mut keys: Vec<_> = totals.keys().collect();
    keys.sort();
    for k in keys {
        println!("  {:<15} {}", k, totals[k]);
    }
    println!();
}

fn write_csv(schedule: &Schedule, path: &str) {
    let file = File::create(path).expect("create csv");
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&["Day","Shift","Employees"]).ok();
    for day in DAYS {
        for shift in SHIFTS {
            let emps = schedule[day][shift].join("; ");
            wtr.write_record(&[day, shift, &emps]).ok();
        }
    }
    wtr.flush().ok();
}

fn write_md(schedule: &Schedule, path: &str) {
    let mut out = String::new();
    out.push_str("| Day | Morning | Afternoon | Evening |\n");
    out.push_str("|---|---|---|---|\n");
    for day in DAYS {
        let m = if schedule[day]["morning"].is_empty() { "-".to_string() } else { schedule[day]["morning"].join(", ") };
        let a = if schedule[day]["afternoon"].is_empty() { "-".to_string() } else { schedule[day]["afternoon"].join(", ") };
        let e = if schedule[day]["evening"].is_empty() { "-".to_string() } else { schedule[day]["evening"].join(", ") };
        out.push_str(&format!("| {} | {} | {} | {} |\n", day, m, a, e));
    }
    std::fs::write(path, out).ok();
}

fn main() {
    println!("== Employee Scheduler (Rust) ==");
    let prefs = collect_preferences();

    let cap = prefs.len() * MAX_DAYS_PER_EMPLOYEE;
    let need = DAYS.len() * SHIFTS.len() * MIN_PER_SHIFT;
    if cap < need {
        println!("\n[Warning] Weekly capacity {} < required minimum {}. Some shifts may be under-staffed.\n", cap, need);
    }

    let (schedule, totals) = schedule_week(&prefs, RANDOM_SEED);

    print_schedule(&schedule, &totals);
    std::fs::write("preferences.json", serde_json::to_string_pretty(&prefs).unwrap()).ok();
    std::fs::write("schedule.json", serde_json::to_string_pretty(&schedule).unwrap()).ok();
    write_csv(&schedule, "schedule.csv");
    write_md(&schedule, "schedule.md");
    println!("Wrote: preferences.json, schedule.json, schedule.csv, schedule.md");
}