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

    def test_fill_emits_expected_solution_format_without_search(self):
        universe = solver.instantiate_universe()
        for x in range(3):
            for y in range(3):
                for z in range(3):
                    universe[x][y][z] = 100 * x + 10 * y + z

        output = io.StringIO()
        with redirect_stdout(output):
            solver.fill(universe, [], 0, 1, to_fill=[])

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


if __name__ == "__main__":
    unittest.main()
