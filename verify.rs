use std::env;

const SMALL_FIRST: usize = 6;
const SMALL_LAST: usize = 10_000_000;
const SMALL_MAX_B: usize = 1000;

const FIRST_M: usize = 10_000;
const FIRST_X: usize = 99_000_000;
const FIRST_SEED_COUNT: usize = 94;
const FIRST_SEED_C: usize = 469_615;
const FIRST_SEED_U: usize = 480_503;

const LARGE_M: usize = 20_000_000;
const LARGE_MARGIN: usize = 2_000_000;
const LARGE_FIRST: usize = 105_000_000;
const LARGE_LAST: usize = 156_000_000;

const BRIDGE_X: u64 = 399_000_000_000_000;
const BRIDGE_NEXT_PRIME_AFTER_2M: usize = 40_000_003;
const BRIDGE_BLOCK_RATIO: u64 = 3;
const HELFGOTT_START: u128 = 1_000_000_000_000_000_000_100_000_000;

fn check(condition: bool, message: impl Into<String>) {
    if !condition {
        panic!("{}", message.into());
    }
}

fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|arg| arg == name)
}

fn sieve(limit: usize) -> Vec<u8> {
    check(limit >= 2, format!("bad sieve limit {limit}"));
    let mut is_prime = vec![1_u8; limit + 1];
    is_prime[0] = 0;
    is_prime[1] = 0;
    let root = (limit as f64).sqrt() as usize;
    for p in 2..=root {
        if is_prime[p] == 0 {
            continue;
        }
        let mut k = p * p;
        while k <= limit {
            is_prime[k] = 0;
            k += p;
        }
    }
    is_prime
}

fn primes_from_sieve(is_prime: &[u8]) -> Vec<usize> {
    is_prime
        .iter()
        .enumerate()
        .filter_map(|(n, &flag)| (flag != 0).then_some(n))
        .collect()
}

fn first_prime_greater_than(primes: &[usize], n: usize) -> usize {
    primes.partition_point(|&p| p <= n)
}

fn divisors_of(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    let root = (n as f64).sqrt() as usize;
    for d in 1..=root {
        if n % d != 0 {
            continue;
        }
        divisors.push(d);
        let other = n / d;
        if other != d {
            divisors.push(other);
        }
    }
    divisors.sort_unstable();
    divisors
}

fn prefix_sums(values: &[usize]) -> Vec<usize> {
    let mut out = Vec::with_capacity(values.len());
    let mut running = 0;
    for &value in values {
        running += value;
        out.push(running);
    }
    out
}

