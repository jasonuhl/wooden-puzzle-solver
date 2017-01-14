struct Dingus<'a> {                 // a puzzle piece consisting of 3 or 4 points
    points: &'a[Point],
}

type Point = [i32; 3];              // x,y,z in full cartesian space (can be negative)
type Universe = [[[i32; 3];3];3];   // dingus numbers on a 3*3*3 grid

fn place_dingus(universe: &mut Universe, dingus: &Dingus, origin: Point, xy_rotnum: i32, z_rotnum: i32, dingnum: i32) -> bool {
    for point in dingus.points {
        let (untranslated_x, untranslated_y) = rotate_xy(point[0], point[1], xy_rotnum);
        let untranslated_z = point[2];
        let (untranslated_x, untranslated_y, untranslated_z) = rotate_z(untranslated_x, untranslated_y, untranslated_z, z_rotnum);

        let x = (untranslated_x + origin[0]) as usize;
        let y = (untranslated_y + origin[1]) as usize;
        let z = (untranslated_z + origin[2]) as usize;

        if x >= 3 || y >= 3 || z >= 3 {
            return false;   // off edge
        }
        if universe[x][y][z] != 0 {
            return false;   // collision
        }
        universe[x][y][z] = dingnum;
    }
    true
}

fn print_universe(universe: &Universe) {
    let mut z = 2;
    loop {
        let mut y = 2;
        loop {
            println!("{} {} {}", universe[0][y][z], universe[1][y][z], universe[2][y][z]);
            if y == 0 {
                break;
            }
            y -= 1;
        }
        println!("");
        if z == 0 {
            break;
        }
        z -= 1;
    }
}

fn rotate_xy(x: i32, y: i32, rotnum: i32) -> (i32, i32) {
    match rotnum {
        0 => { ( x,  y) },
        1 => { ( y, -x) },
        2 => { (-x, -y) },
        3 => { (-y,  x) },
        _ => { panic!("invalid rotation"); },
    }
}

fn rotate_z(x: i32, y: i32, z: i32, rotnum: i32) -> (i32, i32, i32) {
    match rotnum {
        0 => { ( x,  y,  z) },
        1 => { ( x,  z, -y) },
        2 => { ( x, -y, -z) },
        3 => { ( x, -z,  y) },
        4 => { (-z,  y,  x) },
        5 => { ( z,  y, -x) },
        _ => { panic!("invalid rotation"); },
    }
}

fn turn_the_crank(universe: &Universe, dingi: &[&Dingus]) {
    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
                if universe[x][y][z] != 0 {
                    continue;   // space already occupied
                }
                let origin = [x as i32, y as i32, z as i32];

                for z_rotnum in 0..6 {
                    for xy_rotnum in 0..4 {
                        let mut newuniverse = universe.clone();
                        let ret = place_dingus(&mut newuniverse, &dingi[0], origin, xy_rotnum, z_rotnum, 8 - dingi.len() as i32);
                        if ret == false {
                            continue;   // dingus failed to fit here
                        }
                        if dingi.len() == 1 {
                            println!("We win!");
                            print_universe(&newuniverse);
                        } else {
                            turn_the_crank(&newuniverse, &dingi[1..]);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let dingus_l = Dingus {points: &[[0,0,0], [1,0,0], [2,0,0], [2,1,0]]};
    let dingus_t = Dingus {points: &[[0,0,0], [1,0,0], [1,1,0], [2,0,0]]};
    let dingus_s = Dingus {points: &[[0,0,0], [1,0,0], [1,1,0], [2,1,0]]};
    let dingus_r = Dingus {points: &[[0,0,0], [0,1,0], [1,1,0]]};
    let dingi = [&dingus_l, &dingus_l, &dingus_l, &dingus_l, &dingus_t, &dingus_s, &dingus_r];

    /* The placement code assumes every dingus is defined starting at
       the origin ([0,0,0]) before any rotation/translation.
     */
    for dingus in dingi.iter() {
        assert_eq!(dingus.points[0], [0,0,0]);
    }

    let universe = [[[0; 3];3];3];

    turn_the_crank(&universe, &dingi);
}
