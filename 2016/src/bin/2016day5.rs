use md5::{Digest, Md5};

fn problem1(label: &String) -> String {
    let mut hash = Md5::new();
    let mut ret = String::new();
    let mut idx: u64 = 0;
    let chs = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    for _ in 0..8 {
        loop {
            hash.reset();
            hash.update(label.as_bytes());
            hash.update(idx.to_string().as_bytes());
            let output = hash.finalize_reset();
            let five = (output[0] as u32) + (output[1] as u32) + ((output[2] as u32) >> 4);
            idx += 1;
            if five == 0 {
                let n = (output[2] & 0xf) as usize;
                ret.push(chs[n]);
                println!("Found {} at {}", chs[n], idx - 1);
                break;
            }
        }
    }
    return ret;
}

fn problem2(label: &String) -> String {
    let mut hash = Md5::new();
    let mut ret = ['_', '_', '_', '_', '_', '_', '_', '_'];
    let mut done = 0;
    let mut idx: u64 = 0;
    let chs = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    while done < 8 {
        loop {
            hash.reset();
            hash.update(label.as_bytes());
            hash.update(idx.to_string().as_bytes());
            let output = hash.finalize_reset();
            let five = (output[0] as u32) + (output[1] as u32) + ((output[2] as u32) >> 4);
            idx += 1;
            if five == 0 {
                let pos = (output[2] & 0xf) as usize;
                let val = (output[3] >> 4) as usize;
                if pos < 8 && ret[pos] == '_' {
                    println!("Found {} at {} going into {}", chs[val], idx - 1, pos);
                    if ret[pos] == '_' {
                        done += 1;
                    }
                    ret[pos] = chs[val];
                }
                break;
            }
        }
    }
    return ret.iter().cloned().collect::<String>();
}

fn main() {
    let puzzleinput = ("uqwqemis").to_string();
    //    println!("Test result is {}", problem1(&("abc".to_string())));
    println!("Result 1 is {}", problem1(&puzzleinput));
    //    println!("Test result is {}", problem2(&("abc".to_string())));
    println!("Result 2 is {}", problem2(&puzzleinput));
}
