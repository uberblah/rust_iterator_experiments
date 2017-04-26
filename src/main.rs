#![allow(dead_code)]

const TOP: usize = 220;
const N: usize = 17;

fn main() {
    //solve_problem();
    let primes = iterate_primes(1000000 + N);
    let probs: Vec<(&[usize],usize)> = primes.windows(N).map(
        |seq| (seq, seq.iter().sum())
    ).inspect(|item| {println!("{:?}", item)}).collect();
}

fn iterate_primes(nprimes: usize) -> Vec<usize> {
    (2..).scan(Vec::<usize>::new(), |mut prevs, n| {
        let is_prime = prevs.iter().cloned().take_while(|x| {
                *x < ((n as f64).sqrt() as usize)
            }).all(|x| {
                n % x != 0
            });
        if is_prime {prevs.push(n);}
        Some((n, is_prime))
    }).filter_map(|tuple| {
        let (n, is_prime) = tuple;
        if is_prime {Some(n)} else {None}
    }).take(nprimes).collect()
}

fn solve_problem() {
    // enumerate all primes less than 220, by filtering out non-primes
    let firstprimes: Vec<usize> = (1..TOP).filter(|k| {
        // enumerate pairs (i, j) j <= i < cand where i * j = cand
        (1..TOP).take(k - 1).flat_map(|i| {
            (1..TOP).take(i).filter(move|j| {
                j * i == *k
            })
        }).count() == 0
    }).collect();
    println!("{:?}", firstprimes);
    // take all runs of N items and check if they add to that number
    let sumruns: Vec<[usize;N]> = firstprimes.iter().scan([0;N], |st, item| {
            for i in 0..N-1 {st[i] = st[i+1];}
            st[N-1] = *item;
            Some(*st)
        }).filter(|st| {
            (&st).iter().cloned().sum::<usize>() == TOP
        }).collect();
    println!("{:?}", sumruns);
}
