#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

int nbors(bool *rolls, bool *ptr, int line_len, int lines) {
    int line = (ptr - rolls) / line_len;
    int col = (ptr - rolls) % line_len;

    int nbors = 0;
    bool space_left = col > 0;
    bool space_up = line > 0;
    bool space_right = col < (line_len - 1);
    bool space_down = line < (lines - 1);

    if (space_left) {
        if (*(ptr - 1)) {
            nbors++;
        }
        if (space_up && *(ptr - 1 - line_len)) {
            nbors++;
        }
        if (space_down && *(ptr - 1 + line_len)) {
            nbors++;
        }
    }
    if (space_right) {
        if (*(ptr + 1)) {
            nbors++;
        }
        if (space_up && *(ptr + 1 - line_len)) {
            nbors++;
        }
        if (space_down && *(ptr + 1 + line_len)) {
            nbors++;
        }
    }
    if (space_up && *(ptr - line_len)) {
        nbors++;
    }
    if (space_down && *(ptr + line_len)) {
        nbors++;
    }

    return nbors;
}

int main() {
    FILE *f = fopen("day04_larger.txt", "r");

    struct stat st;
    fstat(fileno(f), &st);
    size_t size = st.st_size;

    char *buf = malloc(size + 1);
    fread(buf, 1, size, f);
    buf[size] = '\0';
    fclose(f);

    bool *rolls = malloc(size);
    bool *roll_ptr = rolls;
    int line_len = 0;
    int lines = 1;

    char *ptr = buf;
    while (*ptr) {
        if (*ptr == '@') {
            *roll_ptr = true;
            roll_ptr++;
        } else if (*ptr == '\n') {
            if (line_len == 0) {
                line_len = ptr - buf;
            }
            lines++;
        } else {
            *roll_ptr = false;
            roll_ptr++;
        }
        ptr++;
    }

    int part1 = 0;
    for (roll_ptr = rolls; (roll_ptr - rolls) < (lines * line_len);
         roll_ptr++) {
        if (*roll_ptr) {
            int neighbours = nbors(rolls, roll_ptr, line_len, lines);
            if (neighbours < 4) {
                part1++;
            }
        }
    }
    printf("Part 1: %i\n", part1);

    int removed = 0;
    while (true) {
        int new_removed = 0;
        for (roll_ptr = rolls; (roll_ptr - rolls) < (lines * line_len);
             roll_ptr++) {
            if (*roll_ptr) {
                int neighbours = nbors(rolls, roll_ptr, line_len, lines);
                if (neighbours < 4) {
                    *roll_ptr = false;
                    new_removed++;
                }
            }
        }
        if (new_removed == 0) {
            break;
        }
        removed += new_removed;
    }

    printf("removed: %i\n", removed);
    free(rolls);
    return 0;
}
