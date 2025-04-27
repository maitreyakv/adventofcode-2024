use std::ops::Range;

#[derive(Debug)]
pub struct Array2D<T> {
    n_rows: usize,
    n_cols: usize,
    data: Vec<T>,
}

impl<T> Array2D<T> {
    pub fn from_rows(rows: impl IntoIterator<Item = impl IntoIterator<Item = T>>) -> Self {
        let mut rows = rows.into_iter();
        let first_row: Vec<T> = rows.next().unwrap().into_iter().collect();
        let n_cols = first_row.len();

        let mut data = first_row;
        data.extend(rows.flatten());

        let n_rows = data.len().div_ceil(n_cols);

        Self {
            n_rows,
            n_cols,
            data,
        }
    }

    pub fn n_rows(&self) -> usize {
        self.n_rows
    }

    pub fn n_cols(&self) -> usize {
        self.n_cols
    }

    pub fn rows(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.n_rows).map(|row_idx| {
            (0..self.n_cols)
                .map(|col_idx| &self[row_idx][col_idx])
                .collect()
        })
    }

    pub fn cols(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.n_cols).map(|col_idx| {
            (0..self.n_rows)
                .map(|row_idx| &self[row_idx][col_idx])
                .collect()
        })
    }

    // NOTE: This could return a ConvBuilder class to add stride and padding info before running.
    // The ConvBuilder could even hold the function until running.
    pub fn convolve<K, R>(&self, kernel_func: K, kernel_size: (usize, usize)) -> Array2D<R>
    where
        K: Fn(Array2D<&T>) -> R,
    {
        let output_n_rows = self.n_rows - kernel_size.0 + 1;
        let output_n_cols = self.n_cols - kernel_size.1 + 1;
        let mut output: Vec<R> = Vec::with_capacity(output_n_rows * output_n_cols);

        for i in 0..output_n_rows {
            for j in 0..output_n_cols {
                let kernel_input = {
                    let slice_rows = i..(i + kernel_size.0);
                    let slice_cols = j..(j + kernel_size.1);
                    self.slice(slice_rows, slice_cols)
                };
                output.push(kernel_func(kernel_input))
            }
        }

        Array2D::<R>::from_row_major_vec(output, (output_n_rows, output_n_cols))
    }
}

impl<T> Array2D<T>
where
    T: for<'a> std::iter::Sum<&'a T>,
{
    pub fn sum(&self) -> T {
        self.data.iter().sum()
    }
}

impl<T> Array2D<T> {
    fn from_row_major_vec(v: Vec<T>, size: (usize, usize)) -> Self {
        Self {
            n_rows: size.0,
            n_cols: size.1,
            data: v,
        }
    }

    fn slice(&self, row_idxs: Range<usize>, col_idxs: Range<usize>) -> Array2D<&T> {
        let n_rows = row_idxs.end - row_idxs.start;
        let n_cols = col_idxs.end - col_idxs.start;
        let mut items = Vec::with_capacity(n_rows * n_cols);
        for row_idx in row_idxs {
            let col_idxs = col_idxs.clone();
            for col_idx in col_idxs {
                items.push(&self[row_idx][col_idx])
            }
        }
        Array2D::from_row_major_vec(items, (n_rows, n_cols))
    }
}

impl<T> std::ops::Index<usize> for Array2D<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.n_cols;
        let end = start + self.n_cols;
        &self.data[start..end]
    }
}
