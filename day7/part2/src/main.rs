use permutohedron::LexicalPermutation;
use std::env;
use std::fs;

mod intcode;

fn tokenize(filepath: &String) -> Vec<i64> {
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    return contents
        .split(',')
        .map(|input| input.parse().unwrap())
        .collect();
}

fn create_amps(phases: &[i64], program: &Vec<i64>) -> Vec<intcode::Machine> {
    let mut amps: Vec<intcode::Machine> =  Vec::new();

    let mut amp1 = intcode::create_new_machine(program.clone());
    amp1.push_input(phases[0]);
    amp1.push_input(0);
    amps.push(amp1);

    let mut amp2 = intcode::create_new_machine(program.clone());
    amp2.push_input(phases[1]);
    amps.push(amp2);

    let mut amp3 = intcode::create_new_machine(program.clone());
    amp3.push_input(phases[2]);
    amps.push(amp3);

    let mut amp4 = intcode::create_new_machine(program.clone());
    amp4.push_input(phases[3]);
    amps.push(amp4);

    let mut amp5 = intcode::create_new_machine(program.clone());
    amp5.push_input(phases[4]);
    amps.push(amp5);

    return amps;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Provide filename");
        return;
    }
    let filepath = &args[1];
    let mut max: i64 = 0;
    let mut phases = [5, 6, 7, 8, 9];
    let mut permutations = Vec::new();

    let program = tokenize(filepath);

    loop {
        permutations.push(phases.to_vec());
        if !phases.next_permutation() {
            break;
        }
        let mut amps: Vec<intcode::Machine> = create_amps(&phases, &program);

        loop {
            for i in 0..5 {
                amps[i].execute_program();

                let output = amps[i].pop_output().unwrap();
                if i == 4 {
                    amps[0].push_input(output);
                    if output > max {
                        max = output;
                    }
                    continue;
                }
                amps[i + 1].push_input(output);
            }
            if amps[4].is_finished() {
                break;
            }
        }
    }
    println!("{}", max);
}
