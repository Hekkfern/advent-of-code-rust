#[derive(Eq, PartialEq, Clone, Hash, Debug)]
enum ExpandedSpringStatus {
    OneDamaged,
    GroupOfOperational,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Record {
    springs: String,
    contiguous_group_info: Vec<i32>,
}

fn expand_contiguous_group_info(
    group_info: &Vec<i32>,
    repetitions: i32,
) -> Vec<ExpandedSpringStatus> {
    let mut result: Vec<ExpandedSpringStatus> = vec![ExpandedSpringStatus::GroupOfOperational];
    (0..repetitions).for_each(|_| {
        group_info.iter().for_each(|&group_size| {
            (0..group_size).for_each(|_| {
                result.push(ExpandedSpringStatus::OneDamaged);
            });
            result.push(ExpandedSpringStatus::GroupOfOperational);
        });
    });
    result
}

impl Record {
    pub fn new(springs: String, contiguous_group_info: Vec<i32>) -> Self {
        Self {
            springs,
            contiguous_group_info,
        }
    }

    pub fn solve_original(&self) -> u64 {
        // add extra operational springs in both sides
        let extended_springs = format!(".{}.", self.springs).chars().collect::<Vec<_>>();
        // translate groupInfo
        let expanded_group_info = expand_contiguous_group_info(&self.contiguous_group_info, 1);

        // solve
        let n = extended_springs.len();
        let m = expanded_group_info.len();
        let mut dp = vec![vec![0u64; m + 1]; n + 1];
        dp[n][m] = 1;
        for i in (0..n).rev() {
            let start_j = m.checked_sub(n - i).unwrap_or(0);
            for j in (start_j..m).rev() {
                let mut value: u64 = 0;
                if extended_springs[i] != '.'
                    && expanded_group_info[j] == ExpandedSpringStatus::OneDamaged
                {
                    value = dp[i + 1][j + 1];
                } else if extended_springs[i] != '#'
                    && expanded_group_info[j] == ExpandedSpringStatus::GroupOfOperational
                {
                    value = dp[i + 1][j + 1] + dp[i + 1][j];
                }
                dp[i][j] = value;
            }
        }

        dp[0][0]
    }

    pub fn solve_unfolded(&self) -> u64 {
        // create 4 more copies and add extra operational springs in both sides
        let extended_springs = format!(".{0}?{0}?{0}?{0}?{0}.", self.springs)
            .chars()
            .collect::<Vec<_>>();
        // translate groupInfo
        let expanded_group_info = expand_contiguous_group_info(&self.contiguous_group_info, 5);

        // solve
        let n = extended_springs.len();
        let m = expanded_group_info.len();
        let mut dp = vec![vec![0u64; m + 1]; n + 1];
        dp[n][m] = 1;
        for i in (0..n).rev() {
            let start_j = m.checked_sub(n - i).unwrap_or(0);
            for j in (start_j..m).rev() {
                let mut value: u64 = 0;
                if extended_springs[i] != '.'
                    && expanded_group_info[j] == ExpandedSpringStatus::OneDamaged
                {
                    value = dp[i + 1][j + 1];
                } else if extended_springs[i] != '#'
                    && expanded_group_info[j] == ExpandedSpringStatus::GroupOfOperational
                {
                    value = dp[i + 1][j + 1] + dp[i + 1][j];
                }
                dp[i][j] = value;
            }
        }

        dp[0][0]
    }
}
