package main

import (
	"bufio"
	"bytes"
	"testing"
)

func TestRotateXYFourTimesIdentity(t *testing.T) {
	cube := Cube{1, 2, 0}
	rotated := Repeat(RotateXY, 4)(cube)
	if rotated != cube {
		t.Fatalf("expected identity after 4 XY rotations, got %v want %v", rotated, cube)
	}
}

func TestCubeListSetDistinct(t *testing.T) {
	set := CubeListSet{
		{{0, 0, 0}, {1, 0, 0}},
		{{0, 0, 0}, {1, 0, 0}},
		{{0, 0, 0}, {0, 1, 0}},
		{{0, 0, 0}, {0, 1, 0}},
	}

	distinct := set.Distinct()
	if len(distinct) != 2 {
		t.Fatalf("expected 2 distinct elements, got %d", len(distinct))
	}
}

func TestWorldValidityAndTranslations(t *testing.T) {
	var w World

	if !w.ValidCube(Cube{0, 0, 0}) {
		t.Fatalf("expected origin cube to be valid")
	}
	if w.ValidCube(Cube{3, 0, 0}) {
		t.Fatalf("expected out-of-bounds cube to be invalid")
	}

	single := CubeList{{0, 0, 0}}
	translatedSingle := w.AllTranslations(single)
	if len(translatedSingle) != 27 {
		t.Fatalf("expected 27 translations for single cube, got %d", len(translatedSingle))
	}

	domino := CubeList{{0, 0, 0}, {1, 0, 0}}
	translatedDomino := w.AllTranslations(domino)
	if len(translatedDomino) != 18 {
		t.Fatalf("expected 18 translations for x-domino, got %d", len(translatedDomino))
	}
}

func TestFitsPlaceAndUnplace(t *testing.T) {
	var w World
	cubes := CubeList{{0, 0, 0}, {1, 1, 1}}

	if !w.Fits(cubes) {
		t.Fatalf("expected cubes to fit in empty world")
	}

	w.Place(cubes, '7')
	if w.Fits(cubes) {
		t.Fatalf("expected cubes not to fit after placement")
	}
	if w[0][0][0] != '7' || w[1][1][1] != '7' {
		t.Fatalf("expected placed values to be set")
	}

	w.Unplace(cubes)
	if !w.Fits(cubes) {
		t.Fatalf("expected cubes to fit after unplace")
	}
}

func TestSolverProduces415CanonicalSolutions(t *testing.T) {
	var world World
	DingusL := NewDingus(&world, "L", true, CubeList{{0, 0, 0}, {1, 0, 0}, {2, 0, 0}, {2, 1, 0}})
	DingusT := NewDingus(&world, "T", true, CubeList{{0, 0, 0}, {1, 0, 0}, {1, 1, 0}, {2, 0, 0}})
	DingusS := NewDingus(&world, "S", false, CubeList{{0, 0, 0}, {1, 0, 0}, {1, 1, 0}, {2, 1, 0}})
	DingusR := NewDingus(&world, "R", true, CubeList{{0, 0, 0}, {0, 1, 0}, {1, 1, 0}})
	Dingi := []*Dingus{&DingusL, &DingusL, &DingusL, &DingusL, &DingusT, &DingusS, &DingusR}

	solutions := make(chan World)
	go func() {
		world.Fill(Dingi, AllPositions, '1', solutions)
		close(solutions)
	}()

	var count int
	for w := range solutions {
		if !w.IsCanonical() {
			continue
		}
		// Validate every cell is filled with piece number '1'..'7'
		for x := 0; x < 3; x++ {
			for y := 0; y < 3; y++ {
				for z := 0; z < 3; z++ {
					v := w[x][y][z]
					if v < '1' || v > '7' {
						t.Fatalf("solution %d: unexpected value %c at (%d,%d,%d)", count, v, x, y, z)
					}
				}
			}
		}
		// Validate all 7 pieces present
		var present [8]bool
		for x := 0; x < 3; x++ {
			for y := 0; y < 3; y++ {
				for z := 0; z < 3; z++ {
					present[w[x][y][z]-'0'] = true
				}
			}
		}
		for p := 1; p <= 7; p++ {
			if !present[p] {
				t.Fatalf("solution %d: missing piece %d", count, p)
			}
		}
		count++
	}

	if count != 415 {
		t.Fatalf("expected 415 canonical solutions, got %d", count)
	}
}

func TestWorldPrintIncludesExpectedCharacters(t *testing.T) {
	var w World
	w.Place(CubeList{{0, 0, 0}}, '1')

	var buf bytes.Buffer
	writer := bufio.NewWriter(&buf)
	w.Print(writer)
	if err := writer.Flush(); err != nil {
		t.Fatalf("flush failed: %v", err)
	}

	output := buf.String()
	if len(output) == 0 {
		t.Fatalf("expected non-empty print output")
	}
	if !bytes.Contains([]byte(output), []byte("1")) {
		t.Fatalf("expected output to contain placed value '1'")
	}
	if !bytes.Contains([]byte(output), []byte("-")) {
		t.Fatalf("expected output to contain '-' for empty cells")
	}
}
