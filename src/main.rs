use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};

use rand::seq::SliceRandom;
use rand::thread_rng;

trait Rule {
    // returns the original string to replace
    fn original(&self) -> Cow<str>;
    // returns the string to be substituted.
    // Allowed to have side-effects and should only be called once for each substitution.
    fn substitution(&self) -> Cow<str>;
}

// substitutes 'original' for 'substitute'
#[derive(Clone, Debug)]
struct Substitution {
    original: Box<str>,
    substitute: Box<str>,
}

impl Substitution {
    fn new(original: &str, substitute: &str) -> Self {
        Substitution {
            original: original.to_string().into_boxed_str(),
            substitute: substitute.to_string().into_boxed_str(),
        }
    }
}

impl Rule for Substitution {
    fn original(&self) -> Cow<str> {
        Cow::Borrowed(&self.original)
    }

    fn substitution(&self) -> Cow<str> {
        Cow::Borrowed(&self.substitute)
    }
}

// replaces 'original' with line from stdin
#[derive(Clone, Debug)]
struct Input {
    original: Box<str>,
}

impl Input {
    fn new(original: &str) -> Self {
        Input {
            original: original.to_string().into_boxed_str(),
        }
    }
}

impl Rule for Input {
    fn original(&self) -> Cow<str> {
        Cow::Borrowed(&self.original)
    }

    fn substitution(&self) -> Cow<str> {
        let mut out = String::new();
        stdin().read_line(&mut out).unwrap();
        out = out[..out.len() - 1].to_string();
        Cow::Owned(out)
    }
}

// replaces 'original' with the null string and outputs 'output' to stdout
#[derive(Clone, Debug)]
struct Output {
    original: Box<str>,
    output: Box<str>,
}

impl Output {
    fn new(original: &str, output: &str) -> Self {
        let mut output = output;
        if output == "" {
            output = "\n";
        }

        Output {
            original: original.to_string().into_boxed_str(),
            output: output.to_string().into_boxed_str(),
        }
    }
}

impl Rule for Output {
    fn original(&self) -> Cow<str> {
        Cow::Borrowed(&self.original)
    }

    fn substitution(&self) -> Cow<str> {
        stdout().lock().write_all(&self.output.as_bytes()).unwrap();
        Cow::Owned("".to_string())
    }
}

fn main() -> Result<(), std::io::Error> {
    let file_name = env::args().nth(1).expect("Missing program file argument!");
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);

    let rule_list = parse_rules(&mut buf_reader)?;

    let mut initial_state = String::new();
    buf_reader.read_to_string(&mut initial_state)?;
    initial_state = initial_state.replace("\n", "");

    run_program(rule_list, initial_state);

    Ok(())
}

// parses and returns list of rules, leaving the buffer at the first line after the list terminator
fn parse_rules(buf_reader: &mut BufReader<File>) -> Result<Box<[Box<dyn Rule>]>, std::io::Error> {
    let mut out: Vec<Box<dyn Rule>> = vec![];
    loop {
        let mut next_line = String::new();
        if buf_reader.read_line(&mut next_line).unwrap() == 0 {
            panic!("Invalid input file!");
        };
        next_line = next_line[..next_line.len() - 1].to_string();

        if let Some((original, substitute)) = get_rule_params(&next_line) {
            if original.trim() == "" && substitute.trim() == "" {
                // reached end of rule list
                break;
            } else if original.trim() == "" && substitute.trim() != "" {
                panic!("Invalid syntax!");
            } else if substitute == ":::" {
                out.push(Box::new(Input::new(original)));
            } else if substitute.starts_with('~') {
                out.push(Box::new(Output::new(original, &substitute[1..])));
            } else {
                out.push(Box::new(Substitution::new(original, substitute)));
            }
        }
    }
    Ok(out.into_boxed_slice())
}

// returns the rule parameters as '(original, substitute)' or None if not a valid rule
fn get_rule_params(line: &str) -> Option<(&str, &str)> {
    if let Some(i) = line.find("::=") {
        let (head, tail) = line.split_at(i);
        Some((head, &tail[3..]))
    } else {
        None
    }
}

// runs Thue program using collected 'rule_list' and 'initial_state'
fn run_program(mut rule_list: Box<[Box<dyn Rule>]>, initial_state: String) {
    let mut rng = thread_rng();
    let mut state = initial_state;

    let mut running = true;

    while running {
        rule_list.shuffle(&mut rng);

        running = false;
        for rule in rule_list.iter() {
            let original = rule.original();

            if let Some(index) = state
                .match_indices(original.as_ref())
                .map(|(i, _)| i)
                .collect::<Vec<usize>>()
                .choose(&mut rng)
            {
                running = true;
                state.replace_range(*index..index + original.len(), &rule.substitution());
                break;
            }
        }
    }
    print!("\n");
}
