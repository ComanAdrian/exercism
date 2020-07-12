use std::ops::Range;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    let min_in_columns_precalculated = (Range {start: 0, end: input[0].len()})
        .map(|col_index| {
            let mut vec_col: Vec<u64> = Vec::new();

            for vec_row in input.into_iter() {
                vec_col.push(vec_row[col_index])
            }
            vec_col.into_iter().min()
        }).collect::<Vec<_>>();

    for (vec_i, vec) in input.into_iter().enumerate() {
        let max_in_rows = vec.into_iter().max();
        for (i, n) in vec.into_iter().enumerate() {
            if Some(n) == max_in_rows && Some(*n) == min_in_columns_precalculated[i] {
                result.push((vec_i, i));
            }
        }
    }
    result
}
