#[derive(Clone, Copy)]
struct Dingus {
    points: &'static [Point],
}

#[derive(Clone)]
struct Piece {
    orientations: Vec<Vec<Point>>,
}

type Point = [i32; 3];
type Universe = [[[i32; 3]; 3]; 3];

fn place_dingus(
    universe: &mut Universe,
    points: &[Point],
    origin: Point,
    dingnum: i32,
) -> bool {
    for point in points {
        let x = (point[0] + origin[0]) as usize;
        let y = (point[1] + origin[1]) as usize;
        let z = (point[2] + origin[2]) as usize;

        if x >= 3 || y >= 3 || z >= 3 {
            return false;
        }
        if universe[x][y][z] != 0 {
            return false;
        }
        universe[x][y][z] = dingnum;
    }
    true
}

fn rotate_xy(x: i32, y: i32, rotnum: i32) -> (i32, i32) {
    match rotnum {
        0 => (x, y),
        1 => (y, -x),
        2 => (-x, -y),
        3 => (-y, x),
        _ => panic!("invalid rotation"),
    }
}

fn rotate_z(x: i32, y: i32, z: i32, rotnum: i32) -> (i32, i32, i32) {
    match rotnum {
        0 => (x, y, z),
        1 => (x, z, -y),
        2 => (x, -y, -z),
        3 => (x, -z, y),
        4 => (-z, y, x),
        5 => (z, y, -x),
        _ => panic!("invalid rotation"),
    }
}

fn rotate_point(point: Point, xy_rotnum: i32, z_rotnum: i32) -> Point {
    let (x, y) = rotate_xy(point[0], point[1], xy_rotnum);
    let (x, y, z) = rotate_z(x, y, point[2], z_rotnum);
    [x, y, z]
}

fn normalize_points(points: &mut [Point]) {
    let min_x = points.iter().map(|point| point[0]).min().unwrap_or(0);
    let min_y = points.iter().map(|point| point[1]).min().unwrap_or(0);
    let min_z = points.iter().map(|point| point[2]).min().unwrap_or(0);

    for point in points.iter_mut() {
        point[0] -= min_x;
        point[1] -= min_y;
        point[2] -= min_z;
    }
}

fn rotate_universe(universe: &Universe, xy_rotnum: i32, z_rotnum: i32) -> Universe {
    let mut rotated = [[[0; 3]; 3]; 3];

    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                let point = [x as i32 - 1, y as i32 - 1, z as i32 - 1];
                let rotated_point = rotate_point(point, xy_rotnum, z_rotnum);
                let rx = (rotated_point[0] + 1) as usize;
                let ry = (rotated_point[1] + 1) as usize;
                let rz = (rotated_point[2] + 1) as usize;
                rotated[rx][ry][rz] = universe[x][y][z];
            }
        }
    }

    rotated
}

fn universe_key(universe: &Universe) -> [i32; 27] {
    let mut key = [0; 27];
    let mut index = 0;

    for z in 0..3 {
        for y in 0..3 {
            for x in 0..3 {
                key[index] = universe[x][y][z];
                index += 1;
            }
        }
    }

    key
}

fn is_canonical_under_cube_rotations(universe: &Universe) -> bool {
    let canonical_key = universe_key(universe);

    for z_rotnum in 0..6 {
        for xy_rotnum in 0..4 {
            if z_rotnum == 0 && xy_rotnum == 0 {
                continue;
            }

            let rotated = rotate_universe(universe, xy_rotnum, z_rotnum);
            if universe_key(&rotated) < canonical_key {
                return false;
            }
        }
    }

    true
}

fn build_piece(dingus: &Dingus) -> Piece {
    let mut orientations: Vec<Vec<Point>> = Vec::new();

    for z_rotnum in 0..6 {
        for xy_rotnum in 0..4 {
            let mut rotated = dingus
                .points
                .iter()
                .map(|point| rotate_point(*point, xy_rotnum, z_rotnum))
                .collect::<Vec<Point>>();

            normalize_points(&mut rotated);
            rotated.sort();

            if !orientations.contains(&rotated) {
                orientations.push(rotated);
            }
        }
    }

    Piece { orientations }
}

fn format_universe(universe: &Universe) -> String {
    let mut output = String::new();

    for z in (0..3).rev() {
        for y in (0..3).rev() {
            output.push_str(&format!(
                "{} {} {}\n",
                universe[0][y][z], universe[1][y][z], universe[2][y][z]
            ));
        }
        output.push('\n');
    }

    output
}

fn solution_block(universe: &Universe) -> String {
    let mut output = String::from("We win!\n");
    output.push_str(&format_universe(universe));
    output
}

