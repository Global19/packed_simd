//! Scalar fannkuch redux implementation

use std::{cmp, mem, thread};

// FIXME: replace with slice rotate
fn rotate(x: &mut [i32]) {
    let mut prev = x[0];
    for place in x.iter_mut().rev() {
        prev = mem::replace(place, prev)
    }
}

fn next_permutation(perm: &mut [i32], count: &mut [i32]) {
    for i in 1..perm.len() {
        rotate(&mut perm[..i + 1]);
        let count_i = &mut count[i];
        if *count_i >= i as i32 {
            *count_i = 0;
        } else {
            *count_i += 1;
            break;
        }
    }
}

#[derive(Clone, Copy)]
struct P {
    p: [i32; 16],
}

#[derive(Clone, Copy)]
struct Perm {
    cnt: [i32; 16],
    fact: [u32; 16],
    n: u32,
    permcount: u32,
    perm: P,
}

impl Perm {
    fn new(n: u32) -> Self {
        let mut fact = [1; 16];
        for i in 1..n as usize + 1 {
            fact[i] = fact[i - 1] * i as u32;
        }
        Self { cnt: [0; 16], fact, n, permcount: 0, perm: P { p: [0; 16] } }
    }

    fn get(&mut self, mut idx: i32) -> P {
        let mut pp = [0_u8; 16];
        self.permcount = idx as u32;
        for (i, place) in self.perm.p.iter_mut().enumerate() {
            *place = i as i32 + 1;
        }

        for i in (1..self.n as usize).rev() {
            let d = idx / self.fact[i] as i32;
            self.cnt[i] = d;
            idx %= self.fact[i] as i32;
            for (place, val) in
                pp.iter_mut().zip(self.perm.p[..(i + 1)].iter())
            {
                *place = (*val) as u8
            }

            let d = d as usize;
            for j in 0..i + 1 {
                self.perm.p[j] = i32::from(if j + d <= i {
                    pp[j + d]
                } else {
                    pp[j + d - i - 1]
                });
            }
        }

        self.perm
    }

    fn count(&self) -> u32 {
        self.permcount
    }
    fn max(&self) -> u32 {
        self.fact[self.n as usize]
    }

    fn next(&mut self) -> P {
        next_permutation(&mut self.perm.p, &mut self.cnt);
        self.permcount += 1;

        self.perm
    }
}

fn reverse(tperm: &mut [i32], k: usize) {
    tperm[..k].reverse()
}

fn work(mut perm: Perm, n: usize, max: usize) -> (i32, i32) {
    let mut checksum = 0;
    let mut maxflips = 0;

    let mut p = perm.get(n as i32);

    while perm.count() < max as u32 {
        let mut flips = 0;

        while p.p[0] != 1 {
            let k = p.p[0] as usize;
            reverse(&mut p.p, k);
            flips += 1;
        }

        checksum += if perm.count() % 2 == 0 { flips } else { -flips };
        maxflips = cmp::max(maxflips, flips);

        p = perm.next();
    }

    (checksum, maxflips)
}

pub fn fannkuch_redux(n: usize) -> (i32, i32) {
    let perm = Perm::new(n as u32);

    let m = 1;
    let mut futures = vec![];
    let k = perm.max() / m;

    for j in (0..).map(|x| x * k).take_while(|&j| j < k * m) {
        let max = cmp::min(j + k, perm.max());

        futures
            .push(thread::spawn(move || work(perm, j as usize, max as usize)))
    }

    let mut checksum = 0;
    let mut maxflips = 0;
    for fut in futures {
        let (cs, mf) = fut.join().unwrap();
        checksum += cs;
        maxflips = cmp::max(maxflips, mf);
    }
    (checksum, maxflips)
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(fannkuch_redux(7), (228, 16));
}
