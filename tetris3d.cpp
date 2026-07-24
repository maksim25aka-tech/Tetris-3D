// tetris3d.cpp — трёхмерный тетрис на C++

#include <iostream>
#include <vector>
#include <random>
#include <thread>
#include <chrono>
#include <cstdlib>
#include <ctime>
#include <conio.h> // для _kbhit и _getch (Windows) или используем termios на Linux
#ifdef _WIN32
#include <windows.h>
#else
#include <termios.h>
#include <unistd.h>
#include <fcntl.h>
#endif

using namespace std;

const int W = 4, H = 4, D = 4;

// фигуры
vector<vector<tuple<int,int,int>>> shapes = {
    {{0,0,0},{1,0,0},{0,1,0},{1,1,0}},
    {{0,0,0},{1,0,0},{2,0,0},{3,0,0}},
    {{0,0,0},{1,0,0},{2,0,0},{1,1,0}},
    {{0,0,0},{1,0,0},{2,0,0},{0,1,0}},
    {{0,0,0},{1,0,0},{2,0,0},{2,1,0}},
    {{1,0,0},{2,0,0},{0,1,0},{1,1,0}},
    {{0,0,0},{1,0,0},{1,1,0},{2,1,0}}
};

class Tetris3D {
private:
    int field[W][H][D];
    int score, level;
    double fallInterval;
    vector<tuple<int,int,int>> currentPiece, nextPiece;
    tuple<int,int,int> currentPos;
    bool gameOver;

    bool isValid(vector<tuple<int,int,int>>& piece, tuple<int,int,int> pos) {
        auto [px, py, pz] = pos;
        for (auto [dx, dy, dz] : piece) {
            int x = px+dx, y = py+dy, z = pz+dz;
            if (x<0 || x>=W || y<0 || y>=H || z<0 || z>=D) return false;
            if (field[x][y][z] != 0) return false;
        }
        return true;
    }

    void placePiece() {
        auto [px, py, pz] = currentPos;
        for (auto [dx, dy, dz] : currentPiece) {
            field[px+dx][py+dy][pz+dz] = 1;
        }
        clearLayers();
        spawnPiece();
    }

    void clearLayers() {
        int cleared = 0;
        for (int z=0; z<D; z++) {
            bool full = true;
            for (int x=0; x<W; x++) for (int y=0; y<H; y++) if (field[x][y][z]==0) full=false;
            if (full) {
                // сдвиг вниз
                for (int zz=z; zz<D-1; zz++) {
                    for (int x=0; x<W; x++) for (int y=0; y<H; y++) field[x][y][zz] = field[x][y][zz+1];
                }
                for (int x=0; x<W; x++) for (int y=0; y<H; y++) field[x][y][D-1] = 0;
                cleared++; z--;
            }
        }
        if (cleared) {
            score += cleared*100;
            level = 1 + score/500;
            fallInterval = max(0.2, 1.0/(1+(level-1)*0.2));
        }
    }

    void spawnPiece() {
        currentPiece = nextPiece;
        nextPiece = shapes[rand()%shapes.size()];
        currentPos = {1,1,3}; // центр сверху
        if (!isValid(currentPiece, currentPos)) gameOver = true;
    }

public:
    Tetris3D() : score(0), level(1), fallInterval(1.0), gameOver(false) {
        srand(time(nullptr));
        memset(field, 0, sizeof(field));
        nextPiece = shapes[rand()%shapes.size()];
        spawnPiece();
    }

    bool move(int dx, int dy, int dz) {
        auto [px, py, pz] = currentPos;
        tuple<int,int,int> newPos = {px+dx, py+dy, pz+dz};
        if (isValid(currentPiece, newPos)) {
            currentPos = newPos;
            return true;
        }
        return false;
    }

    void rotate(char axis) {
        vector<tuple<int,int,int>> newPiece;
        if (axis == 'z') {
            for (auto [x,y,z] : currentPiece) newPiece.push_back({-y, x, z});
        } else if (axis == 'x') {
            for (auto [x,y,z] : currentPiece) newPiece.push_back({x, -z, y});
        } else if (axis == 'y') {
            for (auto [x,y,z] : currentPiece) newPiece.push_back({z, y, -x});
        }
        if (isValid(newPiece, currentPos)) currentPiece = newPiece;
    }

    void hardDrop() {
        while (isValid(currentPiece, {get<0>(currentPos), get<1>(currentPos), get<2>(currentPos)-1})) {
            currentPos = {get<0>(currentPos), get<1>(currentPos), get<2>(currentPos)-1};
        }
        placePiece();
    }

    void update() {
        if (isValid(currentPiece, {get<0>(currentPos), get<1>(currentPos), get<2>(currentPos)-1})) {
            currentPos = {get<0>(currentPos), get<1>(currentPos), get<2>(currentPos)-1};
        } else {
            placePiece();
        }
    }

    void draw() {
        system("cls");
        cout << "ТЕТРИС 3D\n";
        cout << "Очки: " << score << " | Уровень: " << level << "\n";
        for (int z=D-1; z>=0; z--) {
            cout << "\nСЛОЙ " << z+1 << " (Y=" << z << ")\n";
            cout << "  0 1 2 3\n";
            for (int y=H-1; y>=0; y--) {
                cout << y << " ";
                for (int x=0; x<W; x++) {
                    if (field[x][y][z]) cout << "X ";
                    else {
                        bool inPiece = false;
                        for (auto [dx,dy,dz] : currentPiece) {
                            if (x==get<0>(currentPos)+dx && y==get<1>(currentPos)+dy && z==get<2>(currentPos)+dz) inPiece=true;
                        }
                        cout << (inPiece ? "█ " : ". ");
                    }
                }
                cout << "\n";
            }
        }
        cout << "\nСледующая фигура:\n";
        for (int dy=0; dy<2; dy++) {
            for (int dx=0; dx<2; dx++) {
                bool found = false;
                for (auto [ex,ey,ez] : nextPiece) if (ex==dx && ey==dy && ez==0) found=true;
                cout << (found ? "█ " : ". ");
            }
            cout << "\n";
        }
        cout << "\nWASD - движение, Q/E - поворот Z, R - поворот X, Space - падение, Esc - выход\n";
    }

    void run() {
        bool running = true;
        auto lastFall = chrono::steady_clock::now();
        while (running && !gameOver) {
            draw();
            // ввод (упрощённо, с Enter)
            if (_kbhit()) {
                char ch = _getch();
                if (ch == 'a') move(-1,0,0);
                else if (ch == 'd') move(1,0,0);
                else if (ch == 'w') move(0,1,0);
                else if (ch == 's') move(0,-1,0);
                else if (ch == 'q') rotate('z');
                else if (ch == 'e') { rotate('z'); rotate('z'); rotate('z'); }
                else if (ch == 'r') rotate('x');
                else if (ch == ' ') hardDrop();
                else if (ch == 27) { running = false; break; }
            }
            auto now = chrono::steady_clock::now();
            if (chrono::duration<double>(now-lastFall).count() > fallInterval) {
                update();
                lastFall = now;
            }
            this_thread::sleep_for(chrono::milliseconds(50));
        }
        cout << "ИГРА ОКОНЧЕНА! Счёт: " << score << "\n";
    }
};

int main() {
    Tetris3D game;
    game.run();
    return 0;
}
