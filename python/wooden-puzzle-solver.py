#!/usr/bin/env python
import copy
import os
import sys
import multiprocessing

#threads = multiprocessing.cpu_count()
threads = None

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
    def __init__(self, name, cubes, rotate=True):
        self.name = name
        self.cubes = cubes

        if rotate:
            rotations = (
                rotate_z(rotate_xy(self.cubes, n), m)
                for m in xrange(6)
                for n in xrange(4)
            )
            rotations = (sorted(cubes) for cubes in rotations)
            rotations = (
                translate(cubes, (-cubes[0][0], -cubes[0][1], -cubes[0][2]))
                for cubes in rotations
            )
            self.rotations = list(nodups(sorted(rotations)))
            sys.stderr.write("Dingus %s: found %d rotations\n" % (self.name, len(self.rotations),))
        else:
            self.rotations = [sorted(self.cubes)]
            sys.stderr.write("Dingus %s: ignoring rotations\n" % (self.name,))

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
            if valid(cubes)
            )
        self.placements = list(nodups(placements))

        # A map of placements based on their "lowest" cube:
        self.fillers = {}
        for space in SPACES:
            self.fillers[space] = []
        for placement in placements:
            self.fillers[placement[0]].append(placement)

        sys.stderr.write("          found %d placements\n" % (len(self.placements),))


def main():
    dingusL = Dingus("L", [(0,0,0), (1,0,0), (2,0,0), (2,1,0)])
    dingusT = Dingus("T", [(0,0,0), (1,0,0), (1,1,0), (2,0,0)])
    dingusS = Dingus("S", [(0,0,0), (1,0,0), (1,1,0), (2,1,0)], rotate=False)
    dingusR = Dingus("R", [(0,0,0), (0,1,0), (1,1,0)])
    dingi = [dingusL, dingusL, dingusL, dingusL, dingusT, dingusS, dingusR]

    universe = instantiate_universe()

    if threads:
        pids = []
        for j in xrange(threads):
            pid = os.fork()
            if pid:
                pids.append(pid)
            else:
                fill(universe, dingi, j, threads)
                return

        for pid in pids:
            os.waitpid(pid, 0)

    else:
        fill(universe, dingi, 0, 1)


def fill(universe, dingi, thread, threads, to_fill=None):
    """Try to fill universe.

    Try to fill universe given that all spaces not in the `empty` list have
    already been filled."""

    if to_fill is None:
        to_fill = list(reversed(SPACES))
    elif not to_fill:
        print_universe(universe)
        return

    space = to_fill.pop()
    if universe[space[0]][space[1]][space[2]] != 0:
        # Space is already filled; continue to the next space:
        fill(universe, dingi, thread, threads, to_fill)
    else:
        # Find a dingus that can fill space. We only need to look
        # among dingus.fillers[space] because any other positions of a
        # dingus would protrude into the spaces that we have already
        # filled.
        last = None
        for i in range(len(dingi)):
            dingus = dingi.pop(i)
            if dingus != last:
                for placement in dingus.fillers[space]:
                    if placeable(universe, placement):
                        place_cubes(universe, placement, 7 - len(dingi))
                        fill(universe, dingi, thread, threads, to_fill)
                        place_cubes(universe, placement, 0)
                last = dingus
            dingi.insert(i, dingus)
    to_fill.append(space)


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


def valid(cubes):
    for cube in cubes:
        if not all(0 <= coord < 3 for coord in cube):
            return False

    return True


def translate(cubes, origin):
    newcubes = []
    for cube in cubes:
        newcubes.append(tuple(cube[i] + origin[i] for i in xrange(len(cube))))

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
    for y in xrange(3):
        for x in xrange(3):
            if x != 0:
                print '   ',
            for z in xrange(3):
                print universe[x][y][z],
        print
    print


if __name__ == '__main__':
    main()
