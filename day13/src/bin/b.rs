use std::fs::File;
use std::io::{BufRead,BufReader};
use num_integer::gcd;

fn make_coprime(xs: &mut Vec<(isize, isize)>) {
  let l = xs.len();
  for i in 0..l-1 {
    let a = xs[i];
    for j in i+1..l {
      let b = xs[j];
      let n = gcd(a.1, b.1);
      if n > 1 {
        // Strictly speaking, we only need to reduce one of them to make them
        // coprime.
        if n < a.1 {
          xs[i] = (a.0, a.1 / n);
        }
        if n < b.1 {
          xs[j] = (b.0, b.1 / n);
        }
      }
    }
  }
}

// Convert each (x, y) meaning "t + x = ny" into (r, y) meaning "t = r (mod y)"
fn get_remainders(xs: &mut Vec<(isize, isize)>) {
  for x in xs.iter_mut() {
    let mut r = -x.0;
    while r < 0 {
      r += x.1;
    }
    *x = (r, x.1);
  }
}

struct Routing {
  routes: Vec<Option<isize>>,
}

impl Routing {
  fn fixed(&self) -> Vec<(isize, isize)> {
    self.routes.iter().enumerate().flat_map(|(idx, id)| {
      match id {
        Some(id) => Some((idx as isize, *id)),
        None => None,
      }
    }).collect()
  }

  fn find_sequence(&self, sort: bool) -> isize {
    /*
    We are solving for the smallest t such that (ignoring unfixed constraints):
    t     = k0 * b0
    t + 1 = k1 * b1
    t + 2 = k2 * b2
    ...
    t + n = kn * bn

    In other words:
    t = 0 (mod b0)
    t = -1 (mod b1) = (b1 - 1) (mod b1)
    t = -2 (mod b2) = (b2 - 2) (mod b2)
    ...
    t = -n (mod bn) = (bn - n) (mod bn)

    This is an application of the Chinese remainder theorem, using a sieve
    approach. The theorem requires that all elements be pairwise coprime for it
    to work, so a first pass is done to eliminate common factors. It then
    proceeds like so:
    
    1. Find t1 = -1 (mod b1) in {b0, 2*b0, 3*b0, ...}
    2. Find t2 = -2 (mod b2) in {t1, t1 + b0*b1, t1 + 2*b0*b1, ...}
    3. Find t3 = -3 (mod b3) in {t2, t2 + b0*b1*b2, t2 + 2*b0*b1*b2, ...}
    etc.

    The final tn will be the solution. If any of the original values were not
    coprime, the solution must be repeated a final time against the original
    values.
    */
    let mut values = self.fixed();
    make_coprime(&mut values);
    get_remainders(&mut values);
    if sort {
      // Hopefully this will speed things up...
      values.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    }
    let mut iter = values.iter();
    let first = iter.next().expect("Expected a non-empty list");
    let mut t = first.0;
    let mut f = first.1;
    for x in iter {
      while t % x.1 != x.0 {
        t += f;
      }
      f *= x.1;
    }
    t
  }
}

fn main() {
  let f = File::open("input.txt").expect("Unable to open input file");
  let reader = BufReader::new(f);
  let mut lines = reader.lines().into_iter().map(|x| x.unwrap());

  lines.next().expect("Missing earliest departure line");
  let routes = lines.next().expect("Missing routes line").split(',').map(
    |x| match x {
      "x" => None,
      _ => Some(isize::from_str_radix(&x, 10).expect("Invalid bus id"))
    }
  ).collect();

  let routing = Routing{routes};
  let t = routing.find_sequence(false);
  println!("The first sequence begins at {}", t);
}
