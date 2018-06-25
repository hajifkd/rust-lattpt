use std::collections::VecDeque;

pub struct ListNearPoints2D {
    uncompleted_rows: VecDeque<(u32, u32)>,
    unused_smallest_y: u32,
}

impl ListNearPoints2D {
    pub fn new() -> ListNearPoints2D {
        ListNearPoints2D {
            uncompleted_rows: VecDeque::new(),
            unused_smallest_y: 1,
        }
    }

    fn multiply(pts: Vec<(u32, u32)>) -> Vec<(i32, i32)> {
        pts.into_iter()
            .map(|(y, x)| (y as i32, x as i32))
            .flat_map(|(y, x)| {
                // by definition, y != 0
                let mut points = vec![(y, x), (-y, x)];
                if x != 0 {
                    points.push((y, -x));
                    points.push((-y, -x));
                }

                if x != y {
                    let s = points.len();
                    for i in 0..s {
                        let (y, x) = points[i];
                        points.push((x, y));
                    }
                }

                points.into_iter()
            })
            .collect()
    }
}

impl Iterator for ListNearPoints2D {
    type Item = Vec<(i32, i32)>;

    fn next(&mut self) -> Option<Self::Item> {
        // First, get smallest indices from uncompleted rows
        let mut min_len_sq = 0u64;
        let mut min_pts = vec![];
        let mut indices = vec![];

        self.uncompleted_rows
            .iter()
            .map(|&(y, x)| ((y, x), (x as u64).pow(2) + (y as u64).pow(2)))
            .enumerate()
            .for_each(|(i, (v, lsq))| {
                if min_len_sq == 0 || lsq < min_len_sq {
                    min_len_sq = lsq;
                    min_pts = vec![v];
                    indices = vec![i];
                } else if min_len_sq == lsq {
                    min_pts.push(v);
                    indices.push(i);
                }
            });

        let unused_len_sq = (self.unused_smallest_y as u64).pow(2);

        Some(ListNearPoints2D::multiply(
            if min_len_sq == 0 || unused_len_sq < min_len_sq {
                self.uncompleted_rows.push_back((self.unused_smallest_y, 1));
                let result = vec![(self.unused_smallest_y, 0)];
                self.unused_smallest_y += 1;
                result
            } else {
                let mut popflag = false;
                min_pts
                    .iter()
                    .zip(indices.iter())
                    .for_each(|(&(y, x), &i)| {
                        if y == x {
                            assert_eq!(i, 0);
                            popflag = true;
                        } else {
                            self.uncompleted_rows[i] = (y, x + 1);
                        }
                    });

                if popflag {
                    self.uncompleted_rows.pop_front().unwrap();
                }

                if unused_len_sq > min_len_sq {
                    min_pts
                } else {
                    // unused_len_sq == min_len_sq
                    self.uncompleted_rows.push_back((self.unused_smallest_y, 1));
                    min_pts.push((self.unused_smallest_y, 0));
                    self.unused_smallest_y += 1;
                    min_pts
                }
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn list_near_points() {
        let lnps = ::ListNearPoints2D::new();
        let pts: Vec<_> = lnps.take(6).collect();
        assert_eq!(
            pts,
            vec![
                vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
                vec![(1, 1), (-1, 1), (1, -1), (-1, -1)],
                vec![(2, 0), (-2, 0), (0, 2), (0, -2)],
                vec![
                    (2, 1),
                    (-2, 1),
                    (2, -1),
                    (-2, -1),
                    (1, 2),
                    (1, -2),
                    (-1, 2),
                    (-1, -2),
                ],
                vec![(2, 2), (-2, 2), (2, -2), (-2, -2)],
                vec![(3, 0), (-3, 0), (0, 3), (0, -3)],
            ]
        );
    }

    #[test]
    fn is_sorted() {
        let lnps = ::ListNearPoints2D::new();
        let limit = 10000;
        assert!(
            lnps.map(|x| x[0])
                .map(|(x, y)| x.pow(2) + y.pow(2))
                .take(limit)
                .fold((true, 0), |(res, p), x| (res && (p < x), x))
                .0
        );
    }

    #[test]
    fn is_completed() {
        let lnps = ::ListNearPoints2D::new();
        let limit: usize = 1000;
        let mut appearance = vec![0u8; limit * limit];
        lnps.take(limit * limit * 2).for_each(|points| {
            points
                .into_iter()
                .filter(|&(x, y)| x >= 0 && y >= 0 && y < (limit as i32) && x < (limit as i32))
                .for_each(|(x, y)| appearance[(x as usize) * limit + (y as usize)] += 1);
        });

        assert!(appearance.into_iter().skip(1).all(|x| x == 1u8));
    }
}
