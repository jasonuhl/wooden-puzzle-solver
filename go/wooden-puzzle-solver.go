package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"sort"
)

type Coord int8

type Position struct {
	x, y, z Coord
}

var AllPositions []Position

func init() {
	for x := Coord(0); x < 3; x++ {
		for y := Coord(0); y < 3; y++ {
			for z := Coord(0); z < 3; z++ {
				AllPositions = append(AllPositions, Position{x, y, z})
			}
		}
	}
}

type Cube Position

func CubeLess(c1, c2 Cube) bool {
	if c1.x != c2.x {
		return c1.x < c2.x
	} else if c1.y != c2.y {
		return c1.y < c2.y
	} else {
		return c1.z < c2.z
	}
}

type CubeRotation func(Cube) Cube

func Identity(c Cube) Cube {
	return c
}

func RotateXY(c Cube) Cube {
	return Cube{c.y, -c.x, c.z}
}

func RotateYZ(c Cube) Cube {
	return Cube{c.x, c.z, -c.y}
}

func RotateXZ(c Cube) Cube {
	return Cube{-c.z, c.y, c.x}
}

func Translation(dx, dy, dz Coord) func(Cube) Cube {
	return func(c Cube) Cube {
		return Cube{c.x + dx, c.y + dy, c.z + dz}
	}
}

func Compose(rs ...CubeRotation) CubeRotation {
	return func(c Cube) Cube {
		for i := len(rs) - 1; i >= 0; i-- {
			c = rs[i](c)
		}
		return c
	}
}

func Repeat(r CubeRotation, n int) CubeRotation {
	var rn CubeRotation = Identity
	for i := 0; i < n; i++ {
		rn = Compose(r, rn)
	}
	return rn
}

var XYRotations = []CubeRotation{
	Repeat(RotateXY, 0),
	Repeat(RotateXY, 1),
	Repeat(RotateXY, 2),
	Repeat(RotateXY, 3),
}

var ZRotations = []CubeRotation{
	Repeat(RotateYZ, 0),
	Repeat(RotateYZ, 1),
	Repeat(RotateYZ, 2),
	Repeat(RotateYZ, 3),
	Repeat(RotateXZ, 1),
	Repeat(RotateXZ, 3),
}

var AllRotations []CubeRotation

func init() {
	for _, r1 := range XYRotations {
		for _, r2 := range ZRotations {
			AllRotations = append(AllRotations, Compose(r2, r1))
		}
	}
}

type Dingus struct {
	// A map that groups the placements of this Dingus according
	// to the "lowest" cube in that placement:
	placements map[Position]CubeListSet
}

// Return a new CubeListSet containing only the distinct elements of
// the original.
func (set CubeListSet) Distinct() CubeListSet {
	var distinct CubeListSet

	if len(set) != 0 {
		sort.Sort(set)

		last := set[0]
		distinct = append(distinct, last)

		for _, cubes := range set[1:] {
			if !CubeListsEqual(last, cubes) {
				distinct = append(distinct, cubes)
				last = cubes
			}
		}
	}

	return distinct
}

func NewDingus(w *World, name string, rotate bool, cubes CubeList) Dingus {
	// Note that ordering is invarient under translation, so we
	// can sort the cubes as soon as they are rotated.

	var rotated CubeListSet
	if rotate {
		for _, r := range AllRotations {
			cubes := Apply(cubes, r)
			sort.Sort(cubes)
			c := cubes[0]
			cubes = Apply(cubes, Translation(-c.x, -c.y, -c.z))
			rotated = append(rotated, cubes)
		}
		rotated = rotated.Distinct()
		fmt.Fprintf(os.Stderr, "Dingus %s: found %d rotations\n", name, len(rotated))
	} else {
		sort.Sort(cubes)
		rotated = append(rotated, cubes)
		fmt.Fprintf(os.Stderr, "Dingus %s: ignoring rotations\n", name)
	}

	var all CubeListSet
	for _, cubes := range rotated {
		for _, placed := range w.AllTranslations(cubes) {
			all = append(all, placed)
		}
	}

	all = all.Distinct()

	placements := make(map[Position]CubeListSet)
	var count int
	if len(all) != 0 {
		last := all[0]
		placements[Position(last[0])] = append(placements[Position(last[0])], last)
		count++

		for _, cubes := range all[1:] {
			if !CubeListsEqual(last, cubes) {
				placements[Position(cubes[0])] = append(placements[Position(cubes[0])], cubes)
				count++
				last = cubes
			}
		}
	}

	fmt.Fprintf(os.Stderr, "          found %d placements\n", count)

	return Dingus{placements: placements}
}

type CubeList []Cube

func (c Cube) String() string {
	return fmt.Sprintf("(%d,%d,%d)", c.x, c.y, c.z)
}

func (cubes CubeList) String() string {
	var buf bytes.Buffer
	for i, cube := range cubes {
		if i == 0 {
			fmt.Fprintf(&buf, "%s", cube)
		} else {
			fmt.Fprintf(&buf, ",%s", cube)
		}
	}
	return buf.String()
}

func CubeListLess(cubes1, cubes2 CubeList) bool {
	for n := 0; ; n++ {
		if n == len(cubes1) {
			return n != len(cubes2)
		} else if n == len(cubes2) {
			return false
		} else if cubes1[n] != cubes2[n] {
			return CubeLess(cubes1[n], cubes2[n])
		}
	}
	return false
}

// Allow the cubes in a CubeList to be sorted:

