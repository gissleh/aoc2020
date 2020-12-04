use common::aoc::{load_input, run_many, print_time, print_result};

const ZERO: u32 = '0' as u8 as u32;

fn main() {
    let input = load_input("day04");

    let (coll, dur_parse) = run_many(1000, || PassportCollection::parse(&input));
    let (res_part1, dur_part1) = run_many(100000, || coll.count_noempty_fields());
    let (res_part2, dur_part2) = run_many(100000, || coll.count_valid());

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Total", dur_parse + dur_part1 + dur_part2);
}

struct PassportCollection {
    passports: Vec<Passport>,
}

impl PassportCollection {
    pub fn count_noempty_fields(&self) -> usize {
        let mut count = 0;

        for pass in self.passports.iter() {
            if pass.missing_fields() {
                continue;
            }

            count += 1;
        }

        count
    }

    pub fn count_valid(&self) -> usize {
        let mut count = 0;

        for pass in self.passports.iter() {
            if !pass.validate() {
                continue;
            }

            count += 1;
        }

        count
    }

    pub fn parse(input: &str) -> PassportCollection {
        let mut passports = Vec::with_capacity(100);

        let mut current_key = String::with_capacity(3);
        let mut current_value = String::with_capacity(16);
        let mut current_passport = Passport::new();
        let mut state = 0;
        let mut prev = 0 as char;
        for ch in input.chars() {
            if ch == '\n' && prev == '\n' {
                passports.push(current_passport);
                current_passport = Passport::new();
                state = 0;
                current_key.clear();
                current_value.clear();
            } else if ch == ' ' || ch == '\n' {
                state = 0;

                match current_key.as_str() {
                    "byr" => { current_passport.byr.push_str(&current_value) }
                    "iyr" => { current_passport.iyr.push_str(&current_value) }
                    "eyr" => { current_passport.eyr.push_str(&current_value) }
                    "hgt" => { current_passport.hgt.push_str(&current_value) }
                    "hcl" => { current_passport.hcl.push_str(&current_value) }
                    "ecl" => { current_passport.ecl.push_str(&current_value) }
                    "pid" => { current_passport.pid.push_str(&current_value) }
                    _ => {}
                }

                current_key.clear();
                current_value.clear();

            } else if ch == ':' {
                state = 1;
            } else {
                if state == 1 {
                    current_value.push(ch);
                } else {
                    current_key.push(ch);
                }
            }

            prev = ch;
        }

        passports.push(current_passport);

        PassportCollection{
            passports
        }
    }
}

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Passport {
    fn missing_fields(&self) -> bool{
        self.byr.is_empty() || self.iyr.is_empty() || self.eyr.is_empty()
            || self.hgt.is_empty() || self.hcl.is_empty() || self.ecl.is_empty()
            || self.pid.is_empty()
    }

    fn validate(&self) -> bool {
        if self.missing_fields() {
            return false;
        }

        if !valid_pid(&self.pid) {
            return false;
        }
        if !valid_color(&self.hcl) {
            return false;
        }
        if !one_of(&self.ecl, &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]) {
            return false
        }

        let (height, height_unit) = parse_int_unit(&self.hgt);
        let valid_height = match height_unit {
            "cm" => height >= 150 && height <= 193,
            "in" => height >= 59 && height <= 76,
            _ => false,
        };
        if !valid_height {
            return false;
        }

        let byr = parse_int(&self.byr);
        if byr < 1920 || byr > 2002 {
            return false;
        }
        let iyr = parse_int(&self.iyr);
        if iyr < 2010 || iyr > 2020 {
            return false;
        }
        let eyr = parse_int(&self.eyr);
        if eyr < 2020 || eyr > 2030 {
            return false;
        }

        true
    }

    fn new() -> Passport {
        Passport{
            byr: String::with_capacity(8),
            iyr: String::with_capacity(8),
            eyr: String::with_capacity(8),
            hgt: String::with_capacity(16),
            hcl: String::with_capacity(16),
            ecl: String::with_capacity(16),
            pid: String::with_capacity(16),
        }
    }
}

fn valid_color(s: &str) -> bool {
    if s.len() != 7 || !s.starts_with('#') {
        return false;
    }
    for ch in s[1..].chars() {
        if !((ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f')) {
            return false;
        }
    }

    true
}

fn valid_pid(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }

    for ch in s.chars() {
        if ch < '0' || ch > '9'  {
            return false;
        }
    }

    true
}

fn one_of(s: &str, values: &[&str]) -> bool {
    for v in values.iter() {
        if s == *v {
            return true;
        }
    }

    false
}

fn parse_int(s: &str) -> u32 {
    let mut result = 0;

    for ch in s.chars() {
        if ch >= '0' && ch <= '9' {
            result = (result * 10) + ((ch as u32) - ZERO);
        } else {
            break
        }
    }

    result
}

fn parse_int_unit(s: &str) -> (u32, &str) {
    let mut result = 0;
    let mut pos = 0usize;

    for ch in s.chars() {
        if ch >= '0' && ch <= '9' {
            result = (result * 10) + ((ch as u32) - ZERO);
        } else {
            return (result, &s[pos..])
        }

        pos += 1;
    }

    (result, &s[s.len()..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let coll = PassportCollection::parse("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
");
        assert_eq!(coll.count_valid(), 4);

        let coll = PassportCollection::parse("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
");
        assert_eq!(coll.count_valid(), 0);
    }
}