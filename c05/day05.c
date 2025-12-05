#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

#define START_SIZE 8
// https://stackoverflow.com/questions/3437404/min-and-max-in-c
#define max(a, b)                                                              \
    ({                                                                         \
        __typeof__(a) _a = (a);                                                \
        __typeof__(b) _b = (b);                                                \
        _a > _b ? _a : _b;                                                     \
    })

#define min(a, b)                                                              \
    ({                                                                         \
        __typeof__(a) _a = (a);                                                \
        __typeof__(b) _b = (b);                                                \
        _a < _b ? _a : _b;                                                     \
    })

int main() {

    FILE *f = fopen("day05_larger.txt", "r");

    struct stat st;
    fstat(fileno(f), &st);
    size_t size = st.st_size;

    char *buf = malloc(size + 1);
    fread(buf, 1, size, f);
    buf[size] = '\0';

    int interval_size = START_SIZE;
    long *starts = malloc(interval_size * sizeof(long));
    long *ends = malloc(interval_size * sizeof(long));
    int intervals = 0;

    int ingred_size = START_SIZE;
    long *ingreds = malloc(ingred_size * sizeof(long));
    int ingred_count = 0;

    bool interval_phase = true;
    for (char *p = buf; *p; p++) {
        if (*p == '\n') {
            if (*(p - 1) == '\n') {
                interval_phase = false;
            }
            continue;
        }

        if (interval_phase) {
            long start = 0;
            while (*p >= '0' && *p <= '9') {
                start = start * 10 + (*p++ - '0');
            }
            p++;

            long end = 0;
            while (*p >= '0' && *p <= '9') {
                end = end * 10 + (*p++ - '0');
            }

            if (intervals == interval_size) {
                interval_size *= 2;
                starts = realloc(starts, interval_size * sizeof(long));
                ends = realloc(ends, interval_size * sizeof(long));
            }

            starts[intervals] = start;
            ends[intervals++] = end;
        } else {
            long ingredient = 0;
            while (*p >= '0' && *p <= '9') {
                ingredient = ingredient * 10 + (*p++ - '0');
            }

            if (ingred_count == ingred_size) {
                ingred_size *= 2;
                ingreds = realloc(ingreds, ingred_size * sizeof(long));
            }

            ingreds[ingred_count++] = ingredient;
        }
    }

    // Part 2: simplifying intervals
    bool *mask = calloc(intervals, sizeof(bool));
    while (true) {
        int changes = 0;
        for (int idx1 = 0; idx1 < intervals - 1; idx1++) {
            if (mask[idx1]) {
                continue;
            }
            for (int idx2 = idx1 + 1; idx2 < intervals; idx2++) {
                if (mask[idx2]) {
                    continue;
                }

                // disjoint intervals
                if ((starts[idx1] > ends[idx2]) ||
                    (starts[idx2] > ends[idx1])) {
                    continue;
                }

                changes++;
                // first interval contains second interval
                if ((starts[idx1] <= starts[idx2]) &&
                    (ends[idx1] > ends[idx2])) {
                    mask[idx2] = true;
                    continue;
                }

                // second interval contains first interval
                if ((starts[idx1] <= starts[idx2]) &&
                    (ends[idx1] > ends[idx2])) {
                    mask[idx1] = true;
                    continue;
                }

                // intervals overlap
                starts[idx1] = min(starts[idx1], starts[idx2]);
                ends[idx1] = max(ends[idx1], ends[idx2]);
                mask[idx2] = true;
            }
        }

        if (changes == 0) {
            break;
        }
    }

    // Part1: checking for fresh ingredients
    int fresh = 0;
    for (int ingred_idx = 0; ingred_idx < ingred_count; ingred_idx++) {
        for (int interv_idx = 0; interv_idx < intervals; interv_idx++) {
            if (mask[interv_idx]) {
                continue;
            }
            if ((ingreds[ingred_idx] <= ends[interv_idx]) &&
                (ingreds[ingred_idx] >= starts[interv_idx])) {
                fresh++;
                break;
            }
        }
    }

    printf("Part 1: %i\n", fresh);

    long total_fresh = 0;
    for (int interv_idx = 0; interv_idx < intervals; interv_idx++) {
        if (!mask[interv_idx]) {
            total_fresh += ends[interv_idx] - starts[interv_idx] + 1;
        }
    }

    printf("Part 2: %li\n", total_fresh);

    free(starts);
    free(ends);
    free(ingreds);
    return 0;
}
