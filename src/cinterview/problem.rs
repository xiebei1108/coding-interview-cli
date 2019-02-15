use std;
use std::io;
use std::io::{stdin, Write};

use std::env::current_dir;

use std::fs;
use std::path::PathBuf;

use std::collections::{HashMap};

extern crate dirs;
use dirs::home_dir;

extern crate termion;
use termion::color;

use crate::cinterview::crawler::*;
use crate::cinterview::error::*;

pub type ProblemList = Vec<Problem>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    /// The number of problem. Start from 0 to 65 for now.
    pub num: u32,

    /// The name of problem
    pub name: String,

    /// The content of problem
    pub content: String,

    /// The map of language name to it's coding template
    pub templates: HashMap<String, String>,

    /// Whether have pass this problem. If use choose login mode, it's based on the remote status.
    /// Otherwise, it's loaded from local log.
    pub passed: bool,
}

impl Problem {
    fn save(&self, path: &PathBuf) -> GenResult<()> {
        // save detail

        Ok(())
    }

}

/// TODO, support login
// pub fn list_problems_login() {}

pub fn list_problems_unlogin() {
    let local_root = home_dir().unwrap().join(".coding-interview");
    let problem_path = local_root.join("problem.json");
    ensure_local_data(&local_root, &problem_path);
    print_problem_infos(read_local_problems(&problem_path).expect("read local problem fail"));
}

pub fn clean_problems() {
    let mut s = String::new();
    println!("ATTENTION! Really clean? [y/n]");
    stdin().read_line(&mut s).expect("invalid input");
    match s.as_str().trim() {
        "y" | "Y" => {
            let root = home_dir().unwrap().join(".coding-interview");
            if root.exists() {
                fs::remove_dir_all(root).expect("remove local root fail");
            }
            println!("ok");
        }
        "n" | "N" => {}
        _ => println!("invalid input"),
    };
}

pub fn init_problems() {
    let local_root = home_dir().unwrap().join(".coding-interview");
    ensure_local_data(&local_root, &local_root.join("problem.json"));
    init_projects().expect("init projects fail");
    println!("\n 😘😘😘\tinit ok...");
}

fn init_projects() -> GenResult<()> {
    let local_root = home_dir().unwrap().join(".coding-interview/problem.json");
    let root = current_dir()?
        .join("coding-inverview/");
    if root.exists() {
        println!("already exist!");
        return Ok(());
    }

    for x in read_local_problems(&local_root)? {
        let dir_name = format!("{}_{}", x.num, x.name);
        x.save(&root.join(dir_name))?
    }
    Ok(())
}

fn read_local_problems(path: &PathBuf) -> io::Result<ProblemList> {
    match fs::read_to_string(path) {
        Ok(data) => {
            let problems: ProblemList = serde_json::from_str(data.as_str())?;
            Ok(problems)
        }
        Err(_e) => Err(_e),
    }
}

fn ensure_local_data(root: &PathBuf, problem_path: &PathBuf) {
    ensure_dir(root).expect("ensure dir fail!");
    if !problem_path.exists() {
        update_problems(get_problems()).expect("update problems ! ❌");
    }
}

fn print_problem_infos(problems: ProblemList) {
    for x in problems {
        let emoji = if x.passed {
            print!("{}", color::Fg(color::Green));
            "👍🏻"
        } else {
            print!("{}", color::Fg(color::Red));
            "😡"
        };
        println!(
            "{}\t {}[{}] \t{}",
            emoji,
            color::Fg(color::Red),
            x.num,
            x.name
        );
    }
    println!("{}[passed] [num] [problem-title]", color::Fg(color::Reset));
}

fn ensure_dir(path: &PathBuf) -> GenResult<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn ensure_open(path: &PathBuf) -> GenResult<fs::File> {
    let result = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
    Ok(result)
}

fn update_problems(problems: ProblemList) -> GenResult<()> {
    let json_str = serde_json::to_string(&problems)?;
    let problem_path = home_dir().unwrap().join(".coding-interview/problem.json");
    let mut file = ensure_open(&problem_path)?;
    file.write_all(json_str.as_bytes())?;
    Ok(())
}
