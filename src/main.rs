use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

// command := <Inc> | <Dec> | <Jmp> | <Nop>
// Inc := [label] 'inc' <variable>
// Dec := [label] 'dec' <variable>
// Jmp := [label] 'jmp' <variable> <label>
// Nop := [Label] 'nop'
// label := letter[number]
// variable := <'X' | 'Z'>[number] | 'Y'
// letter := [A-Wa-w]
// number := [0-9]
//

struct Command {
    label: Option<String>,
    cmd_type: String,
    variable: Option<String>,
    dest_label: Option<String>,
}

impl FromStr for Command {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_ascii_whitespace().collect();
        match tokens.len() {
            1 => Ok(Command {
                label: None,
                cmd_type: "nop".to_string(),
                variable: None,
                dest_label: None,
            }),
            2 => {
                if validate_label(tokens[0]) {
                    return Ok(Command {
                        label: Some(tokens[0].to_string()),
                        cmd_type: "nop".to_string(),
                        variable: None,
                        dest_label: None,
                    });
                }
                match (tokens[0], validate_variable(tokens[1])) {
                    ("inc" | "dec" | "jmp", true) => {
                        return Ok(Command {
                            label: None,
                            cmd_type: tokens[0].to_string(),
                            variable: Some(tokens[1].to_string()),
                            dest_label: None,
                        })
                    }
                    _ => Err("invalid command or variable"),
                }
            }
            3 => {
                if validate_label(tokens[0]) && validate_variable(tokens[2]) {
                    match tokens[1] {
                        "inc" | "dec" | "jmp" => {
                            return Ok(Command {
                                label: Some(tokens[0].to_string()),
                                cmd_type: tokens[1].to_string(),
                                variable: Some(tokens[1].to_string()),
                                dest_label: None,
                            })
                        }
                    _ => Err("invalid command or variable"),
                    }
                } else if tokens[0] == "jmp"
                    && validate_variable(tokens[1])
                    && validate_label(tokens[2])
                {
                    return Ok(Command {
                        label: None,
                        cmd_type: tokens[0].to_string(),
                        variable: Some(tokens[1].to_string()),
                        dest_label: Some(tokens[2].to_string()),
                    });
                } else {
                    return Err("invalid label, command or variable");
                }
            }
            4 => {
                if validate_label(tokens[0])
                    && tokens[1] == "jmp"
                    && validate_variable(tokens[2])
                    && validate_label(tokens[3])
                {
                    return Ok(Command {
                        label: Some(tokens[0].to_string()),
                        cmd_type: tokens[1].to_string(),
                        variable: Some(tokens[2].to_string()),
                        dest_label: Some(tokens[3].to_string()),
                    });
                } else {
                    return Err("invalid label, command or variable");
                }
            }
            _ => Err("invalid label, command or variable"),
        }
    }
}

fn validate_subscript(i: &str) -> bool {
    if i == "" {
        return true;
    } else {
        match i.parse::<u32>() {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

fn validate_label(s: &str) -> bool {
    let letter = s.chars().nth(0).unwrap();
    if letter.is_ascii_alphabetic() && validate_subscript(&s[1..]) {
        return match letter {
            'X' | 'Y' | 'Z' => false,
            _ => true,
        };
    }
    false
}

fn validate_variable(s: &str) -> bool {
    let letter = s.chars().nth(0).unwrap();
    if letter.is_ascii_alphabetic() && validate_subscript(&s[1..]) {
        return match letter {
            'X' | 'Y' | 'Z' => true,
            _ => false,
        };
    }
    false
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    let program_file = File::open(&args[1]).expect("Input file not found");
    let program_lines: Vec<String> = io::BufReader::new(program_file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let parsed_commands: Result<Vec<Command>, _> = program_lines
        .iter()
        .enumerate()
        .map(|(i, line)| Command::from_str(line).map_err(|e| format!("{} on line {}",e, i)))
        .collect();

    if let Err(e) = parsed_commands {
        println!("{}", e);
        return Ok(());
    }
    let commands = parsed_commands.unwrap();

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

    for command in &commands {
        if let Some(var) = &command.variable {
            variables.entry(var.to_string()).or_insert(0);
        }
    }

    let mut program_counter = 0;
    loop {
        if program_counter >= commands.len() {
            println!("{}", variables["Y"]);
            return Ok(());
        }
        let Command {
            label,
            cmd_type,
            variable,
            dest_label,
        } = &commands[program_counter];
        match (label, cmd_type.as_str(), variable, dest_label) {
            (_, "inc", Some(v), _) => {
                *variables.get_mut(&v.to_string()).unwrap() += 1;
                program_counter += 1;
            }
            (_, "dec", Some(v), _) => {
                let mut value = variables[v];
                if value != 0 {
                    value -= 1;
                }
                *variables.get_mut(&v.to_string()).unwrap() = value;
                program_counter += 1;
            }
            (_, "jmp", Some(v), Some(d)) => {
                program_counter += 1;
                if variables[v] != 0 {
                    let mut i = 0;
                    loop {
                        if i == commands.len() - 1 {
                            println!("{}", variables["Y"]);
                            return Ok(());
                        }
                        if program_counter >= commands.len() {
                            program_counter = 0;
                        }
                        if let Some(l) = &commands[program_counter].label {
                            if l == d {
                                break;
                            }
                        }
                        program_counter += 1;
                        i += 1;
                    }
                }
            }
            (_, "nop", _, _) => {
                program_counter += 1;
            }
            (_, _, _, _) => {
                panic!("welp")
            }
        }
    }
}
