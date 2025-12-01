#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int rem_euclid(int a, int m) {
    int r = a % m;
    return (r < 0) ? r + m : r;
}

int main() {
    const int POSITIONS = 100;
    int current_pos = 50;
    int zeros = 0;
    int passes = 0;

    FILE *f = fopen("input.txt", "r");
    char line[32];
    while (fgets(line, sizeof(line), f)) {
        char *end;
        int value = strtol(line + 1, &end, 10);

        int partial_rotation = value % POSITIONS;
        if (*line == 'L') {
            if ((partial_rotation >= current_pos) & (current_pos > 0)) {
                passes++;
            }
            current_pos -= value;
        } else if (*line == 'R') {
            if ((partial_rotation + current_pos) >= POSITIONS) {
                passes++;
            }
            current_pos += value;
        } else {
            fprintf(stderr, "panic: %s\n", "Bad input");
            abort();
        }

        // adding full rotations
        passes += value / POSITIONS;

        current_pos = rem_euclid(current_pos, POSITIONS);
        if (current_pos == 0) {
            zeros++;
        }
    }

    printf("Part 1: %i\n", zeros);
    printf("Part 2: %i\n", passes);
    return 0;
}
