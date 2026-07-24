# tetris3d.py — трёхмерный тетрис на Python

import random
import os
import sys
import time
from collections import deque

try:
    import keyboard  # для неблокирующего ввода
    HAS_KEYBOARD = True
except ImportError:
    HAS_KEYBOARD = False
    import msvcrt if sys.platform == 'win32' else None

# Размеры поля
W, H, D = 4, 4, 4  # x, y, z (ширина, глубина, высота)

# Фигуры (относительные координаты кубиков) — 7 фигур (как тетрамино, но в 3D)
SHAPES = [
    # O (квадрат 2x2 на плоскости XY, высота 1)
    [(0,0,0), (1,0,0), (0,1,0), (1,1,0)],
    # I (палка 4x1x1)
    [(0,0,0), (1,0,0), (2,0,0), (3,0,0)],
    # T (в плоскости XY)
    [(0,0,0), (1,0,0), (2,0,0), (1,1,0)],
    # L
    [(0,0,0), (1,0,0), (2,0,0), (0,1,0)],
    # J
    [(0,0,0), (1,0,0), (2,0,0), (2,1,0)],
    # S
    [(1,0,0), (2,0,0), (0,1,0), (1,1,0)],
    # Z
    [(0,0,0), (1,0,0), (1,1,0), (2,1,0)],
]