func (cubes CubeList) Len() int {
	return len(cubes)
}

func (cubes CubeList) Less(i, j int) bool {
	return CubeLess(cubes[i], cubes[j])
}

func (cubes CubeList) Swap(i, j int) {
	cubes[i], cubes[j] = cubes[j], cubes[i]
}

// Return true if the two CubeLists' cubes are at the exact same
// locations.
func CubeListsEqual(cubes1, cubes2 CubeList) bool {
	if len(cubes1) != len(cubes2) {
		return false
	}
	for i := 0; i < len(cubes1); i++ {
		if cubes1[i] != cubes2[i] {
			return false
		}
	}
	return true
}

type CubeListSet []CubeList

// Allow the cubeLists in a CubeListSet to be sorted:

func (cubeLists CubeListSet) Len() int {
	return len(cubeLists)
}

func (cubeLists CubeListSet) Less(i, j int) bool {
	return CubeListLess(cubeLists[i], cubeLists[j])
}

func (cubeLists CubeListSet) Swap(i, j int) {
	cubeLists[i], cubeLists[j] = cubeLists[j], cubeLists[i]
}

func Apply(cubes CubeList, r CubeRotation) CubeList {
	var ret CubeList
	for _, c := range cubes {
		ret = append(ret, r(c))
	}
	return ret
}

type World [3][3][3]byte

func (w *World) AllTranslations(cubes CubeList) []CubeList {
	var ret []CubeList

	for dx := Coord(0); dx < 3; dx++ {
		for dy := Coord(0); dy < 3; dy++ {
			for dz := Coord(0); dz < 3; dz++ {
				translated := Apply(cubes, Translation(dx, dy, dz))
				if w.Valid(translated) {
					ret = append(ret, translated)
				}
			}
		}
	}
	return ret
}

func (w *World) ValidCube(c Cube) bool {
	return 0 <= c.x && c.x < 3 &&
		0 <= c.y && c.y < 3 &&
		0 <= c.z && c.z < 3
}

func (w *World) Valid(cubes CubeList) bool {
	for _, c := range cubes {
		if !w.ValidCube(c) {
			return false
		}
	}
	return true
}

// Check whether `cubes` can currently be placed in `w`. `cubes` must be valid.
func (w *World) Fits(cubes CubeList) bool {
	for _, c := range cubes {
		if w[c.x][c.y][c.z] != 0 {
			return false
		}
	}
	return true
}

// Place `cubes` in this world. `cubes` must be valid and must be known to fit
// in this world.
func (w *World) Place(cubes CubeList, value byte) {
	for _, c := range cubes {
		w[c.x][c.y][c.z] = value
	}
}

// Erase `d` from this world. `d` must be valid and should previously
// have been placed in this world.
func (w *World) Unplace(cubes CubeList) {
	for _, c := range cubes {
		w[c.x][c.y][c.z] = 0
	}
}

func (w *World) Print(f *bufio.Writer) {
	for y := 0; y < 3; y++ {
		for x := 0; x < 3; x++ {
			if x != 0 {
				f.WriteString("     ")
			}
			for z := 0; z < 3; z++ {
				c := w[x][y][z]
				if c == 0 {
					f.WriteByte('-')
				} else {
					f.WriteByte(c)
				}
				if z != 2 {
					f.WriteByte(' ')
				}
			}
		}
		f.WriteByte('\n')
	}
	f.WriteByte('\n')
}

func (w *World) Fill(dingi []*Dingus, toFill []Position, c byte, solutions chan<- World) {
	// Find the next position that needs filling:
	var p Position
	for {
		if len(toFill) == 0 {
			solutions <- *w
			return
		}

		p = toFill[0]
		toFill = toFill[1:]
		if w[p.x][p.y][p.z] == 0 {
			break
		}
	}

	// Try placing each distinct dingus at `p`:
	var last *Dingus
	for i, d := range dingi {
		if d == nil || d == last {
			continue
		}
		dingi[i] = nil
		for _, cubes := range d.placements[p] {
			if w.Fits(cubes) {
				w.Place(cubes, c)
				w.Fill(dingi, toFill, c + 1, solutions)
				w.Unplace(cubes)
			}
		}
		dingi[i] = d
		last = d
	}
}

func main() {
	var world World

	var DingusL = NewDingus(&world, "L", true, CubeList{Cube{0, 0, 0}, Cube{1, 0, 0}, Cube{2, 0, 0}, Cube{2, 1, 0}})
	var DingusT = NewDingus(&world, "T", true, CubeList{Cube{0, 0, 0}, Cube{1, 0, 0}, Cube{1, 1, 0}, Cube{2, 0, 0}})
	var DingusS = NewDingus(&world, "S", false, CubeList{Cube{0, 0, 0}, Cube{1, 0, 0}, Cube{1, 1, 0}, Cube{2, 1, 0}})
	var DingusR = NewDingus(&world, "R", true, CubeList{Cube{0, 0, 0}, Cube{0, 1, 0}, Cube{1, 1, 0}})

	var Dingi = []*Dingus{&DingusL, &DingusL, &DingusL, &DingusL, &DingusT, &DingusS, &DingusR}

	solutions := make(chan World)
	go func() {
		world.Fill(Dingi, AllPositions, '1', solutions)
		close(solutions)
	}()

	var count int
	f := bufio.NewWriter(os.Stdout)
	for w := range solutions {
		w.Print(f)
		count++
	}
	f.Flush()
	fmt.Fprintf(os.Stderr, "Found %d solutions\n", count)
}
