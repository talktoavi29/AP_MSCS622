// rr_scheduler_basic.rs
// Round-robin per shift; beginner style; std only

use std::io::{self, Write};

const DAYS: [&str; 7] = ["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"];
const SHIFTS: [&str; 3] = ["morning","afternoon","evening"];

const MIN_COVER: usize = 2;
const MAX_CAP:   usize = 3;
const MAX_DAYS:  usize = 5;

// tiny LCG RNG (std only)
struct Lcg { st: u64 }
impl Lcg {
    fn new(seed: u64) -> Self { Self { st: seed } }
    fn next_u32(&mut self) -> u32 {
        self.st = self.st.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.st >> 16) as u32
    }
}

fn read_line(p: &str) -> String {
    print!("{p}");
    let _ = io::stdout().flush();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

fn parse_rank(raw: &str) -> Option<Vec<&'static str>> {
    let s = raw.trim().to_lowercase();
    if s.is_empty() || s == "off" || s == "o" { return Some(vec!["off"]); }
    let t = s.replace(",", ">").replace("/", ">").replace("|", ">");
    let parts: Vec<&str> = t.split('>').map(|x| x.trim()).filter(|x| !x.is_empty()).collect();
    if parts.is_empty() { return Some(vec!["off"]); }
    let mut out: Vec<&str> = Vec::new();
    for p in parts {
        let v = match p {
            "morning" | "m" => "morning",
            "afternoon" | "a" => "afternoon",
            "evening" | "e" => "evening",
            "off" | "o" => "off",
            _ => return None,
        };
        if !out.contains(&v) { out.push(v); }
    }
    if out.is_empty() || out.contains(&"off") { return Some(vec!["off"]); }
    for sh in SHIFTS {
        if !out.contains(&sh) { out.push(sh); }
    }
    Some(out)
}

fn main() {
    // names
    let raw = loop {
        let s = read_line("Enter employee names (comma-separated): ");
        if !s.trim().is_empty() { break s; }
        println!("Please enter at least one name.");
    };
    let names: Vec<String> = raw.split(',')
        .map(|x| x.trim()).filter(|x| !x.is_empty())
        .map(|x| x.to_string()).collect();

    // prefs[emp][day] -> Vec<&str>
    let mut prefs: Vec<Vec<Vec<&str>>> = vec![vec![vec![]; DAYS.len()]; names.len()];
    println!("\nEnter preferences (morning/afternoon/evening/off or m>a>e). Blank = off.\n");
    for ei in 0..names.len() {
        println!("-- {} --", names[ei]);
        for di in 0..DAYS.len() {
            loop {
                let r = read_line(&format!("{} preference: ", DAYS[di]));
                if r.trim().is_empty() { prefs[ei][di] = vec!["off"]; break; }
                if let Some(v) = parse_rank(&r) { prefs[ei][di] = v; break; }
                println!("  Invalid. Try again.");
            }
        }
    }

    // schedule[day][shift] -> Vec<emp_index>
    let mut schedule: Vec<Vec<Vec<usize>>> = vec![vec![Vec::new(); SHIFTS.len()]; DAYS.len()];
    let mut totals: Vec<usize> = vec![0; names.len()];
    let mut rotation: [usize; 3] = [0, 0, 0];
    let mut carry: Vec<Vec<usize>> = vec![Vec::new(); DAYS.len()];

    let mut _rng = Lcg::new(1); // not used much here, but kept for parity

    for di in 0..DAYS.len() {
        let mut placed = vec![false; names.len()];

        // build first/second/third buckets per shift
        let mut first: [Vec<usize>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        let mut second: [Vec<usize>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        let mut third: [Vec<usize>; 3] = [Vec::new(), Vec::new(), Vec::new()];

        // order: carry first, then others
        let mut order: Vec<usize> = Vec::new();
        for &ei in &carry[di] { order.push(ei); }
        for ei in 0..names.len() {
            if !order.contains(&ei) { order.push(ei); }
        }

        for &ei in &order {
            let r = &prefs[ei][di];
            if r.len() == 1 && r[0] == "off" { continue; }
            // map shift str -> index 0..2
            let mut idx = |label: &str| -> Option<usize> {
                for i in 0..3 { if SHIFTS[i] == label { return Some(i); } }
                None
            };
            if let Some(i0) = idx(r[0]) { first[i0].push(ei); }
            if let Some(i1) = idx(r[1]) { second[i1].push(ei); }
            if let Some(i2) = idx(r[2]) { third[i2].push(ei); }
        }

        // rotate queues
        for si in 0..3 {
            let k = rotation[si] % (if first[si].is_empty() { 1 } else { first[si].len() });
            first[si].rotate_left(k);
            let k2 = rotation[si] % (if second[si].is_empty() { 1 } else { second[si].len() });
            second[si].rotate_left(k2);
            let k3 = rotation[si] % (if third[si].is_empty() { 1 } else { third[si].len() });
            third[si].rotate_left(k3);
        }

        // helper to assign from a bucket
        let mut assign_from = |si: usize, bucket: &mut Vec<usize>| {
            let mut i = 0usize;
            while schedule[di][si].len() < MAX_CAP && i < bucket.len() {
                let e = bucket[i];
                if !placed[e] && totals[e] < MAX_DAYS {
                    schedule[di][si].push(e);
                    placed[e] = true;
                    totals[e] += 1;
                    bucket.remove(i);
                } else {
                    i += 1;
                }
            }
        };

        // three passes
        for si in 0..3 { assign_from(si, &mut first[si]); }
        for si in 0..3 { assign_from(si, &mut second[si]); }
        for si in 0..3 { assign_from(si, &mut third[si]); }

        // ensure minimum staffing
        for si in 0..3 {
            while schedule[di][si].len() < MIN_COVER {
                // prefer available and not-off today
                let mut cand: Vec<usize> = Vec::new();
                for ei in 0..names.len() {
                    if placed[ei] { continue; }
                    if totals[ei] >= MAX_DAYS { continue; }
                    let r = &prefs[ei][di];
                    if !(r.len() == 1 && r[0] == "off") {
                        cand.push(ei);
                    }
                }
                if cand.is_empty() {
                    for ei in 0..names.len() {
                        if !placed[ei] && totals[ei] < MAX_DAYS {
                            cand.push(ei);
                        }
                    }
                }
                if cand.is_empty() { break; }
                // pick least-used for simple fairness
                cand.sort_by_key(|&ei| totals[ei]);
                let pick = cand[0];
                if schedule[di][si].len() < MAX_CAP {
                    schedule[di][si].push(pick);
                    placed[pick] = true;
                    totals[pick] += 1;
                } else { break; }
            }
        }

        // advance rotation
        for si in 0..3 { rotation[si] = (rotation[si] + 1) % 1000; }

        // carry over for next day
        if di + 1 < DAYS.len() {
            let mut nxt: Vec<usize> = Vec::new();
            for ei in 0..names.len() {
                let r = &prefs[ei][di];
                let not_off = !(r.len() == 1 && r[0] == "off");
                if !placed[ei] && totals[ei] < MAX_DAYS && not_off {
                    nxt.push(ei);
                }
            }
            carry[di + 1] = nxt;
        }
    }

    // output
    println!("\n===== FINAL SCHEDULE (Round-Robin) =====\n");
    for di in 0..DAYS.len() {
        println!("{}", DAYS[di]);
        for si in 0..3 {
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
        println!("  {:<12} {}", names[ei], totals[ei]);
    }
}