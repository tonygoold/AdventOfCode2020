use std::io::{BufRead,BufReader};
use std::fs::File;
use regex::Regex;

struct Passport {
  byr: String,
  iyr: String,
  eyr: String,
  hgt: String,
  hcl: String,
  ecl: String,
  pid: String,
  cid: String,
}

impl Passport {
  fn valid(&self) -> bool {
    self.byr.len() > 0 && self.iyr.len() > 0 && self.eyr.len() > 0 &&
    self.hgt.len() > 0 && self.hcl.len() > 0 && self.ecl.len() > 0 &&
    self.pid.len() > 0
  }
  
  fn add(&mut self, field: &str, value: &str) {
    match field {
      "byr" => { self.byr = String::from(value); },
      "iyr" => { self.iyr = String::from(value); },
      "eyr" => { self.eyr = String::from(value); },
      "hgt" => { self.hgt = String::from(value); },
      "hcl" => { self.hcl = String::from(value); },
      "ecl" => { self.ecl = String::from(value); },
      "pid" => { self.pid = String::from(value); },
      "cid" => { self.cid = String::from(value); },
      _ => panic!("Unexpected field")
    }
  }

  fn new() -> Passport {
    Passport {
      byr: String::new(),
      iyr: String::new(),
      eyr: String::new(),
      hgt: String::new(),
      hcl: String::new(),
      ecl: String::new(),
      pid: String::new(),
      cid: String::new()
    }
  }
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let lines = reader.lines().into_iter().map(|x| x.unwrap());
  let re = Regex::new(r"(\w+):(\S+)").expect("Failed to compile regex");

  let mut passport = Passport::new();
  let mut passports = Vec::new();
  for line in lines {
    if &line == "" {
      passports.push(passport);
      passport = Passport::new();
      continue;
    }
    for cap in re.captures_iter(&line) {
      passport.add(&cap[1], &cap[2]);
    }
  }
  passports.push(passport);

  let valid = passports.iter().filter(|x| x.valid()).count();

  println!("{:?} valid passports", valid);
}
