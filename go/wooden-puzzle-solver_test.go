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
