#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>

#define SIZE_START 8

int main() {

    FILE *f = fopen("day06_larger.txt", "r");

    struct stat st;
    fstat(fileno(f), &st);
    size_t size = st.st_size;

    char *buf = malloc(size + 1);
    fread(buf, 1, size, f);
    buf[size] = '\0';

    int n_nums = 0;
    int nums_capacity = SIZE_START;
    long *numbers = malloc(nums_capacity * sizeof(long));

    // Part 1
    int width = 0;
    int width_chars = 0;
    char *p = buf;
    for (; *p; p++) {
        if (*p == '*' || *p == '+') {
            break;
        }
        if (*p <= '9' && *p >= '0') {
            if (n_nums == nums_capacity) {
                nums_capacity *= 2;
                numbers = realloc(numbers, nums_capacity * sizeof(long));
            }
            char *endptr = p;
            numbers[n_nums++] = strtol(p, &endptr, 10);
            p = endptr;
        }

        if (*p == '\n' && width == 0) {
            width = n_nums;
            width_chars = p - buf + 1;
        }
    }

    char *oprow_start = p;
    int height = n_nums / width;

    int opcount = 0;
    long total = 0;
    for (; *p; p++) {
        if (*p == '+') {
            long answer = 0;
            for (int row = 0; row < height; row++) {
                answer += numbers[opcount + row * width];
            }
            total += answer;
            opcount++;
        } else if (*p == '*') {
            long answer = 1;
            for (int row = 0; row < height; row++) {
                answer *= numbers[opcount + row * width];
            }
            total += answer;
            opcount++;
        }
    }
    free(numbers);

    printf("Part 1: %li\n", total);
    // Part 2

    opcount = 0;
    p = oprow_start;
    char operation = '+';
    long answer = 0;
    total = 0;

    for (; *p; p++) {
        if (*p == '\n') {
            continue;
        }

        if (*p != ' ') {
            total += answer;

            if (*p == '+') {
                answer = 0;
            } else {
                answer = 1;
            }
            operation = *p;
        }

        long number = 0;
        for (int row = 0; row < height; row++) {
            char digit = buf[p - oprow_start + row * width_chars];

            if (digit != ' ') {
                number *= 10;
                number += digit - '0';
            }
        }

        if (operation == '+' && number > 0) {
            answer += number;
        } else if (operation == '*' && number > 0) {
            answer *= number;
        }
    }

    total += answer;

    printf("Part 2: %li\n", total);
    free(buf);
}
