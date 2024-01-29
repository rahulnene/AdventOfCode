use md5::Digest;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::time::{Duration, Instant};

const SEED: &str = "ahsbgdzn";

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve(single_hash), solve(stretched_hash))
}

fn solve(hash_strat: impl Fn(&[u8]) -> Digest + Sync) -> (usize, Duration) {
    let now = Instant::now();

    let (mut index_to_successive_triples, mut index_to_successive_quintuples): (Vec<_>, Vec<_>) =
        (0..30_000)
            .par_bridge()
            .map(|index| {
                let seed_bytes = SEED.as_bytes();
                let mut input = Vec::with_capacity(seed_bytes.len() + 5);
                input.extend_from_slice(seed_bytes);
                input.extend_from_slice(index.to_string().as_bytes());
                let digest = hash_strat(&input);
                let triple = has_triple(&digest);
                let quintuple = has_quintuple(&digest);
                ((index, triple), (index, quintuple))
            })
            .unzip();
    index_to_successive_quintuples.sort_unstable_by_key(|(i, _)| *i);
    index_to_successive_triples.sort_unstable_by_key(|(i, _)| *i);
    let ans = find_keys(
        &index_to_successive_triples,
        &index_to_successive_quintuples,
    );
    (ans, now.elapsed())
}

fn single_hash(s: &[u8]) -> Digest {
    md5::compute(s)
}

fn stretched_hash(s: &[u8]) -> Digest {
    let mut digest = single_hash(s);
    for _ in 0..2016 {
        digest = single_hash(format!("{:x}", digest).as_bytes());
    }
    digest
}

fn has_triple(digest: &Digest) -> Option<u8> {
    let digest = format!("{:x}", digest);
    let triple_window = digest.as_bytes().windows(3);
    // dbg!(&triple_window);
    for window in triple_window {
        if window[0] == window[1] && window[0] == window[2] {
            return Some(window[0]);
        }
    }
    None
}

fn has_quintuple(digest: &Digest) -> Option<u8> {
    let digest = format!("{:x}", digest);
    let quintuple_window = digest.as_bytes().windows(5);
    // dbg!(&quintuple_window);
    for window in quintuple_window {
        if window[0] == window[1]
            && window[0] == window[2]
            && window[0] == window[3]
            && window[0] == window[4]
        {
            return Some(window[0]);
        }
    }
    None
}

fn find_keys(triples: &Vec<(u32, Option<u8>)>, quintuples: &Vec<(u32, Option<u8>)>) -> usize {
    let mut keys = 0;
    let mut counter = 0;
    for (index, triple) in triples {
        if let Some(triple) = triple {
            let quintuple = quintuples
                .iter()
                .filter(|(i, q)| *i > *index && *i <= *index + 1000 && *q == Some(*triple))
                .next();
            if quintuple.is_some() {
                keys = *index;
                counter += 1;
                // dbg!(index, i);
            }
        }
        if counter == 64 {
            break;
        }
    }
    keys as usize
}
