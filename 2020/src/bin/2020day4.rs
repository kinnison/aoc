use aoc2020::*;

struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    fn load_all(input: &str) -> Vec<Self> {
        let mut ret = Vec::new();
        let mut this = Self::new();
        for l in input.lines() {
            if l.is_empty() {
                ret.push(this);
                this = Self::new();
            } else {
                this.accumulate(l);
            }
        }
        ret.push(this);
        ret
    }

    fn accumulate(&mut self, line: &str) {
        for kv in line.split_ascii_whitespace() {
            if let Some(colon) = kv.find(':') {
                self.fields
                    .insert(kv[..colon].to_string(), kv[colon + 1..].to_string());
            }
        }
    }

    fn validate1(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .copied()
            .all(|e| self.fields.contains_key(e))
    }

    fn field(&self, f: &str) -> &str {
        self.fields.get(f).unwrap()
    }

    fn intfield(&self, f: &str) -> Option<usize> {
        let fc = self.field(f);
        fc.parse().ok()
    }

    fn validate2(&self) -> bool {
        if !self.validate1() {
            return false;
        }
        // Validation rules
        // byr 1920-2002
        if let Some(byr) = self.intfield("byr") {
            if byr < 1920 || byr > 2002 {
                return false;
            }
        } else {
            return false;
        }
        // iyr 2010-2020
        if let Some(iyr) = self.intfield("iyr") {
            if iyr < 2010 || iyr > 2020 {
                return false;
            }
        } else {
            return false;
        }
        // eyr 2020-2030
        if let Some(eyr) = self.intfield("eyr") {
            if eyr < 2020 || eyr > 2030 {
                return false;
            }
        } else {
            return false;
        }
        // hgt 150-193cm or 59-76in
        {
            let hgt = self.field("hgt");
            #[allow(clippy::never_loop)]
            if !loop {
                if let Some(cm) = hgt.strip_suffix("cm") {
                    if let Ok(cm) = cm.parse::<usize>() {
                        if cm >= 150 && cm <= 193 {
                            break true;
                        }
                    }
                } else if let Some(inch) = hgt.strip_suffix("in") {
                    if let Ok(inch) = inch.parse::<usize>() {
                        if inch >= 59 && inch <= 76 {
                            break true;
                        }
                    }
                }
                break false;
            } {
                return false;
            }
        }
        // hcl #hexhex
        {
            let hcl = self.field("hcl");
            if let Some(rest) = hcl.strip_prefix('#') {
                if rest.len() != 6 || !rest.chars().all(|c| "0123456789abcdef".contains(c)) {
                    return false;
                }
            } else {
                return false;
            }
        }
        // ecl amb blu brn gry grn hzl oth
        {
            let ecl = self.field("ecl");
            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .any(|&s| s == ecl)
            {
                return false;
            }
        }
        // pid 9 digits, any
        {
            let pid = self.field("pid");
            if pid.len() != 9 || !pid.chars().all(|c| "0123456789".contains(c)) {
                return false;
            }
        }
        true
    }
}

fn part1(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.validate1()).count()
}

fn part2(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.validate2()).count()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

    const INVALID_INPUT: &str = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;

    const VALID_INPUT: &str = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;

    #[test]
    fn testcase1() {
        let input = Passport::load_all(TEST_INPUT);
        assert_eq!(input.iter().filter(|p| p.validate1()).count(), 2);
    }

    #[test]
    fn testcase2() {
        let invalids = Passport::load_all(INVALID_INPUT);
        let valids = Passport::load_all(VALID_INPUT);
        assert_eq!(invalids.iter().filter(|p| p.validate2()).count(), 0);
        assert_eq!(
            valids.iter().filter(|p| p.validate2()).count(),
            valids.len()
        );
    }
}

fn main() -> Result<()> {
    let input: String = read_input(4)?;
    let input = Passport::load_all(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
