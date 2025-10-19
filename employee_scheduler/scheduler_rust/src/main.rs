use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};

const DAYS: [&str; 7] = ["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
enum Shift {
    Morning,
    Afternoon,
    Evening,
    Off,
}

fn parse_shift_input(raw: &str) -> Vec<Shift> {
    let mut s = raw.trim().to_lowercase();
    if s.is_empty() {
        return vec![Shift::Off];
    }
    for sep in [",", "/", "|"] {
        s = s.replace(sep, ">");
    }
    let mut out: Vec<Shift> = Vec::new();
    for p in s.split('>').map(|x| x.trim()).filter(|x| !x.is_empty()) {
        let sh = match p {
            "morning" | "m" => Some(Shift::Morning),
            "afternoon" | "a" => Some(Shift::Afternoon),
            "evening" | "e" => Some(Shift::Evening),
            "off" | "o" => Some(Shift::Off),
            _ => None,
        };
        if let Some(sh) = sh {
            if !out.contains(&sh) {
                out.push(sh);
            }
        }
    }
    if out.is_empty() {
        return vec![Shift::Off];
    }
    if out.contains(&Shift::Off) {
        return vec![Shift::Off];
    }
    // Fill missing in default order
    for sh in [Shift::Morning, Shift::Afternoon, Shift::Evening] {
        if !out.contains(&sh) {
            out.push(sh);
        }
    }
    out
}

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    buf.trim_end().to_string()
}

fn main() {
    println!("Enter employee names separated by commas (e.g., Alice,Bob,Charlie)");
    let raw_names = loop {
        let s = read_line("> ");
        if !s.trim().is_empty() {
            break s;
        }
        println!("Please enter at least one name.");
    };
    let names: Vec<String> = raw_names
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    type Prefs = HashMap<String, HashMap<String, Vec<Shift>>>;
    let mut prefs: Prefs = HashMap::new();
    for name in &names {
        prefs.insert(name.clone(), HashMap::new());
    }

    println!("\nEnter a preferred shift *per day* for each employee.");
    println!("Accepted: morning/afternoon/evening/off or a ranking like 'm>a>e'. Blank = off.\n");

    for name in &names {
        println!("\n-- {} --", name);
        for day in DAYS {
            let raw = read_line(&format!("{day} preference: "));
            let p = parse_shift_input(&raw);
            prefs
                .get_mut(name)
                .unwrap()
                .insert(day.to_string(), p);
        }
    }

    println!("\nCollected preferences (JSON):");
    let json = serde_json::to_string_pretty(&prefs).expect("serialize");
    println!("{json}");
}