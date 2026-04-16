# Erdos Problem 1054 verifier

This repository supports an answer to [Erdos Problem 1054](https://www.erdosproblems.com/1054) on the Erdos Problems website.

I think the remaining well-definedness issue can be closed. The verifier is the single Rust file [`verify.rs`](verify.rs). A sample full run is committed as [`output.json`](output.json).

```sh
rustc -O verify.rs -o /tmp/erdos_1054_verify
/tmp/erdos_1054_verify all
```

Success means the final JSON object contains `"check": "all"` and `"ok": true`.

Let $d_1(m)<d_2(m)<\cdots$ be the divisors of $m$. Say that $n$ is represented if $n=d_1(m)+\cdots+d_k(m)$ for some $m,k$.

The basic gadget is this. If $M<p_1<\cdots<p_t\le X<M^2$ are primes, then for $m=p_1\cdots p_t$, the divisors begin $1,p_1,\ldots,p_t$, because every composite divisor is $>M^2$. More generally, any subset sum $N=p_{i_1}+\cdots+p_{i_s}$ of primes in $(M,X]$ gives a representation of $N+1$, by taking $m=p_{i_1}\cdots p_{i_s}$.

The other elementary ingredient is the interval-extension trick. If subset sums cover $[C,U]$, and the next prime $p$ satisfies $p\le U-C+1$, then after adding $p$ they cover $[C,U+p]$, since $[C,U]$ overlaps $[C+p,U+p]$.

The verifier checks:

1. A direct $Bq$ divisor-prefix search covers every $6\le n\le10\,000\,000$, except $7$. The remaining value is $7=1+2+4$, represented by $m=4$.

2. A first prime-window certificate with $M=10\,000$ and $X=99\,000\,000<M^2$ gives representations for every $469\,616\le n\le 273\,803\,744\,799\,154$.

3. A larger seed proves every integer in $[105\,000\,000,156\,000\,000]$ is a sum of distinct primes in $(20\,000\,000,40\,000\,000)$. The seed length is $51\,000\,001$, while the next prime after $40\,000\,000$ is $40\,000\,003$, so the interval-extension induction starts. Bertrand's postulate continues it: once a prime $p$ has been adjoined, the interval length is at least $2p$, and the next prime is $<2p$.

Using Rosser-Schoenfeld bounds for $\pi(x)$, the verifier lower-bounds the prime mass up to $X=399\,000\,000\,000\,000<20\,000\,000^2$. This gives representations for every
$$
105\,000\,001\le n\le
1\,111\,351\,202\,532\,220\,892\,436\,000\,001.
$$

4. For the tail, I use an explicit consequence of Helfgott's ternary Goldbach proof: every odd $N\ge 10^{27}$ is a sum of three distinct odd primes, each $>N/(30000\log N)$. The constants come from Helfgott's Section 7; the verifier checks the numerical margins used to discard triples with a small or repeated coordinate.

For even $n$, write $n-1=p+q+r$. Since $pq>n-1>r$, the number $m=pqr$ has initial divisors $1,p,q,r$ and represents $n$.

For odd $n$, choose a prime $60000\log n<\ell<120000\log n$, and write $n-1-\ell=p+q+r$. For $n\ge 10^{27}+10^8$, the verifier checks the slack ensuring $\ell<p$ and $\ell p>r$. So $m=\ell pqr$ has initial divisors $1,\ell,p,q,r$ and represents $n$.

The covered intervals are:

- $[6,10\,000\,000]$,
- $[469\,616,273\,803\,744\,799\,154]$,
- $[105\,000\,001, 1\,111\,351\,202\,532\,220\,892\,436\,000\,001]$,
- $[10^{27}+10^8,\infty)$.

They overlap, so every $n\ge6$ is represented.

Finally, $2$ and $5$ are impossible. The first prefix is $1$, and any longer prefix is at least $1+2=3$, so $2$ is impossible. For $5$, a two-term prefix would have to be $1+4$, but $4$ cannot be the smallest divisor after $1$; with at least three terms the sum is already at least $6$. Also $1,3,4$ are represented by $m=1,2,3$.

Thus the represented positive integers are exactly $\mathbb N\setminus\{2,5\}$.

References:

- Harald Helfgott, [The ternary Goldbach conjecture is true](https://arxiv.org/abs/1312.7748), Section 7.
- J. Barkley Rosser and Lowell Schoenfeld, [Approximate formulas for some functions of prime numbers](https://doi.org/10.1215/ijm/1255631807).
