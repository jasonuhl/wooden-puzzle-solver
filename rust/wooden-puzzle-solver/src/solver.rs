use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Dingus {
    points: &'static [Point],
}

#[derive(Clone)]
struct Piece {
    fillers: HashMap<[usize; 3], Vec<Vec<[usize; 3]>>>,
    kind: usize,
}

type Point = [i32; 3];
type Universe = [[[i32; 3]; 3]; 3];

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

fn build_piece(dingus: &Dingus, rotate: bool, kind: usize) -> Piece {
    let mut orientations: Vec<Vec<Point>> = Vec::new();

    if rotate {
        for z_rotnum in 0..6 {
            for xy_rotnum in 0..4 {
                let mut rotated: Vec<Point> = dingus
                    .points
                    .iter()
                    .map(|point| rotate_point(*point, xy_rotnum, z_rotnum))
                    .collect();

                rotated.sort();
                let base = rotated[0];
                for p in rotated.iter_mut() {
                    p[0] -= base[0];
                    p[1] -= base[1];
                    p[2] -= base[2];
                }

                if !orientations.contains(&rotated) {
                    orientations.push(rotated);
                }
            }
        }
    } else {
        let mut sorted = dingus.points.to_vec();
        sorted.sort();
        orientations.push(sorted);
    }

    // Generate all valid placements
    let mut all_placements: Vec<Vec<[usize; 3]>> = Vec::new();

    for orientation in &orientations {
        for dx in 0..3i32 {
            for dy in 0..3i32 {
                for dz in 0..3i32 {
                    let translated: Vec<[i32; 3]> = orientation
                        .iter()
                        .map(|p| [p[0] + dx, p[1] + dy, p[2] + dz])
                        .collect();

                    if translated
                        .iter()
                        .all(|p| (0..3).contains(&p[0]) && (0..3).contains(&p[1]) && (0..3).contains(&p[2]))
                    {
                        let mut placement: Vec<[usize; 3]> = translated
                            .iter()
                            .map(|p| [p[0] as usize, p[1] as usize, p[2] as usize])
                            .collect();
                        placement.sort();
                        all_placements.push(placement);
                    }
                }
            }
        }
    }

    all_placements.sort();
    all_placements.dedup();

    // Build fillers map: group placements by their lowest cube (first after sorting)
    let mut fillers: HashMap<[usize; 3], Vec<Vec<[usize; 3]>>> = HashMap::new();
    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                fillers.insert([x, y, z], Vec::new());
            }
        }
    }

    for placement in &all_placements {
        fillers.get_mut(&placement[0]).unwrap().push(placement.clone());
    }

    Piece { fillers, kind }
}

fn find_first_empty(universe: &Universe) -> Option<[usize; 3]> {
    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                if universe[x][y][z] == 0 {
                    return Some([x, y, z]);
                }
            }
        }
    }
    None
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

    let piece_l = build_piece(&dingus_l, true, 0);
    let piece_t = build_piece(&dingus_t, true, 1);
    let piece_s = build_piece(&dingus_s, false, 2);
    let piece_r = build_piece(&dingus_r, true, 3);

    vec![
        piece_l.clone(),
        piece_l.clone(),
        piece_l.clone(),
        piece_l,
        piece_t,
        piece_s,
        piece_r,
    ]
}

struct Frame {
    universe: Universe,
    remaining_pieces: Vec<usize>,
    cell: [usize; 3],
    piece_iter_index: usize,
    placement_index: usize,
    last_kind: Option<usize>,
}

pub struct SolverIterator {
    pieces: Vec<Piece>,
    stack: Vec<Frame>,
}

impl SolverIterator {
    pub fn new() -> Self {
        let pieces = make_pieces();
        let remaining: Vec<usize> = (0..pieces.len()).collect();
        let stack = vec![Frame {
            universe: [[[0; 3]; 3]; 3],
            remaining_pieces: remaining,
            cell: [0, 0, 0],
            piece_iter_index: 0,
            placement_index: 0,
            last_kind: None,
        }];
        SolverIterator { pieces, stack }
    }
}

enum SearchResult {
    Solution(Universe),
    Recurse {
        universe: Universe,
        remaining_pieces: Vec<usize>,
        cell: [usize; 3],
    },
}

