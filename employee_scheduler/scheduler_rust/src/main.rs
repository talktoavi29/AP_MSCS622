use std::io::{self, Write};

const DAYS: [&str; 7] = ["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"];
const SHIFTS: [&str; 3] = ["morning","afternoon","evening"];

const MIN_PER_SHIFT: usize = 2;
const MAX_PER_SHIFT: usize = 3;
const MAX_DAYS_PER_EMPLOYEE: usize = 5;

struct Lcg {
    state: u64,
}
impl Lcg {
    fn new(seed: u64) -> Self { Self { state: seed } }
    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.state >> 16) as u32
    }
    fn choose<'a>(&mut self, v: &'a [usize]) -> Option<usize> {
        if v.is_empty() { None } else {
            let idx = (self.next_u32() as usize) % v.len();
            Some(v[idx])
        }
    }
}

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

fn parse_ranking(raw: &str) -> Option<Vec<&'static str>> {
    let s = raw.trim().to_lowercase();
    if s.is_empty() || s == "off" || s == "o" {
        return Some(vec!["off"]);
    }
    let mut t = s.replace(",", ">").replace("/", ">").replace("|", ">");
    let parts: Vec<&str> = t.split('>').map(|x| x.trim()).filter(|x| !x.is_empty()).collect();
    if parts.is_empty() {
        return Some(vec!["off"]);
    }

    let mut out: Vec<&str> = Vec::new();
    for p in parts {
        let val = match p {
            "morning" | "m" => "morning",
            "afternoon" | "a" => "afternoon",
            "evening" | "e" => "evening",
            "off" | "o" => "off",
            _ => return None,
        };
        if !out.contains(&val) {
            out.push(val);
        }
    }
    if out.is_empty() || out.contains(&"off") {
        return Some(vec!["off"]);
    }
    for sh in SHIFTS {
        if !out.contains(&sh) {
            out.push(sh);
        }
    }
    Some(out)
}

fn main() {
    let names_raw = loop {
        let s = read_line("Enter employee names (comma-separated): ");
        if !s.trim().is_empty() { break s; }
        println!("Please enter at least one name.");
    };
    let mut names: Vec<String> = names_raw
        .split(',')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();

    let mut prefs: Vec<Vec<Vec<&str>>> = vec![vec![vec![]; DAYS.len()]; names.len()];
    println!("\nEnter per-day preference for each person.");
    println!("Use morning/afternoon/evening/off or ranking like m>a>e. Blank = off.\n");
    for ei in 0..names.len() {
        println!("\n-- {} --", names[ei]);
        for di in 0..DAYS.len() {
            loop {
                let raw = read_line(&format!("{} preference: ", DAYS[di]));
                if raw.trim().is_empty() {
                    prefs[ei][di] = vec!["off"];
                    break;
                }
                if let Some(r) = parse_ranking(&raw) {
                    prefs[ei][di] = r;
                    break;
                } else {
                    println!("  Invalid. Try: morning/afternoon/evening/off or m>a>e.");
                }
            }
        }
    }

    let mut schedule: Vec<Vec<Vec<usize>>> =
        vec![vec![Vec::new(); SHIFTS.len()]; DAYS.len()];

    let mut days_count: Vec<usize> = vec![0; names.len()];

    let mut assigned_today: Vec<bool>;
    let mut deferred: Vec<Vec<usize>> = vec![Vec::new(); DAYS.len()];

    let mut rng = Lcg::new(42);

    for di in 0..DAYS.len() {
        assigned_today = vec![false; names.len()];

        let incoming = deferred[di].clone();

        let mut order: Vec<usize> = Vec::new();
        for ei in incoming { order.push(ei); }
        for ei in 0..names.len() {
            if !order.contains(&ei) {
                order.push(ei);
            }
        }

        for rank in 0..3 {
            for &ei in &order {
                if assigned_today[ei] || days_count[ei] >= MAX_DAYS_PER_EMPLOYEE {
                    continue;
                }
                let r = &prefs[ei][di];
                if r.len() == 1 && r[0] == "off" {
                    continue;
                }
                let preferred = r[rank];
                let mut placed = false;
                for si in 0..SHIFTS.len() {
                    let sh = SHIFTS[si];
                    if sh == preferred {
                        if schedule[di][si].len() < MAX_PER_SHIFT && !assigned_today[ei] && days_count[ei] < MAX_DAYS_PER_EMPLOYEE {
                            schedule[di][si].push(ei);
                            assigned_today[ei] = true;
                            days_count[ei] += 1;
                            placed = true;
                        }
                        break;
                    }
                }
                if placed { continue; }
                for si in 0..SHIFTS.len() {
                    if SHIFTS[si] == preferred { continue; }
                    if schedule[di][si].len() < MAX_PER_SHIFT && !assigned_today[ei] && days_count[ei] < MAX_DAYS_PER_EMPLOYEE {
                        schedule[di][si].push(ei);
                        assigned_today[ei] = true;
                        days_count[ei] += 1;
                        break;
                    }
                }
            }
        }

        for si in 0..SHIFTS.len() {
            while schedule[di][si].len() < MIN_PER_SHIFT {
                let mut candidates: Vec<usize> = Vec::new();
                for ei in 0..names.len() {
                    if assigned_today[ei] { continue; }
                    if days_count[ei] >= MAX_DAYS_PER_EMPLOYEE { continue; }
                    let r = &prefs[ei][di];
                    if !(r.len() == 1 && r[0] == "off") {
                        candidates.push(ei);
                    }
                }
                if candidates.is_empty() {
                    for ei in 0..names.len() {
                        if !assigned_today[ei] && days_count[ei] < MAX_DAYS_PER_EMPLOYEE {
                            candidates.push(ei);
                        }
                    }
                }
                if candidates.is_empty() { break; }
                if let Some(pick) = rng.choose(&candidates) {
                    if schedule[di][si].len() < MAX_PER_SHIFT {
                        schedule[di][si].push(pick);
                        assigned_today[pick] = true;
                        days_count[pick] += 1;
                    } else { break; }
                } else { break; }
            }
        }

        if di + 1 < DAYS.len() {
            for ei in 0..names.len() {
                let r = &prefs[ei][di];
                let not_off = !(r.len() == 1 && r[0] == "off");
                if !assigned_today[ei] && days_count[ei] < MAX_DAYS_PER_EMPLOYEE && not_off {
                    deferred[di + 1].push(ei);
                }
            }
        }
    }

    println!("\n===== FINAL SCHEDULE =====\n");
    for di in 0..DAYS.len() {
        println!("{}", DAYS[di]);
        for si in 0..SHIFTS.len() {
            if schedule[di][si].is_empty() {
                println!("  {:<9}: -", SHIFTS[si]);
            } else {
                let mut line = String::new();
                for (k, &ei) in schedule[di][si].iter().enumerate() {
                    if k > 0 { line.push_str(", "); }
                    line.push_str(&names[ei]);
                }
                println!("  {:<9}: {}", SHIFTS[si], line);
            }
        }
        println!();
    }
    println!("Totals (days assigned, max 5):");
    for ei in 0..names.len() {
        println!("  {:<12} {}", names[ei], days_count[ei]);
    }
}