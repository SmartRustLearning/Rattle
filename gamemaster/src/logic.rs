use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{Bytes, Read};
use std::str::from_utf8;

#[derive(Serialize, Deserialize, Debug)]
pub struct Exercise {
    libs: Vec<Vec<u8>>,
    tests: Vec<Vec<u8>>,
    tasks: Vec<Vec<u8>>,
    infos: Vec<Vec<u8>>,
    config: Config,
}

impl Exercise {
    pub fn from_path(path: &str) -> Self {
        let ex_dir = fs::read_dir(std::path::Path::new(path)).expect("Exercise directory does not exist.");
        let mut libs = vec![];
        let mut tests = vec![];
        let mut tasks = vec![];
        let mut infos = vec![];
        let mut config = None;
        /* let load_files = |s| {
            ex_dir
                .filter(|e| e.expect("TODO error").file_name().to_str().unwrap().starts_with(s))
                .map(|e| fs::read(e.unwrap().path()).expect("problem reading file."))
        }; */
        for e in ex_dir {
            let e = e.expect("TODO err");
            let text = fs::read(&e.path()).expect("problem reading file.");
            let filename = e.file_name().to_str().unwrap().to_owned();
            match filename.split(|s: char| s.is_ascii_digit() || s == '.').nth(0).unwrap() {
                "lib"  =>  libs.push( (text, filename[3..filename.len()-3].parse::<u8>().expect("Error parsing digit.")) ),
                "test" => tests.push( (text, filename[4..filename.len()-3].parse::<u8>().expect("Error parsing digit.")) ),
                "task" => tasks.push( (text, filename[4..filename.len()-3].parse::<u8>().expect("Error parsing digit.")) ),
                "info" => infos.push( (text, filename[4..filename.len()-3].parse::<u8>().expect("Error parsing digit.")) ),
                "config" => {
                    config = Some(serde_json::from_slice( &text ).expect("Error parsing config file."))
                },
                _ => unreachable!()
            }
        }
        libs.sort_by_key(|(_, k)| *k);
        tests.sort_by_key(|(_, k)| *k);
        tasks.sort_by_key(|(_, k)| *k);
        infos.sort_by_key(|(_, k)| *k);
        Self {
            libs: libs.into_iter().map(|(x, _)| x).collect(),
            tests: tests.into_iter().map(|(x, _)| x).collect(),
            tasks: tasks.into_iter().map(|(x, _)| x).collect(),
            infos: infos.into_iter().map(|(x, _)| x).collect(),
            config: config.expect("no config found")
        }
    }
}

#[derive(Debug)]
pub struct Task {
    lib: Vec<u8>,
    test: Vec<u8>,
    task: Vec<u8>,
    info: Vec<u8>,
}

impl Task {
    pub fn get_code_with(&self, code: String) -> String {
        format!("{}\n{}\n\n{}", from_utf8(&self.lib).unwrap(), code, from_utf8(&self.task).unwrap())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    hints: Vec<String>,
}

#[derive(Debug)]
enum MatchState {
    Waiting,
    InProgress,
    Finished,
}

#[derive(Debug)]
pub struct Match {
    state: MatchState,
    players: (Option<Player>, Option<Player>),
    exercise: Exercise,
    round: (Option<PlayerRound>, Option<PlayerRound>),
    past_rounds: Vec<(PlayerRound, PlayerRound)>,
}

impl Match {
    pub fn new(p: String, ex: Exercise) -> Self {
        Self { state: MatchState::Waiting, players: (Some(Player { name: p, score: 500 }), None), exercise: ex, round: (None, None), past_rounds: vec![] }
    }

    pub fn join(&mut self, p: String) {
        self.state = MatchState::InProgress;
        self.players.1 = Some(Player { name: p, score: 560 });
    }

    pub fn next_round(&self) -> Task {
        let nr = self.past_rounds.len();
        Task {
            lib: self.exercise.libs[nr].clone(),
            test: self.exercise.tests[nr].clone(),
            task: self.exercise.tasks[nr].clone(),
            info: self.exercise.infos[nr].clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Player {
    name: String,
    score: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlayerRound {
    time: usize,
    code: Vec<u8>,
}
