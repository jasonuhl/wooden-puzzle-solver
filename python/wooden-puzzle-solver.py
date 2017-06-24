#!/usr/bin/env python
import copy
import os
import sys
import multiprocessing

threads = multiprocessing.cpu_count()


SPACES = [
    (x,y,z)
    for x in range(3)
    for y in range(3)
    for z in range(3)
    ]


def instantiate_universe():
    x = [0 for n in xrange(3)]
    y = [copy.deepcopy(x) for n in xrange(3)]
    return [copy.deepcopy(y) for n in xrange(3)]


def nodups(items):
    items = iter(items)
    try:
        last = items.next()
        yield last
        while True:
            item = items.next()
            if item == last:
                continue
            else:
                yield item
                last = item
    except StopIteration:
        return


class Dingus:
    def __init__(self, cubes):
        self.cubes = cubes
        self.rotations = [
            rotate_z(rotate_xy(self.cubes, n), m)
            for m in xrange(6)
            for n in xrange(4)
            ]
        placements = [
            translate(rotation, (dx, dy, dz))
            for rotation in self.rotations
            for dx in xrange(3)
            for dy in xrange(3)
            for dz in xrange(3)
            ]
        placements = sorted(
            sorted(cubes)
            for cubes in placements
            if cubes is not None
            )
        self.placements = list(nodups(placements))


def main():
    dingusL = Dingus([(0,0,0), (1,0,0), (2,0,0), (2,1,0)])
    dingusT = Dingus([(0,0,0), (1,0,0), (1,1,0), (2,0,0)])
    dingusS = Dingus([(0,0,0), (1,0,0), (1,1,0), (2,1,0)])
    dingusR = Dingus([(0,0,0), (0,1,0), (1,1,0)])
    dingi = [dingusL, dingusL, dingusL, dingusL, dingusT, dingusS, dingusR]

    universe = instantiate_universe()

    if threads:
        pids = []
        for j in xrange(threads):
            pid = os.fork()
            if pid:
                pids.append(pid)
            else:
                turn_the_crank(universe, dingi, j, threads)
                return

        for pid in pids:
            os.waitpid(pid, 0)

    else:
        turn_the_crank(universe, dingi, 0, 1)


def turn_the_crank(universe, dingi, thread, threads):
    if not dingi:
        print 'We win!'
        print_universe(universe)
        return True

    for i in xrange(len(dingi[0].placements)):
        if len(dingi) == 7 and i % threads != thread:
            continue

        olduniverse = copy.deepcopy(universe)
        if placeable(universe, dingi[0].placements[i]):
            place_cubes(universe, dingi[0].placements[i], 8 - len(dingi))
            turn_the_crank(copy.deepcopy(universe), dingi[1:], thread, threads)
        universe = olduniverse

    return False


def rotate_xy(cubes, rotnum):
    if rotnum == 0:
        return cubes
    if rotnum == 1:
        return tuple((lambda cube: [cube[1], -cube[0], cube[2]])(cube) for cube in cubes)
    if rotnum == 2:
        return tuple((lambda cube: [-cube[0], -cube[1], cube[2]])(cube) for cube in cubes)
    if rotnum == 3:
        return tuple((lambda cube: [-cube[1], cube[0], cube[2]])(cube) for cube in cubes)


def rotate_z(cubes, rotnum):
    if rotnum == 0:
        return cubes
    if rotnum == 1:
        return tuple((lambda cube: [cube[0], cube[2], -cube[1]])(cube) for cube in cubes)
    if rotnum == 2:
        return tuple((lambda cube: [cube[0], -cube[1], -cube[2]])(cube) for cube in cubes)
    if rotnum == 3:
        return tuple((lambda cube: [cube[0], -cube[2], cube[1]])(cube) for cube in cubes)
    if rotnum == 4:
        return tuple((lambda cube: [-cube[2], cube[1], cube[0]])(cube) for cube in cubes)
    if rotnum == 5:
        return tuple((lambda cube: [cube[2], cube[1], -cube[0]])(cube) for cube in cubes)


def translate(cubes, origin):
    newcubes = []
    for cube in cubes:
        newcube = tuple(cube[i] + origin[i] for i in xrange(len(cube)))
        if not all(0 <= coord < 3 for coord in newcube):
            return None
        newcubes.append(newcube)

    return newcubes


def placeable(universe, cubes):
    for cube in cubes:
        if universe[cube[0]][cube[1]][cube[2]] != 0:
            return False

    return True


def place_cubes(universe, cubes, dingnum):
    for cube in cubes:
        universe[cube[0]][cube[1]][cube[2]] = dingnum


def print_universe(universe):
    for z in xrange(2,-1,-1):
        for y in xrange(2,-1,-1):
            for x in xrange(3):
                print universe[x][y][z],
            print
        print


if __name__ == '__main__':
    main()