fn json_usize_array(values: &[usize]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn verify_small_bq(progress: bool) -> String {
    let is_prime = sieve(SMALL_LAST);
    let primes = primes_from_sieve(&is_prime);
    let mut covered = vec![0_u8; SMALL_LAST + 1];
    let mut examples: Vec<(usize, usize, usize, usize, usize)> = Vec::new();
    let mut max_seen_b = 0;
    let mut marked = 0;

    for b in 1..=SMALL_MAX_B {
        if progress && b % 100 == 0 {
            eprintln!("small-bq: B={b}/{SMALL_MAX_B}");
        }
        let divisors = divisors_of(b);
        let sigma: usize = divisors.iter().sum();
        let slopes = prefix_sums(&divisors);
        let first_prime_index = first_prime_greater_than(&primes, b);

        for slope in slopes {
            let max_q = (SMALL_LAST - sigma) / slope;
            for &q in &primes[first_prime_index..] {
                if q > max_q {
                    break;
                }
                let n = sigma + q * slope;
                if n >= SMALL_FIRST && covered[n] == 0 {
                    covered[n] = 1;
                    marked += 1;
                    if examples.len() < 5 {
                        examples.push((n, b, q, slope, sigma));
                    }
                    max_seen_b = max_seen_b.max(b);
                }
            }
        }
    }

    let mut misses = Vec::new();
    for (n, &flag) in covered
        .iter()
        .enumerate()
        .take(SMALL_LAST + 1)
        .skip(SMALL_FIRST)
    {
        if n == 7 {
            continue;
        }
        if flag == 0 {
            misses.push(n);
            if misses.len() >= 20 {
                break;
            }
        }
    }
    check(
        misses.is_empty(),
        format!("small Bq misses: {}", json_usize_array(&misses)),
    );

    let examples_json = examples
        .iter()
        .map(|&(n, b, q, slope, sigma)| {
            format!(
                "{{\n        \"n\": {n},\n        \"B\": {b},\n        \"q\": {q},\n        \"slope\": {slope},\n        \"sigma\": {sigma}\n      }}"
            )
        })
        .collect::<Vec<_>>()
        .join(",\n      ");

    format!(
        concat!(
            "{{\n",
            "  \"check\": \"small-bq\",\n",
            "  \"range\": [{SMALL_FIRST}, {SMALL_LAST}],\n",
            "  \"maxB\": {SMALL_MAX_B},\n",
            "  \"exceptionHandledSeparately\": {{\n",
            "    \"n\": 7,\n",
            "    \"m\": 4,\n",
            "    \"prefix\": [1, 2, 4]\n",
            "  }},\n",
            "  \"markedCount\": {marked},\n",
            "  \"maxSeenB\": {max_seen_b},\n",
            "  \"examples\": [\n",
            "      {examples_json}\n",
            "  ],\n",
            "  \"ok\": true\n",
            "}}"
        ),
        SMALL_FIRST = SMALL_FIRST,
        SMALL_LAST = SMALL_LAST,
        SMALL_MAX_B = SMALL_MAX_B,
        marked = marked,
        max_seen_b = max_seen_b,
        examples_json = examples_json,
    )
}

fn reachable_subset_sums(values: &[usize]) -> Vec<u8> {
    let mut reachable = vec![1_u8];
    let mut max_sum = 0;
    for &value in values {
        let mut next = vec![0_u8; max_sum + value + 1];
        next[..reachable.len()].copy_from_slice(&reachable);
        for s in 0..=max_sum {
            if reachable[s] != 0 {
                next[s + value] = 1;
            }
        }
        reachable = next;
        max_sum += value;
    }
    reachable
}

fn longest_reachable_interval(reachable: &[u8]) -> (usize, usize) {
    let mut best_start = 0;
    let mut best_end = 0;
    let mut best_len = 0;
    let mut start: Option<usize> = None;
    for n in 0..=reachable.len() {
        let on = n < reachable.len() && reachable[n] != 0;
        if on && start.is_none() {
            start = Some(n);
        }
        if !on {
            if let Some(s) = start {
                let end = n - 1;
                let len = end - s + 1;
                if len > best_len {
                    best_start = s;
                    best_end = end;
                    best_len = len;
                }
                start = None;
            }
        }
    }
    (best_start, best_end)
}

fn verify_first_window() -> String {
    check(
        FIRST_X < FIRST_M * FIRST_M,
        format!("need X < M^2, got {FIRST_X} >= {}", FIRST_M * FIRST_M),
    );

    let primes = primes_from_sieve(&sieve(FIRST_X))
        .into_iter()
        .filter(|&p| p > FIRST_M)
        .collect::<Vec<_>>();
    let seed_primes = &primes[..FIRST_SEED_COUNT];
    let reachable = reachable_subset_sums(seed_primes);
    for n in FIRST_SEED_C..=FIRST_SEED_U {
        check(reachable[n] != 0, format!("seed sum {n} is not reachable"));
    }

    let c = FIRST_SEED_C;
    let mut u: u128 = FIRST_SEED_U as u128;
    for &p in &primes[FIRST_SEED_COUNT..] {
        check(
            (p as u128) <= u - c as u128 + 1,
            format!("extension gap at p={p}, interval=[{c},{u}]"),
        );
        u += p as u128;
    }
    let (longest_start, longest_end) = longest_reachable_interval(&reachable);

    format!(
        concat!(
            "{{\n",
            "  \"check\": \"first-window\",\n",
            "  \"M\": {FIRST_M},\n",
            "  \"X\": {FIRST_X},\n",
            "  \"seedCount\": {FIRST_SEED_COUNT},\n",
            "  \"seedPrimeRange\": [{seed_first}, {seed_last}],\n",
            "  \"assertedSeedInterval\": [{FIRST_SEED_C}, {FIRST_SEED_U}],\n",
            "  \"longestSeedInterval\": [{longest_start}, {longest_end}],\n",
            "  \"primeCountInWindow\": {prime_count},\n",
            "  \"largestPrime\": {largest_prime},\n",
            "  \"primeSumInterval\": [\"{c}\", \"{u}\"],\n",
            "  \"targetInterval\": [\"{target_c}\", \"{target_u}\"],\n",
            "  \"ok\": true\n",
            "}}"
        ),
        FIRST_M = FIRST_M,
        FIRST_X = FIRST_X,
        FIRST_SEED_COUNT = FIRST_SEED_COUNT,
        FIRST_SEED_C = FIRST_SEED_C,
        FIRST_SEED_U = FIRST_SEED_U,
        seed_first = seed_primes[0],
        seed_last = seed_primes[FIRST_SEED_COUNT - 1],
        longest_start = longest_start,
        longest_end = longest_end,
        prime_count = primes.len(),
        largest_prime = primes[primes.len() - 1],
        c = c,
        u = u,
        target_c = c + 1,
        target_u = u + 1,
    )
}

fn next_prime_above(is_prime: &[u8], n: usize) -> usize {
    for (p, &flag) in is_prime.iter().enumerate().skip(n + 1) {
        if flag != 0 {
            return p;
        }
    }
    panic!("sieve too small to find prime above {}", n);
}

fn forbidden(p: usize, f0: usize, f1: usize, f2: usize) -> bool {
    p == f0 || p == f1 || p == f2
}

fn central_pair(
    is_prime: &[u8],
    even: usize,
    lo: usize,
    hi: usize,
    f0: usize,
    f1: usize,
    f2: usize,
) -> Option<(usize, usize)> {
    let start = (lo + 1).max(even.saturating_sub(hi).saturating_add(1));
    let stop = (hi - 1).min(even / 2);
    let first = if start.is_multiple_of(2) {
        start + 1
    } else {
        start
    };
    let mut p = first;
    while p <= stop {
        let q = even - p;
        if p != q
            && is_prime[p] != 0
            && is_prime[q] != 0
            && !forbidden(p, f0, f1, f2)
            && !forbidden(q, f0, f1, f2)
        {
            return Some((p, q));
        }
        p += 2;
    }
    None
}

fn four_prime_certificate(
    is_prime: &[u8],
    even: usize,
    m: usize,
    margin: usize,
    f0: usize,
) -> Option<[usize; 4]> {
    let lo = m;
    let hi = 2 * m;
    let min_even_chunk = 2 * m + margin;
    let max_even_chunk = 4 * m - margin;
    let mid = even / 2;
    let mut e1 = if mid.is_multiple_of(2) { mid } else { mid - 1 };

    while e1 >= min_even_chunk && even - e1 <= max_even_chunk {
        let e2 = even - e1;
        if e2 >= min_even_chunk && e2 <= max_even_chunk {
            if let Some((p1, q1)) = central_pair(is_prime, e1, lo, hi, f0, 0, 0) {
                if let Some((p2, q2)) = central_pair(is_prime, e2, lo, hi, f0, p1, q1) {
                    return Some([p1, q1, p2, q2]);
                }
            }
        }
        if e1 < 2 {
            break;
        }
        e1 -= 2;
    }
    None
}

enum Certificate {
    Four([usize; 4]),
    Five([usize; 5]),
}

impl Certificate {
    fn values(&self) -> &[usize] {
        match self {
            Certificate::Four(values) => values,
            Certificate::Five(values) => values,
        }
    }

    fn to_json_array(&self) -> String {
        json_usize_array(self.values())
    }
}

fn seed_certificate(
    is_prime: &[u8],
    n: usize,
    m: usize,
    margin: usize,
    p0: usize,
) -> Option<Certificate> {
    if n.is_multiple_of(2) {
        return four_prime_certificate(is_prime, n, m, margin, 0).map(Certificate::Four);
    }
    four_prime_certificate(is_prime, n - p0, m, margin, p0)
        .map(|tail| Certificate::Five([p0, tail[0], tail[1], tail[2], tail[3]]))
}

fn assert_prime_certificate(cert: &Certificate, is_prime: &[u8], n: usize, m: usize) {
    let values = cert.values();
    let mut total = 0;
    for (i, &p) in values.iter().enumerate() {
        total += p;
        for &previous in &values[..i] {
            check(p != previous, format!("certificate for {n} repeats prime {p}"));
        }
        check(is_prime[p] != 0, format!("certificate for {n} uses non-prime {p}"));
        check(
            m < p && p < 2 * m,
            format!("certificate for {n} uses {p} outside ({m}, {})", 2 * m),
        );
    }
    check(total == n, format!("certificate for {n} sums to {total}"));
}

fn verify_large_seed(progress: bool) -> String {
    let is_prime = sieve(2 * LARGE_M + 1_000_000);
    let p0 = next_prime_above(&is_prime, LARGE_M);
    let next_after_window = next_prime_above(&is_prime, 2 * LARGE_M);
    let mut misses = Vec::new();
    let mut example: Option<(usize, String)> = None;

    for n in LARGE_FIRST..=LARGE_LAST {
        if progress && (n - LARGE_FIRST) % 1_000_000 == 0 {
            eprintln!("large-seed: n={n}/{LARGE_LAST}");
        }
        let Some(cert) = seed_certificate(&is_prime, n, LARGE_M, LARGE_MARGIN, p0) else {
            misses.push(n);
            if misses.len() >= 20 {
                break;
            }
            continue;
        };
        assert_prime_certificate(&cert, &is_prime, n, LARGE_M);
        if example.is_none() {
            example = Some((n, cert.to_json_array()));
        }
    }

    check(
        misses.is_empty(),
        format!("large seed misses: {}", json_usize_array(&misses)),
    );
    check(
        LARGE_LAST - LARGE_FIRST + 1 >= next_after_window,
        "large seed too short to extend",
    );
    let (example_n, example_primes) = example.expect("large seed should have an example");

    format!(
        concat!(
            "{{\n",
            "  \"check\": \"large-seed\",\n",
            "  \"M\": {LARGE_M},\n",
            "  \"margin\": {LARGE_MARGIN},\n",
            "  \"primeWindow\": [{prime_window_first}, {prime_window_last}],\n",
            "  \"interval\": [{LARGE_FIRST}, {LARGE_LAST}],\n",
            "  \"inclusiveWidth\": {inclusive_width},\n",
            "  \"firstPrimeAboveM\": {p0},\n",
            "  \"nextPrimeAfter2M\": {next_after_window},\n",
            "  \"startsOverlapInduction\": true,\n",
            "  \"example\": {{\n",
            "    \"n\": {example_n},\n",
            "    \"primes\": {example_primes}\n",
            "  }},\n",
            "  \"ok\": true\n",
            "}}"
        ),
        LARGE_M = LARGE_M,
        LARGE_MARGIN = LARGE_MARGIN,
        LARGE_FIRST = LARGE_FIRST,
        LARGE_LAST = LARGE_LAST,
        prime_window_first = LARGE_M + 1,
        prime_window_last = 2 * LARGE_M - 1,
        inclusive_width = LARGE_LAST - LARGE_FIRST + 1,
        p0 = p0,
        next_after_window = next_after_window,
        example_n = example_n,
        example_primes = example_primes,
    )
}

fn pi_lower_floor(x: u64) -> i128 {
    ((x as f64) / (x as f64).ln() - 1000.0).floor() as i128
}

fn pi_upper_ceil(x: u64) -> i128 {
    ((1.25506 * x as f64) / (x as f64).ln() + 1000.0).ceil() as i128
}

#[derive(Clone)]
struct Block {
    a: u64,
    b: u64,
    count_lower: i128,
    contribution: u128,
}

fn lower_prime_sum_by_blocks(first_a: u64, last_b: u64, ratio: u64) -> (u128, Vec<Block>) {
    let mut a = first_a;
    let mut lower_sum = 0_u128;
    let mut blocks = Vec::new();
    while a < last_b {
        let b = last_b.min(a * ratio);
        let count_lower = 0_i128.max(pi_lower_floor(b) - pi_upper_ceil(a));
        let contribution = a as u128 * count_lower as u128;
        lower_sum += contribution;
        blocks.push(Block {
            a,
            b,
            count_lower,
            contribution,
        });
        a = b;
    }
    (lower_sum, blocks)
}

fn block_json(block: &Block) -> String {
    format!(
        concat!(
            "{{\n",
            "      \"a\": {a},\n",
            "      \"b\": {b},\n",
            "      \"countLower\": {count_lower},\n",
            "      \"contribution\": \"{contribution}\"\n",
            "    }}"
        ),
        a = block.a,
        b = block.b,
        count_lower = block.count_lower,
        contribution = block.contribution,
    )
}

fn blocks_json(blocks: &[Block]) -> String {
    blocks.iter().map(block_json).collect::<Vec<_>>().join(",\n    ")
}

fn verify_large_bridge() -> String {
    let seed_width = LARGE_LAST - LARGE_FIRST + 1;
    check(
        BRIDGE_X < (LARGE_M as u64) * (LARGE_M as u64),
        format!(
            "need X < M^2; got {BRIDGE_X} >= {}",
            (LARGE_M as u64) * (LARGE_M as u64)
        ),
    );
    check(
        seed_width >= BRIDGE_NEXT_PRIME_AFTER_2M,
        "large seed interval is too short for the next prime",
    );

    let (lower_sum, blocks) =
        lower_prime_sum_by_blocks(2 * LARGE_M as u64, BRIDGE_X, BRIDGE_BLOCK_RATIO);
    let target_u = LARGE_LAST as u128 + lower_sum + 1;
    check(
        target_u >= HELFGOTT_START,
        format!("large bridge stops too early: {target_u}"),
    );
    let first_blocks = blocks_json(&blocks[..3]);
    let last_blocks = blocks_json(&blocks[blocks.len() - 3..]);

    format!(
        concat!(
            "{{\n",
            "  \"check\": \"large-bridge\",\n",
            "  \"M\": {LARGE_M},\n",
            "  \"X\": {BRIDGE_X},\n",
            "  \"protectedWindow\": true,\n",
            "  \"seedPrimeSumInterval\": [{seed_c}, {seed_u}],\n",
            "  \"seedWidth\": {seed_width},\n",
            "  \"firstPrimeAfter2M\": {BRIDGE_NEXT_PRIME_AFTER_2M},\n",
            "  \"firstOverlapHolds\": true,\n",
            "  \"overlapInvariant\": \"first overlap checked here; subsequent overlaps follow from Bertrand's postulate\",\n",
            "  \"lowerBoundForSumPrimesIn2MToX\": \"{lower_sum}\",\n",
            "  \"targetInterval\": [\"{target_c}\", \"{target_u}\"],\n",
            "  \"helfgottStart\": \"{HELFGOTT_START}\",\n",
            "  \"overlapsHelfgottRange\": true,\n",
            "  \"blockRatio\": {BRIDGE_BLOCK_RATIO},\n",
            "  \"blockCount\": {block_count},\n",
            "  \"firstBlocks\": [\n",
            "    {first_blocks}\n",
            "  ],\n",
            "  \"lastBlocks\": [\n",
            "    {last_blocks}\n",
            "  ],\n",
            "  \"ok\": true\n",
            "}}"
        ),
        LARGE_M = LARGE_M,
        BRIDGE_X = BRIDGE_X,
        seed_c = LARGE_FIRST,
        seed_u = LARGE_LAST,
        seed_width = seed_width,
        BRIDGE_NEXT_PRIME_AFTER_2M = BRIDGE_NEXT_PRIME_AFTER_2M,
        lower_sum = lower_sum,
        target_c = LARGE_FIRST as u128 + 1,
        target_u = target_u,
        HELFGOTT_START = HELFGOTT_START,
        BRIDGE_BLOCK_RATIO = BRIDGE_BLOCK_RATIO,
        block_count = blocks.len(),
        first_blocks = first_blocks,
        last_blocks = last_blocks,
    )
}

fn verify_tail_constants() -> String {
    let weight = 1.079955_f64.powi(2) * 1.414;
    let theta_coeff = (3.0 * weight * 1.03883_f64.powi(2)) / 30_000.0;
    let n0 = 1e27_f64;
    let repeated_ratio = (3.0 * weight * n0.ln().powi(3)) / n0;
    let lower_bound = 0.0002443;
    let bad_bound = theta_coeff + repeated_ratio;
    let even_tail_product_lower_ratio = n0 / (30_000.0 * n0.ln()).powi(2);

    check(
        theta_coeff < 0.000178,
        format!("small-coordinate bound too large: {theta_coeff}"),
    );
    check(
        repeated_ratio < 1e-20,
        format!("repeated-coordinate bound too large: {repeated_ratio}"),
    );
    check(
        bad_bound < lower_bound,
        format!("tail bad mass {bad_bound} exceeds {lower_bound}"),
    );
    check(
        even_tail_product_lower_ratio > 1.0,
        format!("even-tail pq>N margin failed: {even_tail_product_lower_ratio}"),
    );

    let ell_upper = 120_000.0 * (2e27_f64).ln();
    let slack = 100_000_000.0 - 1.0 - ell_upper;
    check(slack > 0.0, format!("odd-tail base prime slack failed: {slack}"));
    let ell_less_than_p_left_margin =
        n0 / (60_000.0 * n0.ln()) - 120_000.0 * (2e27_f64).ln();
    check(
        ell_less_than_p_left_margin > 0.0,
        format!("odd-tail ell < p margin failed: {ell_less_than_p_left_margin}"),
    );

    format!(
        concat!(
            "{{\n",
            "  \"check\": \"tail\",\n",
            "  \"helfgottWeightedLowerBound\": {lower_bound},\n",
            "  \"smallCoordinateBoundWithC30000\": {theta_coeff},\n",
            "  \"repeatedCoordinateRatioAt1e27\": {repeated_ratio},\n",
            "  \"totalBadUpperBoundAt1e27\": {bad_bound},\n",
            "  \"evenTailProductLowerRatioAt1e27\": {even_tail_product_lower_ratio},\n",
            "  \"oddTailSlackAtStart\": {slack},\n",
            "  \"ellLessThanPLeftMargin\": {ell_less_than_p_left_margin},\n",
            "  \"ok\": true\n",
            "}}"
        ),
        lower_bound = lower_bound,
        theta_coeff = theta_coeff,
        repeated_ratio = repeated_ratio,
        bad_bound = bad_bound,
        even_tail_product_lower_ratio = even_tail_product_lower_ratio,
        slack = slack,
        ell_less_than_p_left_margin = ell_less_than_p_left_margin,
    )
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let command = args.get(1).map(String::as_str).unwrap_or("all");
    let progress = has_flag(&args, "--progress");

    let output = match command {
        "small-bq" => verify_small_bq(progress),
        "first-window" => verify_first_window(),
        "large-seed" => verify_large_seed(progress),
        "large-bridge" => verify_large_bridge(),
        "tail" => verify_tail_constants(),
        "all" => {
            let results = [
                verify_small_bq(progress),
                verify_first_window(),
                verify_large_seed(progress),
                verify_large_bridge(),
                verify_tail_constants(),
            ];
            format!(
                concat!(
                    "{{\n",
                    "  \"check\": \"all\",\n",
                    "  \"results\": [\n",
                    "    {results}\n",
                    "  ],\n",
                    "  \"ok\": true\n",
                    "}}"
                ),
                results = results.join(",\n    ")
            )
        }
        _ => panic!(
            "unknown command {}; use all, small-bq, first-window, large-seed, large-bridge, or tail",
            command
        ),
    };
    println!("{output}");
}
