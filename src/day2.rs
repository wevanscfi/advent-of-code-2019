use std::env;
use std::fs;

struct Config {
  args: Vec<String>,
  command: String,
}

fn parse_config() -> Config {
    let mut args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    args.drain(0..2);
    Config { args, command }
}

struct IntCodeProgram {
    pub state: Vec<usize>,
}

impl IntCodeProgram {
    fn new(state: Vec<usize>) -> IntCodeProgram {
         IntCodeProgram {
             state: state.clone()
         }
    }
    
    fn from_file(input: String) -> IntCodeProgram {
        let contents = fs::read_to_string(input)
            .expect("Something went wrong reading the file");

        let state = contents.trim().split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        return IntCodeProgram::new(state)
    }


    fn noun(&mut self, noun: usize) -> &mut IntCodeProgram {
        self.state[1] = noun;
        return self;
    }
    
    fn verb(&mut self, verb: usize) -> &mut IntCodeProgram {
        self.state[2] = verb;
        return self;
    }

    fn run(&mut self) -> usize {
        return IntCodeComputer::new(self).run();
    }
}

struct IntCodeComputer {
    pub state: Vec<usize>,
}

impl IntCodeComputer {
    fn new(program: &mut IntCodeProgram) -> IntCodeComputer {
        IntCodeComputer { 
            state: program.state.clone()
        }
    }

    fn eval_instruction<F: Fn(usize,usize) -> usize>(&mut self, i: usize, op: F) {
        let n1 = self.state[self.state[i+1]];
        let n2 = self.state[self.state[i+2]];
        let output = self.state[i+3];
        self.state[output] = op(n1,n2);
    }

    fn run(&mut self) -> usize {
        for i in (0..self.state.len()).step_by(4) {
            let verb = self.state[i];
            match verb {
                1 => self.eval_instruction(i, |a,b| a + b),
                2 => self.eval_instruction(i, |a,b| a * b),
                _ => break,
            } 
        }

        return self.state[0];
    }
}

fn solve(program: &mut IntCodeProgram, result: usize) -> (usize,usize) {
    for noun in 0..99 {
       for verb in 0..99 {
           let output = program.noun(noun).verb(verb).run();
           if output == result {
               return (noun, verb);
           }
       }
    }
    panic!("no results found!");
}

fn main() {
    let config = parse_config();

    match config.command.as_ref() {
        "exec" => {
            let path = config.args[0].clone();
            let noun = config.args[1].parse::<usize>().unwrap();
            let verb = config.args[2].parse::<usize>().unwrap();
            let mut program = IntCodeProgram::from_file(path);
            let result = program.noun(noun).verb(verb).run();
            println!("result: {}", result);
        }
        "solve" => {
            let path = config.args[0].clone();
            let result = config.args[1].parse::<usize>().unwrap();
            let mut program = IntCodeProgram::from_file(path);
            let (noun, verb) = solve(&mut program, result);
            println!("noun: {}, verb: {}", noun, verb);
        }
        _ => println!("Not a valid command"),
    }  
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_case() {
        let state = vec![1,0,0,0,99];
        let mut program = IntCodeProgram::new(state);
        assert_eq!(program.run(),2);
    }

    #[test]
    fn first_task() {
        let mut program = IntCodeProgram::from_file("inputs/day2.txt".to_string());
        assert_eq!(program.noun(12).verb(2).run(),4330636);
    }
    
    #[test]
    fn second_task() {
        let mut program = IntCodeProgram::from_file("inputs/day2.txt".to_string());
        let (noun,verb) = solve(&mut program, 19690720);
        assert_eq!((noun,verb),(60,86));
    }
}
