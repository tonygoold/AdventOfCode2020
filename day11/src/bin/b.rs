use std::env;
use std::fmt::{self,Write};
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Seat {
  Floor,
  Unoccupied,
  Occupied,
}

impl fmt::Display for Seat {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_char(match self {
      Seat::Floor => '.',
      Seat::Unoccupied => 'L',
      Seat::Occupied => '#',
    })
  }
}

#[derive(Eq, PartialEq)]
struct Seating {
  seats: Vec<Vec<Seat>>,
  width: usize,
  height: usize,
}

impl fmt::Display for Seating {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for row in self.seats.iter() {
      for seat in row.iter() {
        write!(f, "{}", seat)?
      }
      write!(f, "\n")?
    }
    Ok(())
  }
}

impl Seating {
  fn new(lines: &mut dyn Iterator<Item = String>) -> Seating {
    let mut width: usize = 0;
    let seats: Vec<Vec<Seat>> = lines.map(|line| {
      let row: Vec<Seat> = line.chars().map(|x| match x {
        '.' => Seat::Floor,
        'L' => Seat::Unoccupied,
        '#' => Seat::Occupied,
        _ => panic!("Invalid seating character")
      }).collect();
      if width == 0 {
        width = row.len();
      } else if width != row.len() {
        panic!("Inconsistent row width");
      }
      row
    }).collect();
    let height = seats.len();
    Seating{
      seats,
      width,
      height,
    }
  }

  fn nearest_seat(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<Seat> {
    let mut nx = x as isize;
    let mut ny = y as isize;
    let w = self.width as isize;
    let h = self.height as isize;
    loop {
      nx += dx;
      ny += dy;
      if nx < 0 || nx >= w || ny < 0 || ny >= h {
        return None;
      }
      let seat = self.seats[ny as usize][nx as usize];
      if seat != Seat::Floor {
        return Some(seat);
      }
    }
  }

  fn seated_neighbours(&self, x: usize, y: usize) -> usize {
    let dirs = vec![
      (-1,-1), ( 0, -1), ( 1, -1),
      (-1, 0),           ( 1,  0),
      (-1, 1), ( 0,  1), ( 1,  1),
    ];
    dirs.iter()
      .map(|(dx, dy)| self.nearest_seat(x, y, *dx, *dy))
      .filter(|opt| if let Some(seat) = opt { *seat == Seat::Occupied } else { false })
      .count()
  }

  fn next(&self) -> Seating {
    let rows = self.seats.iter().enumerate().map(|(y, row)| {
      row.iter().enumerate().map(|(x, seat)| {
        match *seat {
          Seat::Floor => Seat::Floor,
          Seat::Unoccupied => if self.seated_neighbours(x, y) == 0 {
            Seat::Occupied
          } else {
            Seat::Unoccupied
          },
          Seat::Occupied => if self.seated_neighbours(x, y) >= 5 {
            Seat::Unoccupied
          } else {
            Seat::Occupied
          }
        }
      }).collect()
    });
    Seating{
      seats: rows.collect(),
      width: self.width,
      height: self.height,
    }
  }

  fn num_occupied(&self) -> usize {
    self.seats.iter().flatten().filter(|x| **x == Seat::Occupied).count()
  }
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let mut lines = reader.lines().into_iter().map(|x| x.unwrap());
  let verbose = env::args().nth(1).filter(|arg| arg == "-v").is_some();

  let mut prev = Seating::new(&mut lines);
  if verbose {
    println!("{}", &prev);
  }
  loop {
    let next = prev.next();
    if verbose {
      println!("{}", &next);
    }
    if next == prev {
      println!("{} seats are occupied after stabilizing", next.num_occupied());
      break;
    }
    prev = next;
  }
}
