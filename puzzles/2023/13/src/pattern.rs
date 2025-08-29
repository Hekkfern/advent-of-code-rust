pub struct Pattern {
    /// Stores the information of the rows of this pattern in binary format, where '1' means '#'
    /// and '0' means '.'.
    rows_data: Vec<u64>,
    /// Stores the information of the columns of this pattern in binary format, where '1' means '#'
    /// and '0' means '.'.
    columns_data: Vec<u64>,
}

fn mirror(data: &[u64]) -> Option<(usize, usize)> {
    let n = data.len();
    for split in 1..n {
        let prefix = &data[..split];
        let suffix = &data[split..];
        let len = std::cmp::min(prefix.len(), suffix.len());
        let prefix_rev = prefix.iter().rev().take(len);
        let suffix = suffix.iter().take(len);
        if prefix_rev.clone().eq(suffix.clone()) {
            return Some((split - 1, split));
        }
    }
    None
}

fn mirror_with_single_fix(rng: &[u64]) -> Option<(usize, usize)> {
    let n = rng.len();
    for split in 1..n {
        let prefix = &rng[..split];
        let suffix = &rng[split..];
        let len = std::cmp::min(prefix.len(), suffix.len());
        let prefix_rev = prefix.iter().rev().take(len);
        let suffix = suffix.iter().take(len);
        let bitcount: u32 = prefix_rev
            .clone()
            .zip(suffix.clone())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum();
        if bitcount == 1 {
            return Some((split - 1, split));
        }
    }
    None
}

impl Pattern {
    pub fn new(rows_data: Vec<u64>, columns_data: Vec<u64>) -> Self {
        Self {
            rows_data,
            columns_data,
        }
    }

    /// Looks for a mirror line in the rows.
    ///
    /// # Return
    ///
    /// If a mirror line is found, it returns a pair with the indexes of the rows before and after
    /// the mirror lane. Otherwise, `None`.
    pub fn search_horizontal_reflection_line(&self) -> Option<(usize, usize)> {
        mirror(&self.rows_data)
    }

    /// Looks for a mirror line in the columns.
    ///
    /// # Return
    ///
    /// If a mirror line is found, it returns a pair with the indexes of the columns before and after
    /// the mirror lane. Otherwise, `None`.
    pub fn search_vertical_reflection_line(&self) -> Option<(usize, usize)> {
        mirror(&self.columns_data)
    }

    /// Looks for a mirror line in the rows which it is discarded because it has a single error.
    ///
    /// # Return
    ///
    /// If a mirror line is found, it returns a pair with the indexes of the rows before and after
    /// the mirror lane. Otherwise, `None`.
    pub fn search_horizontal_reflection_line_with_single_fix(&self) -> Option<(usize, usize)> {
        mirror_with_single_fix(&self.rows_data)
    }

    /// Looks for a mirror line in the columns which it is discarded because it has a single error.
    ///
    /// # Return
    ///
    /// If a mirror line is found, it returns a pair with the indexes of the columns before and after
    /// the mirror lane. Otherwise, `None`.
    pub fn search_vertical_reflection_line_with_single_fix(&self) -> Option<(usize, usize)> {
        mirror_with_single_fix(&self.columns_data)
    }
}