fn make_pieces() -> Vec<Piece> {
    let dingus_l = Dingus {
        points: &[[0, 0, 0], [1, 0, 0], [2, 0, 0], [2, 1, 0]],
    };
    let dingus_t = Dingus {
        points: &[[0, 0, 0], [1, 0, 0], [1, 1, 0], [2, 0, 0]],
    };
    let dingus_s = Dingus {
        points: &[[0, 0, 0], [1, 0, 0], [1, 1, 0], [2, 1, 0]],
    };
    let dingus_r = Dingus {
        points: &[[0, 0, 0], [0, 1, 0], [1, 1, 0]],
    };

    let base_dingi = [
        dingus_l, dingus_l, dingus_l, dingus_l, dingus_t, dingus_s, dingus_r,
    ];

    for dingus in base_dingi.iter() {
        assert_eq!(dingus.points[0], [0, 0, 0]);
    }

    base_dingi.iter().map(build_piece).collect()
}

struct Frame {
    universe: Universe,
    piece_index: usize,
    dingnum: i32,
    x: usize,
    y: usize,
    z: usize,
    orientation_index: usize,
}

pub struct SolverIterator {
    pieces: Vec<Piece>,
    stack: Vec<Frame>,
}

impl SolverIterator {
    pub fn new() -> Self {
        let pieces = make_pieces();
        let stack = vec![Frame {
            universe: [[[0; 3]; 3]; 3],
            piece_index: 0,
            dingnum: 1,
            x: 0,
            y: 0,
            z: 0,
            orientation_index: 0,
        }];
        SolverIterator { pieces, stack }
    }
}

impl Iterator for SolverIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            let frame = self.stack.last()?;

            let piece_index = frame.piece_index;
            let dingnum = frame.dingnum;
            let universe = frame.universe;
            let num_orientations = self.pieces[piece_index].orientations.len();
            let is_last = piece_index == self.pieces.len() - 1;
            let mut x = frame.x;
            let mut y = frame.y;
            let mut z = frame.z;
            let mut oi = frame.orientation_index;

            let mut found = None;

            'outer: while x < 3 {
                while y < 3 {
                    while z < 3 {
                        if universe[x][y][z] == 0 {
                            while oi < num_orientations {
                                let orientation = &self.pieces[piece_index].orientations[oi];
                                let origin = [x as i32, y as i32, z as i32];
                                let mut new_universe = universe;
                                if place_dingus(&mut new_universe, orientation, origin, dingnum) {
                                    if is_last {
                                        if is_canonical_under_cube_rotations(&new_universe) {
                                            oi += 1;
                                            found = Some((new_universe, true));
                                            break 'outer;
                                        }
                                    } else {
                                        oi += 1;
                                        found = Some((new_universe, false));
                                        break 'outer;
                                    }
                                }
                                oi += 1;
                            }
                        }
                        z += 1;
                        oi = 0;
                    }
                    y += 1;
                    z = 0;
                    oi = 0;
                }
                x += 1;
                y = 0;
                z = 0;
                oi = 0;
            }

            match found {
                Some((new_universe, is_solution)) => {
                    let frame = self.stack.last_mut().unwrap();
                    frame.x = x;
                    frame.y = y;
                    frame.z = z;
                    frame.orientation_index = oi;

                    if is_solution {
                        return Some(solution_block(&new_universe));
                    } else {
                        self.stack.push(Frame {
                            universe: new_universe,
                            piece_index: piece_index + 1,
                            dingnum: dingnum + 1,
                            x: 0,
                            y: 0,
                            z: 0,
                            orientation_index: 0,
                        });
                    }
                }
                None => {
                    self.stack.pop();
                }
            }
        }
    }
}

pub fn stream_solutions<F: FnMut(String)>(mut on_solution_text: F) {
    for solution in SolverIterator::new() {
        on_solution_text(solution);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_xy_four_times_is_identity() {
        let point = [2, -1, 0];
        let mut rotated = point;
        for _ in 0..4 {
            rotated = rotate_point(rotated, 1, 0);
        }
        assert_eq!(rotated, point);
    }

    #[test]
    fn normalize_points_shifts_min_corner_to_origin() {
        let mut points = vec![[2, 3, 1], [4, 5, 2], [3, 3, 4]];
        normalize_points(&mut points);
        points.sort();

        assert_eq!(points[0], [0, 0, 0]);
        assert!(points.iter().all(|point| point[0] >= 0 && point[1] >= 0 && point[2] >= 0));
    }

    #[test]
    fn build_piece_deduplicates_equivalent_orientations() {
        let line = Dingus {
            points: &[[0, 0, 0], [1, 0, 0], [2, 0, 0]],
        };

        let piece = build_piece(&line);
        assert_eq!(piece.orientations.len(), 3);
    }

    #[test]
    fn solver_iterator_returns_nonempty() {
        let mut iter = SolverIterator::new();
        let first = iter.next().expect("expected at least one solution");
        assert!(first.starts_with("We win!\n"));
    }
}
