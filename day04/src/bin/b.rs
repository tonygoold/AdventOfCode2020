use std::io::{BufRead,BufReader};
use std::fs::File;
use regex::Regex;
use lazy_static::lazy_static;

fn in_range(s: &str, min: usize, max: usize) -> bool {
  match usize::from_str_radix(s, 10) {
    Ok(x) => x >= min && x <= max,
    _ => false
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
  cid: String,
}

impl Passport {
  fn valid(&self) -> bool {
    lazy_static! {
      static ref HGT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$")
        .expect("Failed to compile hgt regex");
      static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$")
        .expect("Failed to compile hcl regex");
      static ref PID_RE: Regex = Regex::new(r"^\d{9}$")
        .expect("Failed to compile pid regex");
    }

    if !in_range(&self.byr, 1920, 2002) { return false; }
    if !in_range(&self.iyr, 2010, 2020) { return false; }
    if !in_range(&self.eyr, 2020, 2030) { return false; }

    match HGT_RE.captures(&self.hgt) {
      Some(cap) => {
        if &cap[2] == "cm" {
          if !in_range(&cap[1], 150, 193) { return false; }
        } else {
          if !in_range(&cap[1], 59, 76) { return false; }
        }
      },
      None => { return false; }
    }

    if !HCL_RE.is_match(&self.hcl) { return false; }

    match self.ecl.as_str() {
      "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {},
      _ => { return false; }
    }

    PID_RE.is_match(&self.pid)
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
