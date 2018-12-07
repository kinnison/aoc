use aoc2018::*;

#[derive(Debug, ParseByRegex)]
#[regex = r"^Step (?P<before>.) must be finished before step (?P<after>.) can begin\.$"]
struct Dependency {
    before: char,
    after: char,
}

static TEST_INPUT: &str = r"
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";

fn part1(input: &[Dependency]) -> Result<String> {
    let mut steps_done: HashSet<char> = HashSet::new();
    let mut ret: String = String::new();
    let mut steps_left: HashSet<char> = HashSet::new();
    for dep in input {
        steps_left.insert(dep.before);
        steps_left.insert(dep.after);
    }
    let mut steps_todo: Vec<char> = steps_left.iter().map(|&c| c).collect();
    steps_todo.sort();

    while !steps_todo.is_empty() {
        // In each iteration, try and find a step which we can do,
        // if the step is earliest, alphabetically, do it.  For that
        // we do the steps in alphabetical order and do the first which
        // we can each time.
        let mut step_todo = None;
        let mut step_index = 0;
        'step: for (i, step) in steps_todo.iter().enumerate() {
            for dep in input {
                if dep.after == *step && !steps_done.contains(&dep.before) {
                    // this is a dependency stating we cannot run yet
                    continue 'step;
                }
            }
            // Nothing says we can't run, so let's run
            step_todo = Some(*step);
            step_index = i;
            break;
        }
        let step_todo = step_todo.ok_or("Unable to find a step to run!")?;
        // We're going to run step_todo so let's do so...
        steps_todo.remove(step_index);
        ret.push(step_todo);
        steps_done.insert(step_todo);
    }

    Ok(ret)
}

fn part2(input: &[Dependency], worker_count: usize, overhead: usize) -> Result<(String, usize)> {
    let mut steps_done: HashSet<char> = HashSet::new();
    let mut ret: String = String::new();
    let mut steps_left: HashSet<char> = HashSet::new();
    for dep in input {
        steps_left.insert(dep.before);
        steps_left.insert(dep.after);
    }
    let mut steps_todo: Vec<char> = steps_left.iter().map(|&c| c).collect();
    steps_todo.sort();
    let mut workers: Vec<(char, usize)> = Vec::new();
    workers.resize(worker_count, ('A', 0));
    let mut working: usize = 0;
    let mut time_passed: usize = 0;

    'stepping: while !steps_todo.is_empty() {
        // In each iteration, try and find a step which we can do,
        // if the step is earliest, alphabetically, do it.  For that
        // we do the steps in alphabetical order and do the first which
        // we can each time.
        let mut step_todo = None;
        let mut step_index = 0;
        'step: for (i, step) in steps_todo.iter().enumerate() {
            for dep in input {
                if dep.after == *step && !steps_done.contains(&dep.before) {
                    // this is a dependency stating we cannot run yet
                    continue 'step;
                }
            }
            // Nothing says we can't run, so let's run
            step_todo = Some(*step);
            step_index = i;
            break;
        }
        // If we do not have a step to do, tick time until at least one worker
        // finishes work, then try again.  If we have a step to do but no worker
        // spare to do it, also tick time...
        if step_todo.is_none() || working == worker_count {
            loop {
                for worker in workers.iter_mut() {
                    if worker.1 > 0 {
                        worker.1 -= 1;
                        if worker.1 == 0 {
                            // this worker has finished work, report such
                            ret.push(worker.0);
                            steps_done.insert(worker.0);
                            working -= 1;
                            time_passed += 1;
                            continue 'stepping;
                        }
                    }
                }
            }
        }
        // We've reached here with work to do and at least one free worker
        // so assign the work.
        steps_todo.remove(step_index);
        for worker in workers.iter_mut() {
            if worker.1 == 0 {
                worker.0 = step_todo.ok_or("WTF?")?;
                worker.1 = overhead + (((worker.0 as u8) - (b'A' as u8)) as usize) + 1;
            }
        }
    }

    // We have reached the end, so tick the workers down
    while working > 0 {
        for worker in workers.iter_mut() {
            if worker.1 > 0 {
                worker.1 -= 1;
                if worker.1 == 0 {
                    ret.push(worker.0);
                    steps_done.insert(worker.0);
                    working -= 1;
                }
            }
        }
    }

    Ok((ret, time_passed))
}

fn main() -> Result<()> {
    let test_input: Vec<Dependency> = input_as_vec(TEST_INPUT)?;
    let input: Vec<Dependency> = read_input_as_vec(7)?;
    println!("Test 1: {}", part1(&test_input)?);
    println!("Part 1: {}", part1(&input)?);
    println!("Test 2: {:?}", part2(&test_input, 2, 0)?);
    Ok(())
}
