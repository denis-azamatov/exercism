pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len() as i32;
    if height == 0 {
        return vec![];
    }

    let width = minefield[0].len() as i32;
    if width == 0 {
        return minefield.iter().map(|&x| x.to_owned()).collect();
    }

    let grid: Vec<&u8> = minefield
        .iter()
        .map(|&x| x.as_bytes())
        .flat_map(|x| x)
        .collect();

    let filled_grid: Vec<u8> = grid
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if **x == '*' as u8 {
                return '*' as u8;
            }
            let neighbors = get_neighbors(i as i32, width, height);
            let mut count = 0;
            for idx in neighbors.iter().filter(|x| x.is_some()).map(|x| x.unwrap()) {
                let cell = grid[idx as usize];
                if *cell == '*' as u8 {
                    count += 1;
                }
            }

            match count {
                0 => ' ' as u8,
                _ => count.to_string().as_bytes()[0],
            }
        })
        .collect();

    unsafe {
        filled_grid
            .chunks(width as usize)
            .map(|x| String::from_utf8_unchecked(x.to_vec()))
            .collect()
    }
}

fn get_neighbors(idx: i32, width: i32, height: i32) -> [Option<i32>; 8] {
    let row = idx / width;

    let normalized_idx = idx % width;
    let normalized_left = normalized_idx - 1;
    let normalized_right = normalized_idx + 1;

    let is_top = row == 0;
    let is_bottom = row == height - 1;
    let is_left = normalized_left < 0;
    let is_right = normalized_right >= width;

    let left = (!is_left).then(|| denormalize(normalized_left, row, width));
    let right = (!is_right).then(|| denormalize(normalized_right, row, width));

    let top = (!is_top).then(|| idx - width);
    let bottom = (!is_bottom).then(|| idx + width);

    let top_left = (!(is_top || is_left)).then(|| idx - width - 1);
    let top_right = (!(is_top || is_right)).then(|| idx - width + 1);

    let bottom_left = (!(is_bottom || is_left)).then(|| idx + width - 1);
    let bottom_right = (!(is_bottom || is_right)).then(|| idx + width + 1);

    [
        top_left,
        top,
        top_right,
        right,
        bottom_right,
        bottom,
        bottom_left,
        left,
    ]
}

fn denormalize(v: i32, row: i32, width: i32) -> i32 {
    v + (row * width)
}

#[cfg(test)]
mod test {
    use super::annotate;

    fn remove_annotations(board: &[&str]) -> Vec<String> {
        board.iter().map(|r| remove_annotations_in_row(r)).collect()
    }

    fn remove_annotations_in_row(row: &str) -> String {
        row.chars()
            .map(|ch| match ch {
                '*' => '*',
                _ => ' ',
            })
            .collect()
    }

    fn run_test(test_case: &[&str]) {
        let cleaned = remove_annotations(test_case);
        let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
        let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, annotate(&cleaned_strs));
    }

    #[test]
    fn no_rows() {
        #[rustfmt::skip]
    run_test(&[
    ]);
    }
    #[test]
    fn no_columns() {
        #[rustfmt::skip]
    run_test(&[
        "",
    ]);
    }
    #[test]
    fn no_mines() {
        #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
    }
    #[test]
    fn board_with_only_mines() {
        #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
    }
    #[test]
    fn mine_surrounded_by_spaces() {
        #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
    }
    #[test]
    fn space_surrounded_by_mines() {
        #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
    }
    #[test]
    fn horizontal_line() {
        #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
    }
    #[test]
    fn horizontal_line_mines_at_edges() {
        #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
    }
    #[test]
    fn vertical_line() {
        #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
    }
    #[test]
    fn vertical_line_mines_at_edges() {
        #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
    }
    #[test]
    fn cross() {
        #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
    }
    #[test]
    fn large_board() {
        #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
    }
}
