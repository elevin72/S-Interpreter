use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

/*******SYNTAX**********/
// command :=   [label] <Inc> | [label] <Dec> | [label] <Jmp> | [label] <Nop>
// Inc :=       <varArrow> + 1
// Dec :=       <VarArrow> - 1
// Jmp :=       if <variable> != 0 GOTO <label>
// Nop :=       <VarArrow>
// VarArrow :=  variable <- variable //this isn't actually possible in a simple grammmer??
// label := \[letter[number]\]:
// variable := <'X' | 'Z'>[number] | 'Y'
// letter := [A-Wa-w]
// number := [0-9] | [0-9] number

struct Command {
    label: Option<String>,
    cmd_type: String,
    variable: Option<String>,
    dest_label: Option<String>,
}

impl FromStr for Command {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cmd) = parse_inc_dec(s) {
            return Ok(cmd);
        } else if let Some(cmd) = parse_jmp(s) {
            return Ok(cmd);
        } else if let Some(cmd) = parse_nop(s) {
            return Ok(cmd);
        } else {
            return Err("no valid command found");
        }
    }
}

fn parse_inc_dec(s: &str) -> Option<Command> {
    lazy_static! {
        static ref INC_DEC_RE: Regex = Regex::new(
            r"^\s*(\[[A-W][0-9]*\]:|)*\s*([X-Z][0-9]*|Y)\s*<-\s*([XZ][0-9]*|Y)\s*([+-])\s*1$"
        )
        .unwrap();
    }
    if let Some(caps) = INC_DEC_RE.captures(s) {
        if &caps[2] != &caps[3] {
            return None;
        }
        let label = caps
            .get(1)
            .and_then(|_| Some(strip_label(&caps[1]).to_string()));
        let variable = Some(caps[2].to_string());
        let cmd_type = match &caps[4] {
            "+" => "inc".to_string(),
            "-" => "dec".to_string(),
            _ => panic!(),
        };
        return Some(Command {
            label,
            cmd_type,
            variable,
            dest_label: None,
        });
    }
    return None;
}

fn parse_jmp(s: &str) -> Option<Command> {
    lazy_static! {
        static ref JMP_RE: Regex = Regex::new(
            r"^\s*(\[[A-W][0-9]*\]:|)*\s*if\s*([XZ][0-9]*|Y)\s*!=\s*0\s*GOTO\s*([A-W][0-9]*)$"
        )
        .unwrap();
    }
    if let Some(caps) = JMP_RE.captures(s) {
        let label = caps
            .get(1)
            .and_then(|_| Some(strip_label(&caps[1]).to_string()));
        let variable = Some(caps[2].to_string());
        let cmd_type = "jmp".to_string();
        let dest_label = Some(caps[3].to_string());
        return Some(Command {
            label,
            cmd_type,
            variable,
            dest_label,
        });
    }
    return None;
}

fn parse_nop(s: &str) -> Option<Command> {
    lazy_static! {
        static ref JMP_RE: Regex = Regex::new(r"^(\s*\[[A-W][0-9]*\]:)\s*$").unwrap();
    }
    if let Some(caps) = JMP_RE.captures(s) {
        let label = Some(strip_label(&caps[1]).to_string());
        let cmd_type = "nop".to_string();
        return Some(Command {
            label,
            cmd_type,
            variable: None,
            dest_label: None,
        });
    }
    return None;
}

fn strip_label(s: &str) -> &str {
    lazy_static! {
        static ref STRIP_LABEL_RE: Regex = Regex::new(r"\[([A-W][0-9]*)\]:").unwrap();
    }
    return STRIP_LABEL_RE.captures(s).unwrap().get(1).unwrap().as_str();
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    let program_file = File::open(&args[1]).expect("Input file not found");
    let program_lines: Vec<String> = io::BufReader::new(program_file)
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .filter(|line| line.chars().nth(0).unwrap() != '#')
        .collect();

    let parsed_commands: Vec<Command> = program_lines
        .iter()
        .enumerate()
        .map(|(i, line)| Command::from_str(line).expect(&format!(" parse error on line {}", i + 1)))
        .collect();

    let inputs: Vec<u32> = args[2..]
        .iter()
        .map(|i| -> u32 { i.parse().expect("All inputs must be natural numbers") })
        .collect();

    let mut variables: HashMap<String, u32> = HashMap::new();
    variables.insert("Y".to_string(), 0);
    for (i, input) in inputs.iter().enumerate() {
        let var = "X".to_string() + &(i + 1).to_string();
        variables.insert(var, *input);
    }

    for command in &parsed_commands {
        if let Some(var) = &command.variable {
            variables.entry(var.to_string()).or_insert(0);
        }
    }

    let mut program_counter = 0;
    loop {
        if program_counter >= parsed_commands.len() {
            println!("{}", variables["Y"]);
            return Ok(());
        }

        let Command {
            label: _,
            cmd_type,
            variable,
            dest_label,
        } = &parsed_commands[program_counter];

        match (cmd_type.as_str(), variable, dest_label) {
            ("inc", Some(v), _) => {
                *variables.get_mut(&v.to_string()).unwrap() += 1;
                program_counter += 1;
            }
            ("dec", Some(v), _) => {
                let mut value = variables[v];
                if value != 0 {
                    value -= 1;
                }
                *variables.get_mut(&v.to_string()).unwrap() = value;
                program_counter += 1;
            }
            ("jmp", Some(v), Some(d)) => {
                program_counter += 1;
                if variables[v] != 0 {
                    let mut i = 0;
                    loop {
                        if i == parsed_commands.len() - 1 {
                            println!("{}", variables["Y"]);
                            return Ok(());
                        }
                        if program_counter >= parsed_commands.len() {
                            program_counter = 0;
                        }
                        if let Some(l) = &parsed_commands[program_counter].label {
                            if l == d {
                                break;
                            }
                        }
                        program_counter += 1;
                        i += 1;
                    }
                }
            }
            ("nop", _, _) => {
                program_counter += 1;
            }
            (_, _, _) => {
                panic!("welp")
            }
        }
    }
}