class Tetris3D:
    def __init__(self):
        self.field = [[[0 for _ in range(D)] for _ in range(H)] for _ in range(W)]
        self.score = 0
        self.level = 1
        self.fall_interval = 1.0  # секунды
        self.current_piece = None
        self.current_pos = (0, 0, 0)
        self.game_over = False
        self.next_piece = random.choice(SHAPES)
        self.spawn_piece()

    def spawn_piece(self):
        self.current_piece = self.next_piece
        self.next_piece = random.choice(SHAPES)
        self.current_pos = (1, 1, 3)  # начальная позиция (x, y, z) — в центре сверху
        if not self.is_valid(self.current_piece, self.current_pos):
            self.game_over = True

    def is_valid(self, piece, pos):
        px, py, pz = pos
        for dx, dy, dz in piece:
            x = px + dx
            y = py + dy
            z = pz + dz
            if x < 0 or x >= W or y < 0 or y >= H or z < 0 or z >= D:
                return False
            if self.field[x][y][z] != 0:
                return False
        return True

    def place_piece(self):
        px, py, pz = self.current_pos
        for dx, dy, dz in self.current_piece:
            x = px + dx
            y = py + dy
            z = pz + dz
            self.field[x][y][z] = 1
        self.clear_layers()
        self.spawn_piece()

    def clear_layers(self):
        cleared = 0
        for z in range(D):
            # проверяем, заполнен ли слой
            layer_full = True
            for x in range(W):
                for y in range(H):
                    if self.field[x][y][z] == 0:
                        layer_full = False
                        break
                if not layer_full:
                    break
            if layer_full:
                # удаляем слой (сдвигаем всё выше вниз)
                for zz in range(z, D-1):
                    for x in range(W):
                        for y in range(H):
                            self.field[x][y][zz] = self.field[x][y][zz+1]
                # очищаем верхний слой
                for x in range(W):
                    for y in range(H):
                        self.field[x][y][D-1] = 0
                cleared += 1
                # после удаления одного слоя, проверяем этот же z снова (поскольку всё сдвинулось)
                # поэтому уменьшаем z
                z -= 1
        if cleared > 0:
            self.score += cleared * 100
            # повышение уровня
            self.level = 1 + self.score // 500
            self.fall_interval = max(0.2, 1.0 / (1 + (self.level-1) * 0.2))

    def move(self, dx, dy, dz):
        new_pos = (self.current_pos[0] + dx, self.current_pos[1] + dy, self.current_pos[2] + dz)
        if self.is_valid(self.current_piece, new_pos):
            self.current_pos = new_pos
            return True
        return False

    def rotate(self, axis='z'):
        # вращение вокруг оси Z (поворот на 90°)
        if axis == 'z':
            new_piece = [(-y, x, z) for x, y, z in self.current_piece]
        elif axis == 'x':
            new_piece = [(x, -z, y) for x, y, z in self.current_piece]
        elif axis == 'y':
            new_piece = [(z, y, -x) for x, y, z in self.current_piece]
        else:
            return
        if self.is_valid(new_piece, self.current_pos):
            self.current_piece = new_piece

    def hard_drop(self):
        while self.is_valid(self.current_piece, (self.current_pos[0], self.current_pos[1], self.current_pos[2]-1)):
            self.current_pos = (self.current_pos[0], self.current_pos[1], self.current_pos[2]-1)
        self.place_piece()

    def update(self):
        # падение вниз (по Z)
        if self.is_valid(self.current_piece, (self.current_pos[0], self.current_pos[1], self.current_pos[2]-1)):
            self.current_pos = (self.current_pos[0], self.current_pos[1], self.current_pos[2]-1)
        else:
            self.place_piece()

    def draw(self):
        os.system('cls' if os.name == 'nt' else 'clear')
        print("ТЕТРИС 3D")
        print(f"Очки: {self.score}  |  Уровень: {self.level}")
        # отображаем каждый слой (z)
        for z in range(D-1, -1, -1):
            print(f"\nСЛОЙ {z+1} (Y={z})")
            print("  0 1 2 3")
            for y in range(H-1, -1, -1):
                row = f"{y} "
                for x in range(W):
                    if self.field[x][y][z] == 1:
                        row += "X "
                    else:
                        # проверяем, не принадлежит ли текущая фигура этому месту
                        if (x, y, z) in [(self.current_pos[0]+dx, self.current_pos[1]+dy, self.current_pos[2]+dz) for dx,dy,dz in self.current_piece]:
                            row += "█ "
                        else:
                            row += ". "
                print(row)
        # инфо о следующей фигуре
        print("\nСледующая фигура:")
        for dz in range(2):
            for dy in range(2):
                row = ""
                for dx in range(2):
                    if (dx, dy, dz) in self.next_piece:
                        row += "█ "
                    else:
                        row += ". "
                print(row)
        print("\nУправление: WASD - движение, Q/E - поворот Z, R - поворот X, Space - падение, Esc - выход")

    def get_input(self):
        if HAS_KEYBOARD:
            if keyboard.is_pressed('a'): return 'left'
            if keyboard.is_pressed('d'): return 'right'
            if keyboard.is_pressed('w'): return 'up'
            if keyboard.is_pressed('s'): return 'down'
            if keyboard.is_pressed('q'): return 'rot_z'
            if keyboard.is_pressed('e'): return 'rot_z_rev'
            if keyboard.is_pressed('r'): return 'rot_x'
            if keyboard.is_pressed('space'): return 'drop'
            if keyboard.is_pressed('esc'): return 'exit'
            return None
        else:
            # упрощённый ввод с Enter
            if sys.stdin.isatty():
                # для Windows используем msvcrt
                pass
            return None

    def run(self):
        last_fall = time.time()
        while not self.game_over:
            self.draw()
            cmd = self.get_input()
            if cmd == 'left':
                self.move(-1, 0, 0)
            elif cmd == 'right':
                self.move(1, 0, 0)
            elif cmd == 'up':
                self.move(0, 1, 0)
            elif cmd == 'down':
                self.move(0, -1, 0)
            elif cmd == 'rot_z':
                self.rotate('z')
            elif cmd == 'rot_z_rev':
                self.rotate('z'); self.rotate('z'); self.rotate('z')
            elif cmd == 'rot_x':
                self.rotate('x')
            elif cmd == 'drop':
                self.hard_drop()
            elif cmd == 'exit':
                break
            # падение по таймеру
            if time.time() - last_fall > self.fall_interval:
                self.update()
                last_fall = time.time()
            time.sleep(0.05)
        print("ИГРА ОКОНЧЕНА! Ваш счёт:", self.score)

if __name__ == "__main__":
    game = Tetris3D()
    game.run()
