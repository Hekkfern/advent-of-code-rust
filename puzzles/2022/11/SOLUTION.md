# Solution explanation for AoC 2022 Day 11

For Part 2, the key is to compute the product of all monkeys' mods and use that modulus on all item values after every
computation.
We use the mathematical property `(a mod kn) mod n ≡ a mod n` which is true for any integer `k`.
In our case, `k` is the product of all the divisors, so we are just storing `a mod kn` instead of `a`, which is
sufficient to determine the divisibility by each monkey's divisor.
