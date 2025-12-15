#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

#define START_LEN 64
#define PART1CONNS 1000

static long *distances;

int cmp_idx(const void *a, const void *b) {
    size_t ia = *(const size_t *)a;
    size_t ib = *(const size_t *)b;
    return (distances[ia] > distances[ib]) - (distances[ia] < distances[ib]);
}

int revcomp(const void *a, const void *b) { return (*(int *)b - *(int *)a); }

int product_3_largest(int *circuits, int len, int n_circuits) {

    int *counts = calloc(n_circuits, sizeof(int));

    for (int idx = 0; idx < len; idx++) {
        if (circuits[idx]) {
            counts[circuits[idx]]++;
        }
    }

    qsort(counts, n_circuits, sizeof(counts[0]), revcomp);

    // unsafe
    return counts[0] * counts[1] * counts[2];
}

typedef struct Answers {
    int part1;
    long part2;
} Anwers;

struct Answers part1(int *x, int *y, int *z, int len) {

    struct Answers ans;

    // calculate pairwise distances
    int npairs = len * (len / 2);
    distances = malloc(npairs * sizeof(long));
    int *first_idx = malloc(npairs * sizeof(int));
    int *second_idx = malloc(npairs * sizeof(int));

    int counter = 0;
    for (int idx1 = 0; idx1 < len; idx1++) {
        for (int idx2 = idx1 + 1; idx2 < len; idx2++) {
            first_idx[counter] = idx1;
            second_idx[counter] = idx2;
            long dx = (long)(x[idx1] - x[idx2]);
            long dy = (long)(y[idx1] - y[idx2]);
            long dz = (long)(z[idx1] - z[idx2]);

            distances[counter++] = dx * dx + dy * dy + dz * dz;
        }
    }

    // sort distances, first_idx, second_idx on distance
    size_t *idx = malloc(counter * sizeof(size_t));
    int *circuit = malloc(counter * sizeof(int));
    for (size_t i = 0; i < counter; i++) {
        idx[i] = i;
        circuit[i] = 0;
    }

    qsort(idx, counter, sizeof(size_t), cmp_idx);

    // assign to circuits
    int available_circuit_no = 1;
    int consumed_circuits = 0;
    bool all_assigned = false;
    for (int step = 0; step < counter; step++) {
        if (step == PART1CONNS) {
            ans.part1 =
                product_3_largest(circuit, counter, available_circuit_no);
        }

        int trs = idx[step];
        int idx1 = first_idx[trs];
        int idx2 = second_idx[trs];
        int distance = distances[trs];
        int circ1 = circuit[idx1]; // maybe translation problem here
        int circ2 = circuit[idx2];

        // both junctions unassigned
        if (circ1 == circ2 && circ2 == 0) {
            circuit[idx1] = available_circuit_no;
            circuit[idx2] = available_circuit_no++;
            // junction 2 assigned, 1 not
        } else if (circ1 == 0 && circ2 > 0) {
            circuit[idx1] = circ2;
            // junction 1 assigned, 2 not
        } else if (circ1 > 0 && circ2 == 0) {
            circuit[idx2] = circ1;
            // both junctions assigned but not to the same circuit
        } else if (circ1 != circ2) { // implicit that both are non-zero
            consumed_circuits++;
            // combine two circuits
            for (int i = 0; i < len; i++) {
                if (circuit[i] == circ2) {
                    circuit[i] = circ1;
                }
            }
        }

        if (!all_assigned) {
            bool tmp = true;
            for (int i = 0; i < len; i++) {
                if (circuit[i] == 0) {
                    tmp = false;
                    break;
                }
            }
            all_assigned = tmp;
        }

        if (all_assigned && consumed_circuits &&
            (available_circuit_no - consumed_circuits == 2)) {
            ans.part2 = (long)x[idx1] * (long)x[idx2];
            break;
        }
    }

    free(distances);
    free(first_idx);
    free(second_idx);
    free(idx);
    free(circuit);

    return ans;
}

int main() {
    FILE *f = fopen("day08_large.txt", "r");

    struct stat st;
    fstat(fileno(f), &st);
    size_t size = st.st_size;

    char *buf = malloc(size + 1);
    fread(buf, 1, size, f);
    buf[size] = '\0';

    int npoints = 0;
    int point_len = START_LEN;
    int *x = malloc(point_len * sizeof(int));
    int *y = malloc(point_len * sizeof(int));
    int *z = malloc(point_len * sizeof(int));

    char *ptr = buf;
    while (*ptr) {
        if (npoints == point_len) {
            point_len *= 2;
            x = realloc(x, point_len * sizeof(int));
            y = realloc(y, point_len * sizeof(int));
            z = realloc(z, point_len * sizeof(int));
        }
        if (*ptr == '\n') {
            ptr++;
        }

        x[npoints] = (int)strtol(ptr, &ptr, 10);
        ptr++;
        y[npoints] = (int)strtol(ptr, &ptr, 10);
        ptr++;
        z[npoints] = (int)strtol(ptr, &ptr, 10);
        ptr++;

        npoints++;
    }

    struct Answers answer = part1(x, y, z, npoints);
    printf("Part1 %i Part2 %li \n", answer.part1, answer.part2);

    free(x);
    free(y);
    free(z);

    return 0;
}
