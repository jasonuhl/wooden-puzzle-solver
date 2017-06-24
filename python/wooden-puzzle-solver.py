#!/usr/bin/env python
import copy
import os
import sys
import multiprocessing

threads = multiprocessing.cpu_count()


def instantiate_universe():
    x = [0 for n in xrange(3)]
    y = [copy.deepcopy(x) for n in xrange(3)]
    return [copy.deepcopy(y) for n in xrange(3)]


class Dingus:
    def __init__(self, cubes):
        self.cubes = cubes
        self.rotations = [
            rotate_z(rotate_xy(self.cubes, n), m)
            for m in xrange(6)
            for n in xrange(4)
            ]


def main():
    dingusL = Dingus([[0,0,0], [1,0,0], [2,0,0], [2,1,0]])
    dingusT = Dingus([[0,0,0], [1,0,0], [1,1,0], [2,0,0]])
    dingusS = Dingus([[0,0,0], [1,0,0], [1,1,0], [2,1,0]])
    dingusR = Dingus([[0,0,0], [0,1,0], [1,1,0]])
    dingi = [dingusL, dingusL, dingusL, dingusL, dingusT, dingusS, dingusR]
    dingusrotations = []

    dingusrotations = [
        dingus.rotations
        for dingus in dingi
        ]

    universe = instantiate_universe()

    if threads:
        pids = []
        for j in xrange(threads):
            pid = os.fork()
            if pid:
                pids.append(pid)
            else:
                turn_the_crank(universe, dingusrotations, j, threads)
                return

        for pid in pids:
            os.waitpid(pid, 0)

    else:
        turn_the_crank(universe, dingusrotations, 0, 1)


def turn_the_crank(universe, dingusrotations, thread, threads):
    if len(dingusrotations)==0:
        print 'We win!'
        print_universe(universe)
        return True
    for x in xrange(3):
        for y in xrange(3):
            for z in xrange(3):

                if universe[x][y][z] != 0:
                    continue
                for i in xrange(len(dingusrotations[0])):
                    if len(dingusrotations)==7:
                        if (i%threads != thread):
                            continue

                    olduniverse = copy.deepcopy(universe)
                    if place_dingus(universe, dingusrotations[0][i], [x,y,z], 8-len(dingusrotations)):
                        turn_the_crank(copy.deepcopy(universe), dingusrotations[1:], thread, threads)
                    universe = olduniverse

    return False


def rotate_xy(dingus, rotnum):
    if rotnum == 0:
        return dingus
    if rotnum == 1:
        return [(lambda cube: [cube[1], -cube[0], cube[2]])(cube) for cube in dingus]
    if rotnum == 2:
        return [(lambda cube: [-cube[0], -cube[1], cube[2]])(cube) for cube in dingus]
    if rotnum == 3:
        return [(lambda cube: [-cube[1], cube[0], cube[2]])(cube) for cube in dingus]


def print_universe(universe):
    for z in xrange(2,-1,-1):
        for y in xrange(2,-1,-1):
            for x in xrange(3):
                print universe[x][y][z],
            print
        print


def rotate_z(dingus, rotnum):
    if rotnum == 0:
        return dingus
    if rotnum == 1:
        return [(lambda cube: [cube[0], cube[2], -cube[1]])(cube) for cube in dingus]
    if rotnum == 2:
        return [(lambda cube: [cube[0], -cube[1], -cube[2]])(cube) for cube in dingus]
    if rotnum == 3:
        return [(lambda cube: [cube[0], -cube[2], cube[1]])(cube) for cube in dingus]
    if rotnum == 4:
        return [(lambda cube: [-cube[2], cube[1], cube[0]])(cube) for cube in dingus]
    if rotnum == 5:
        return [(lambda cube: [cube[2], cube[1], -cube[0]])(cube) for cube in dingus]


def place_dingus(universe, dingus, origin, dingnum):
    d = copy.deepcopy(dingus)
    d = rel_to_abs(d, origin)
    for cube in d:

        for n in xrange(len(cube)):
            if (cube[n] > 2) or (cube[n] < 0):
                return False
        if universe[cube[0]][cube[1]][cube[2]] != 0:
            return False

    for cube in d:
        universe[cube[0]][cube[1]][cube[2]] = dingnum
    return True


def rel_to_abs(dingus, origin):
    for cube in dingus:
        for n in xrange(len(cube)):
            cube[n] = cube[n] + origin[n]
    return dingus


if __name__ == '__main__':
    main()
