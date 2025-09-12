# Solution explanation for AoC 2021 Day 3

## Part 2

We sort the input codes. Then I keep track of the boundaries for both oxygen and CO2, and do a binary search to find which section (1s or 0s) for a particular bit is larger, changing the corresponding upper or lower bound.

