
![doc](https://docs.rs/dorset/badge.svg)
# Table of Contents

1.  [`dorset`, a rust implementation of `Stan Math` library.](#org8042ff3)
    1.  [Example](#org1523006)
2.  [Lisense](#orgcf15d93)


<a id="org8042ff3"></a>

# `dorset`, a rust implementation of `Stan Math` library.

The project is not intended to be an exact port of `Math`,
but as a proof of concept to take advantage of some
Rust features in automatic differentiation,
hence meant to be experimental.


<a id="org1523006"></a>

## Example

Below is how to do the problem example in section 2.1 of
[the Stan Math paper](https://arxiv.org/abs/1509.07164), using `dorset`.

    
    #[macro_use]
    extern crate dorset;
    use dorset::*;
    
    fn main() {
        let y: Real = 1.3;
        let s = cstack!();
        let (mu, sigma) = (var!(s, 0.5), var!(s, 1.2));
        let mut lp = var!(s);
        lp = &lp - 0.5 * (2.0 * PI).ln();
        lp = &lp - ln(&sigma);
        lp = &lp - 0.5 * pow((y - &mu) / &sigma, 2.0);
        lp.grad();
        println!("f(mu, sigma) = {0:.6}", lp.val()); // f(mu, sigma) = -1.323482
        println!("d.f/d.mu = {0:.6}", mu.adj());     // d.f/d.mu = 0.555556
        println!("d.f/d.sigma = {0:.6}", sigma.adj()); // d.f/d.sigma = -0.462963
    }


<a id="orgcf15d93"></a>

# Lisense

BSD-3-Clause

