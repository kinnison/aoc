use aoc2019::*;

#[allow(dead_code)]
fn gather_output(vm: &mut intcode::VM) -> Result<Vec<String>> {
    let mut out = String::new();
    while let intcode::VMState::GaveOutput(c) = vm.interpreter_step(None)? {
        out.push(c as u8 as char);
    }
    Ok(out.lines().map(String::from).collect())
}

#[allow(dead_code)]
fn parse_room(vm: &mut intcode::VM) -> Result<(String, Vec<Direction>, Vec<String>)> {
    // We run this VM until it prompts for input, and then analyse its output
    let mut output = gather_output(vm)?;
    assert!(output.len() >= 5);
    // The first line of a room output is the room's name
    let name = loop {
        let line = output.remove(0);
        if line.starts_with("== ") {
            break line;
        }
    };
    assert!(name.starts_with("== "));
    assert!(name.ends_with(" =="));
    let name = name[3..name.len() - 3].to_string();
    // Now let's hunt for the door list
    while !output.is_empty() {
        let line = output.remove(0);
        if line == "Doors here lead:" {
            break;
        }
    }
    let mut doors = Vec::new();
    while !output.is_empty() {
        let line = output.remove(0);
        if line.starts_with("- ") {
            let dir = Direction::from_str(&line[2..])?;
            doors.push(dir);
        } else {
            break; // Finished doors
        }
    }
    let mut items = Vec::new();
    // Hunt for the items line
    while !output.is_empty() {
        let line = output.remove(0);
        if line == "Command?" {
            break;
        }
        if line == "Items here:" {
            while !output.is_empty() {
                let line = output.remove(0);
                if line.starts_with("- ") {
                    items.push(line[2..].to_string());
                } else {
                    break;
                }
            }
        }
    }
    // And give it all back
    Ok((name, doors, items))
}

//#[derive(Debug, Hash, PartialEq, Eq)]
//struct BotPos {
//    room: String,
//    items: HashSet<String>,
//}

fn main() -> Result<()> {
    let input = read_input(25)?;
    let mut input = intcode::VM::from_str(&input)?;

    //if std::env::args().nth(1) == Some("interactive".to_string()) {
    input.run_ascii_machine()?;
    //    return Ok(());
    //}

    //    let (first_room, first_doors, first_items) = parse_room(&mut input)?;
    //
    //    let bots: HashMap<BotPos, intcode::VM> = HashMap::new();
    //
    //    bots.insert(
    //        BotPos {
    //            room: first_room.clone(),
    //            items: HashSet::new(),
    //        },
    //        input.clone(),
    //    );
    //
    //    loop {
    //        // We are in a room, there are potentially items, and potentially
    //        // exits.  First thing we do is decide if we're going to try picking
    //        // up any of the items
    //    }

    Ok(())
}
