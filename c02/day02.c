#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>

// https://stackoverflow.com/questions/101439/the-most-efficient-way-to-implement-an-integer-based-power-function-powint-int
long ipow(long base, long exp) {
    long result = 1;
    for (;;) {
        if (exp & 1)
            result *= base;
        exp >>= 1;
        if (!exp)
            break;
        base *= base;
    }

    return result;
}

bool is_valid(long code, long length, bool part1) {
    long start;
    long end;
    if (part1) {
        if (length % 2 == 0) {
            start = length / 2;
            end = start;
        } else {
            return true;
        }
    } else {
        start = 1;
        if (length % 2 == 0) {
            end = length / 2 - 1;
        } else {
            end = length / 2;
        }
    }

    for (long split_length = start; split_length <= end; split_length++) {
        long skip_outer = 0;
        long run_code = code;
        if (length % split_length == 0) {
            long divisor = ipow(10, (length - split_length));
            long expected = run_code / divisor;
            run_code %= divisor;
            divisor /= ipow(10, split_length);

            while (divisor > 0) {
                if (run_code / divisor != expected) {
                    skip_outer = 1;
                    break;
                }
                run_code %= divisor;
                divisor /= ipow(10, split_length);
            }

            if (skip_outer)
                continue;

            return false;
        }
    }

    return true;
}

int main() {
    FILE *f = fopen("day02_large.txt", "r");

    struct stat st;
    fstat(fileno(f), &st);
    size_t size = st.st_size;

    char *buf = malloc(size + 1);
    fread(buf, 1, size, f);
    buf[size] = '\0';
    fclose(f);

    long part1 = 0;
    long part2 = 0;

    char *ptr = buf;
    while (*ptr) {

        if (*ptr == '\n') {
            break;
        }

        long start = 0;
        while (*ptr >= '0' && *ptr <= '9') {
            start = start * 10 + (*ptr++ - '0');
        }
        if (*ptr != '-') {
            printf("Expected '-' character, got '%c'\n", *ptr);
            break;
        }
        ptr++;

        long end = 0;
        while (*ptr >= '0' && *ptr <= '9') {
            end = end * 10 + (*ptr++ - '0');
        }
        ptr++;

        for (long code = start; code <= end; code++) {
            long digits = floor(log10(labs(code))) + 1;

            if (!is_valid(code, digits, true)) {
                part1 += code;
                part2 += code;
            } else if (!is_valid(code, digits, false)) {
                part2 += code;
            }
        }
    }

    printf("Part 1: %li\n", part1);
    printf("Part 2: %li\n", part2);

    return 0;
}