impl Iterator for SolverIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            let frame = self.stack.last()?;
            let universe = frame.universe;
            let cell = frame.cell;
            let remaining_pieces = frame.remaining_pieces.clone();
            let mut pi = frame.piece_iter_index;
            let mut pli = frame.placement_index;
            let mut last_kind = frame.last_kind;

            let mut result = None;

            'search: while pi < remaining_pieces.len() {
                let piece_idx = remaining_pieces[pi];
                let kind = self.pieces[piece_idx].kind;

                if last_kind == Some(kind) {
                    pi += 1;
                    pli = 0;
                    continue;
                }

                if let Some(placements) = self.pieces[piece_idx].fillers.get(&cell) {
                    while pli < placements.len() {
                        let placement = &placements[pli];
                        pli += 1;

                        if placement
                            .iter()
                            .all(|p| universe[p[0]][p[1]][p[2]] == 0)
                        {
                            let dingnum = self.stack.len() as i32;
                            let mut new_universe = universe;
                            for p in placement {
                                new_universe[p[0]][p[1]][p[2]] = dingnum;
                            }

                            match find_first_empty(&new_universe) {
                                None => {
                                    if is_canonical_under_cube_rotations(&new_universe) {
                                        result = Some(SearchResult::Solution(new_universe));
                                        break 'search;
                                    }
                                }
                                Some(next_cell) => {
                                    let mut new_remaining = remaining_pieces.clone();
                                    new_remaining.remove(pi);
                                    result = Some(SearchResult::Recurse {
                                        universe: new_universe,
                                        remaining_pieces: new_remaining,
                                        cell: next_cell,
                                    });
                                    break 'search;
                                }
                            }
                        }
                    }
                }

                last_kind = Some(kind);
                pi += 1;
                pli = 0;
            }

            match result {
                Some(SearchResult::Solution(u)) => {
                    let frame = self.stack.last_mut().unwrap();
                    frame.piece_iter_index = pi;
                    frame.placement_index = pli;
                    frame.last_kind = last_kind;
                    return Some(solution_block(&u));
                }
                Some(SearchResult::Recurse {
                    universe,
                    remaining_pieces,
                    cell,
                }) => {
                    let frame = self.stack.last_mut().unwrap();
                    frame.piece_iter_index = pi;
                    frame.placement_index = pli;
                    frame.last_kind = last_kind;

                    self.stack.push(Frame {
                        universe,
                        remaining_pieces,
                        cell,
                        piece_iter_index: 0,
                        placement_index: 0,
                        last_kind: None,
                    });
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
    fn build_piece_generates_correct_fillers() {
        let line = Dingus {
            points: &[[0, 0, 0], [1, 0, 0], [2, 0, 0]],
        };

        let piece = build_piece(&line, true, 0);

        // A 3-cube line has 3 orientations (along x, y, z axes)
        // Each orientation fits in exactly 9 positions (3 translations along
        // the non-line axes), so 3 * 9 = 27 total placements
        let total_placements: usize = piece.fillers.values().map(|v| v.len()).sum();
        assert_eq!(total_placements, 27);
    }

    #[test]
    fn s_piece_has_no_rotations() {
        let dingus_s = Dingus {
            points: &[[0, 0, 0], [1, 0, 0], [1, 1, 0], [2, 1, 0]],
        };

        let piece_no_rotate = build_piece(&dingus_s, false, 0);
        let piece_rotate = build_piece(&dingus_s, true, 0);

        let no_rotate_placements: usize = piece_no_rotate.fillers.values().map(|v| v.len()).sum();
        let rotate_placements: usize = piece_rotate.fillers.values().map(|v| v.len()).sum();

        // Without rotation, should have fewer placements than with rotation
        assert!(no_rotate_placements < rotate_placements);
        // The S-piece without rotation: 3x2x1 shape, valid translations give 6 placements
        assert_eq!(no_rotate_placements, 6);
    }

    #[test]
    fn solver_iterator_returns_nonempty() {
        let mut iter = SolverIterator::new();
        let first = iter.next().expect("expected at least one solution");
        assert!(first.starts_with("We win!\n"));
    }
}
