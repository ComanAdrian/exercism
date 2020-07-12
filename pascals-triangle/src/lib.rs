pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result: Vec<Vec<u32>> = vec![];

        for i in 0..self.row_count as usize {
            result.push(vec![]);
            result[i].push(1);

            for j in 1..i as usize {
                let left = result[i - 1][j - 1];
                let right = result[i - 1][j];

                result[i].push(left + right);
            }

            if i != 0 {
                result[i].push(1);
            }
        }

        result
    }
}
