import importlib.util
import io
import pathlib
import unittest
from contextlib import redirect_stdout


MODULE_PATH = pathlib.Path(__file__).resolve().parents[1] / "wooden-puzzle-solver.py"
SPEC = importlib.util.spec_from_file_location("wooden_puzzle_solver", MODULE_PATH)
solver = importlib.util.module_from_spec(SPEC)
assert SPEC is not None and SPEC.loader is not None
SPEC.loader.exec_module(solver)


class WoodenPuzzleSolverTests(unittest.TestCase):
    def test_instantiate_universe_shape_and_independence(self):
        universe = solver.instantiate_universe()

        self.assertEqual(len(universe), 3)
        self.assertEqual(len(universe[0]), 3)
        self.assertEqual(len(universe[0][0]), 3)

        universe[0][0][0] = 99
        self.assertEqual(universe[0][0][0], 99)
        self.assertEqual(universe[1][0][0], 0)
        self.assertEqual(universe[0][1][0], 0)

    def test_nodups_removes_consecutive_duplicates(self):
        values = [1, 1, 2, 2, 3, 3, 3, 2, 2]
        result = list(solver.nodups(values))
        self.assertEqual(result, [1, 2, 3, 2])

    def test_rotate_xy_quarter_turn(self):
        cubes = ((1, 0, 0), (2, 1, 0))
        rotated = solver.rotate_xy(cubes, 1)
        self.assertEqual(list(rotated), [[0, -1, 0], [1, -2, 0]])

    def test_rotate_z_variant(self):
        cubes = ((1, 2, 0),)
        rotated = solver.rotate_z(cubes, 4)
        self.assertEqual(list(rotated), [[0, 2, 1]])

    def test_translate_and_valid(self):
        cubes = [(0, 0, 0), (1, 1, 1)]
        translated = solver.translate(cubes, (1, 1, 1))

        self.assertEqual(translated, [(1, 1, 1), (2, 2, 2)])
        self.assertTrue(solver.valid(translated))
        self.assertFalse(solver.valid([(3, 0, 0)]))

    def test_placeable_and_place_cubes(self):
        universe = solver.instantiate_universe()
        cubes = [(0, 0, 0), (1, 1, 1)]

        self.assertTrue(solver.placeable(universe, cubes))
        solver.place_cubes(universe, cubes, 7)
        self.assertFalse(solver.placeable(universe, cubes))

    def test_dingus_without_rotation_keeps_one_rotation(self):
        dingus = solver.Dingus("S", [(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 1, 0)], rotate=False)

        self.assertEqual(len(dingus.rotations), 1)
        self.assertGreater(len(dingus.placements), 0)

    def test_print_universe_format(self):
        universe = solver.instantiate_universe()
        for x in range(3):
            for y in range(3):
                for z in range(3):
                    universe[x][y][z] = 100 * x + 10 * y + z

        output = io.StringIO()
        with redirect_stdout(output):
            solver.print_universe(universe)

        non_empty_lines = [line for line in output.getvalue().splitlines() if line.strip()]
        self.assertEqual(len(non_empty_lines), 3)

        expected_rows = [
            [0, 1, 2, 100, 101, 102, 200, 201, 202],
            [10, 11, 12, 110, 111, 112, 210, 211, 212],
            [20, 21, 22, 120, 121, 122, 220, 221, 222],
        ]
        for row_index, line in enumerate(non_empty_lines):
            values = [int(token) for token in line.split()]
            self.assertEqual(values, expected_rows[row_index])

    def test_is_canonical_exactly_one_rotation_is_canonical(self):
        # Fill universe with distinct values and find the canonical rotation.
        universe = solver.instantiate_universe()
        val = 1
        for x in range(3):
            for y in range(3):
                for z in range(3):
                    universe[x][y][z] = val
                    val += 1

        # Exactly one of the 24 rotations should be canonical.
        canonical_count = 0
        for z_rot in range(6):
            for xy_rot in range(4):
                rotated = solver.rotate_world(universe, xy_rot, z_rot)
                if solver.is_canonical(rotated):
                    canonical_count += 1
        self.assertEqual(canonical_count, 1)

    def test_is_canonical_rejects_non_canonical(self):
        # Find a canonical orientation, then rotate it and confirm rejection.
        universe = solver.instantiate_universe()
        val = 1
        for x in range(3):
            for y in range(3):
                for z in range(3):
                    universe[x][y][z] = val
                    val += 1

        # Find the canonical rotation first.
        canonical = None
        for z_rot in range(6):
            for xy_rot in range(4):
                rotated = solver.rotate_world(universe, xy_rot, z_rot)
                if solver.is_canonical(rotated):
                    canonical = rotated
                    break
            if canonical:
                break

        # Rotate the canonical universe; the result should not be canonical.
        rotated = solver.rotate_world(canonical, 1, 0)
        self.assertFalse(solver.is_canonical(rotated))

    def test_rotate_world_round_trip(self):
        universe = solver.instantiate_universe()
        val = 1
        for x in range(3):
            for y in range(3):
                for z in range(3):
                    universe[x][y][z] = val
                    val += 1
        # Four quarter-turns in xy should return to original.
        u = universe
        for _ in range(4):
            u = solver.rotate_world(u, 1, 0)
        self.assertEqual(solver.universe_key(u), solver.universe_key(universe))

    def test_dingus_rotation_counts(self):
        # Known rotation counts for each piece shape.
        dingusL = solver.Dingus("L", [(0,0,0), (1,0,0), (2,0,0), (2,1,0)])
        dingusT = solver.Dingus("T", [(0,0,0), (1,0,0), (1,1,0), (2,0,0)])
        dingusR = solver.Dingus("R", [(0,0,0), (0,1,0), (1,1,0)])
        dingusS = solver.Dingus("S", [(0,0,0), (1,0,0), (1,1,0), (2,1,0)], rotate=False)
        self.assertEqual(len(dingusL.rotations), 24)
        self.assertEqual(len(dingusT.rotations), 12)
        self.assertEqual(len(dingusR.rotations), 12)
        self.assertEqual(len(dingusS.rotations), 1)

    def test_solver_finds_415_canonical_solutions(self):
        """Run the full solver and verify it finds exactly 415 canonical solutions."""
        dingusL = solver.Dingus("L", [(0,0,0), (1,0,0), (2,0,0), (2,1,0)])
        dingusT = solver.Dingus("T", [(0,0,0), (1,0,0), (1,1,0), (2,0,0)])
        dingusS = solver.Dingus("S", [(0,0,0), (1,0,0), (1,1,0), (2,1,0)], rotate=False)
        dingusR = solver.Dingus("R", [(0,0,0), (0,1,0), (1,1,0)])
        dingi = [dingusL, dingusL, dingusL, dingusL, dingusT, dingusS, dingusR]

        universe = solver.instantiate_universe()
        output = io.StringIO()
        with redirect_stdout(output):
            count = solver.fill(universe, dingi)

        self.assertEqual(count, 415)

        # Each solution is 3 non-empty lines followed by 1 blank line.
        lines = output.getvalue().splitlines()
        non_empty = [l for l in lines if l.strip()]
        self.assertEqual(len(non_empty), 415 * 3)

        # Verify each solution row has exactly 9 values (3 slices x 3 columns).
        for line in non_empty:
            values = line.split()
            self.assertEqual(len(values), 9)
            for v in values:
                self.assertIn(int(v), range(1, 8))


if __name__ == "__main__":
    unittest.main()
