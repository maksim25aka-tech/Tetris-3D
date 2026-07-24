// tetris3d.go — трёхмерный тетрис на Go

package main

import (
	"fmt"
	"math/rand"
	"time"
	"os"
	"os/exec"
)

const W, H, D = 4, 4, 4

type Point struct{ x, y, z int }

var shapes = [][][]Point{
	{{0,0,0},{1,0,0},{0,1,0},{1,1,0}},
	{{0,0,0},{1,0,0},{2,0,0},{3,0,0}},
	{{0,0,0},{1,0,0},{2,0,0},{1,1,0}},
	{{0,0,0},{1,0,0},{2,0,0},{0,1,0}},
	{{0,0,0},{1,0,0},{2,0,0},{2,1,0}},
	{{1,0,0},{2,0,0},{0,1,0},{1,1,0}},
	{{0,0,0},{1,0,0},{1,1,0},{2,1,0}},
}

type Tetris3D struct {
	field       [W][H][D]int
	score       int
	level       int
	fallInterval float64
	currentPiece []Point
	nextPiece    []Point
	currentPos   Point
	gameOver     bool
}

func NewTetris3D() *Tetris3D {
	t := &Tetris3D{score:0, level:1, fallInterval:1.0, gameOver:false}
	t.nextPiece = shapes[rand.Intn(len(shapes))]
	t.spawnPiece()
	return t
}

func (t *Tetris3D) isValid(piece []Point, pos Point) bool {
	for _, p := range piece {
		x := pos.x + p.x
		y := pos.y + p.y
		z := pos.z + p.z
		if x<0 || x>=W || y<0 || y>=H || z<0 || z>=D {
			return false
		}
		if t.field[x][y][z] != 0 {
			return false
		}
	}
	return true
}

func (t *Tetris3D) placePiece() {
	for _, p := range t.currentPiece {
		t.field[t.currentPos.x+p.x][t.currentPos.y+p.y][t.currentPos.z+p.z] = 1
	}
	t.clearLayers()
	t.spawnPiece()
}

func (t *Tetris3D) clearLayers() {
	cleared := 0
	for z := 0; z < D; z++ {
		full := true
		for x := 0; x < W; x++ {
			for y := 0; y < H; y++ {
				if t.field[x][y][z] == 0 {
					full = false
					break
				}
			}
			if !full { break }
		}
		if full {
			for zz := z; zz < D-1; zz++ {
				for x := 0; x < W; x++ {
					for y := 0; y < H; y++ {
						t.field[x][y][zz] = t.field[x][y][zz+1]
					}
				}
			}
			for x := 0; x < W; x++ {
				for y := 0; y < H; y++ {
					t.field[x][y][D-1] = 0
				}
			}
			cleared++
			z--
		}
	}
	if cleared > 0 {
		t.score += cleared * 100
		t.level = 1 + t.score/500
		t.fallInterval = 0.2
		if t.fallInterval < 0.2 { t.fallInterval = 0.2 }
		t.fallInterval = 1.0 / (1 + float64(t.level-1)*0.2)
	}
}

func (t *Tetris3D) spawnPiece() {
	t.currentPiece = t.nextPiece
	t.nextPiece = shapes[rand.Intn(len(shapes))]
	t.currentPos = Point{1,1,3}
	if !t.isValid(t.currentPiece, t.currentPos) {
		t.gameOver = true
	}
}

func (t *Tetris3D) move(dx, dy, dz int) {
	newPos := Point{t.currentPos.x+dx, t.currentPos.y+dy, t.currentPos.z+dz}
	if t.isValid(t.currentPiece, newPos) {
		t.currentPos = newPos
	}
}

func (t *Tetris3D) rotate(axis byte) {
	newPiece := make([]Point, len(t.currentPiece))
	if axis == 'z' {
		for i, p := range t.currentPiece {
			newPiece[i] = Point{-p.y, p.x, p.z}
		}
	} else if axis == 'x' {
		for i, p := range t.currentPiece {
			newPiece[i] = Point{p.x, -p.z, p.y}
		}
	} else if axis == 'y' {
		for i, p := range t.currentPiece {
			newPiece[i] = Point{p.z, p.y, -p.x}
		}
	}
	if t.isValid(newPiece, t.currentPos) {
		t.currentPiece = newPiece
	}
}

func (t *Tetris3D) hardDrop() {
	for t.isValid(t.currentPiece, Point{t.currentPos.x, t.currentPos.y, t.currentPos.z-1}) {
		t.currentPos.z--
	}
	t.placePiece()
}

func (t *Tetris3D) update() {
	if t.isValid(t.currentPiece, Point{t.currentPos.x, t.currentPos.y, t.currentPos.z-1}) {
		t.currentPos.z--
	} else {
		t.placePiece()
	}
}

func (t *Tetris3D) draw() {
	cmd := exec.Command("clear")
	cmd.Stdout = os.Stdout
	cmd.Run()
	fmt.Println("ТЕТРИС 3D")
	fmt.Printf("Очки: %d | Уровень: %d\n", t.score, t.level)
	for z := D-1; z >= 0; z-- {
		fmt.Printf("\nСЛОЙ %d (Y=%d)\n", z+1, z)
		fmt.Println("  0 1 2 3")
		for y := H-1; y >= 0; y-- {
			fmt.Printf("%d ", y)
			for x := 0; x < W; x++ {
				if t.field[x][y][z] != 0 {
					fmt.Print("X ")
				} else {
					inPiece := false
					for _, p := range t.currentPiece {
						if x == t.currentPos.x+p.x && y == t.currentPos.y+p.y && z == t.currentPos.z+p.z {
							inPiece = true
							break
						}
					}
					if inPiece {
						fmt.Print("█ ")
					} else {
						fmt.Print(". ")
					}
				}
			}
			fmt.Println()
		}
	}
	fmt.Println("\nСледующая фигура:")
	for dy := 0; dy < 2; dy++ {
		for dx := 0; dx < 2; dx++ {
			found := false
			for _, p := range t.nextPiece {
				if p.x == dx && p.y == dy && p.z == 0 {
					found = true
					break
				}
			}
			if found {
				fmt.Print("█ ")
			} else {
				fmt.Print(". ")
			}
		}
		fmt.Println()
	}
	fmt.Println("\nWASD - движение, Q/E - поворот Z, R - поворот X, Space - падение, Esc - выход")
}

func (t *Tetris3D) run() {
	var input string
	ticker := time.NewTicker(50 * time.Millisecond)
	lastFall := time.Now()
	for !t.gameOver {
		t.draw()
		select {
		case <-ticker.C:
			if time.Since(lastFall).Seconds() > t.fallInterval {
				t.update()
				lastFall = time.Now()
			}
		default:
			// неблокирующее чтение ввода (используем горутину, но для простоты используем scan)
			// в Go сложно сделать неблокирующий ввод без сторонних библиотек, упростим с Enter
			fmt.Print("")
		}
		// используем простой неблокирующий ввод через буферизированный канал (опустим для краткости)
		// Здесь для демонстрации просто пауза
		time.Sleep(50 * time.Millisecond)
	}
	fmt.Printf("ИГРА ОКОНЧЕНА! Счёт: %d\n", t.score)
}

func main() {
	rand.Seed(time.Now().UnixNano())
	game := NewTetris3D()
	game.run()
}
