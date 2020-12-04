use regex::Regex;
use crate::utils;


pub fn run(extra_star: bool) {
    let passports = parse_input();

    let mut valid = 0;
    for passport in passports {
        if passport.is_valid(extra_star) {
            valid += 1;
        }
    }

    println!("{}", valid);
}

#[derive(Default)]
struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
        fn is_valid(&self, extra_star: bool) -> bool {
            if extra_star {
                self.byr_valid() && self.iyr_valid() && self.eyr_valid()
                    && self.hgt_valid() && self.hcl_valid() && self.ecl_valid()
                    && self.pid_valid()
            } else {
                self.byr.is_some() && self.iyr.is_some() && self.eyr.is_some()
                    && self.hgt.is_some() && self.hcl.is_some() && self.ecl.is_some()
                    && self.pid.is_some()
            }
        }

        fn byr_valid(&self) -> bool {
            match self.byr {
                Some(v) => v >= 1920 && v <= 2002,
                None => false
            }
        }

        fn iyr_valid(&self) -> bool {
            match self.iyr {
                Some(v) => v >= 2010 && v <= 2020,
                None => false
            }
        }

        fn eyr_valid(&self) -> bool {
            match self.eyr {
                Some(v) => v >= 2020 && v <= 2030,
                None => false
            }
        }

        fn hgt_valid(&self) -> bool {
            match &self.hgt {
                Some(v) => {
                    let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();

                    match re.captures(&v) {
                        Some(x) => {
                            let height: u8 = x.get(1).unwrap().as_str().parse().unwrap();
                            let system = x.get(2).unwrap().as_str();
                            match system {
                                "cm" => {
                                    height >= 150 && height <= 193
                                },
                                "in" => {
                                    height >= 59 && height <= 76
                                },
                                &_ => {
                                    panic!("Unknown height system");
                                }
                            }
                        },
                        None => false
                    }
                },
                None => false
            }
        }

        fn hcl_valid(&self) -> bool {
            match &self.hcl {
                Some(v) => {
                    let re = Regex::new(r"^#[{a-f}\d}]+$").unwrap();

                    re.captures(&v).is_some()
                },
                None => false
            }
        }

        fn ecl_valid(&self) -> bool {
            match &self.ecl {
                Some(v) => {
                    v == "amb" || v == "blu" || v == "brn"
                        || v == "gry" || v == "grn" || v == "hzl"
                        || v == "oth"
                },
                None => false
            }
        }

        fn pid_valid(&self) -> bool {
            match &self.pid {
                Some(v) => {
                    let re = Regex::new(r"^\d{9}$").unwrap();

                    re.captures(&v).is_some()
                },
                None => false
            }
        }

        fn default() -> Self {
            Passport {
                byr: None,
                iyr: None,
                eyr: None,
                hgt: None,
                hcl: None,
                ecl: None,
                pid: None
            }
        }
    }


    fn parse_input() -> Vec<Passport> {
        let lines = utils::read_lines("inputs/4.txt");

        let mut indx: usize = 0;

        let mut passports: Vec<Passport> = Vec::new();
        let mut p = Passport::default();
        loop {
            if indx == lines.len() {
                break;
            }

            let line = &lines[indx];
            indx += 1;

            if line.is_empty() {
                // New passports
                passports.push(p);
                p = Passport::default();
                continue;
            }

            for part in line.split(" ") {
                let creds: Vec<&str> = part.split(":").collect();

                match creds[0] {
                    "byr" => p.byr = Some(creds[1].parse::<u16>().unwrap()),
                    "eyr" => p.eyr = Some(creds[1].parse::<u16>().unwrap()),
                    "hgt" => p.hgt = Some(creds[1].to_string()),
                    "pid" => p.pid = Some(creds[1].to_string()),
                    "hcl" => p.hcl = Some(creds[1].to_string()),
                    "iyr" => p.iyr = Some(creds[1].parse::<u16>().unwrap()),
                    "ecl" => p.ecl = Some(creds[1].to_string()),
                    "cid" | &_ => ()
                };
            }
        }

        passports
    }


