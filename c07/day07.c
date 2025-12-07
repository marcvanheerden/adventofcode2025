#include <gmp.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

typedef struct Point {
    int row;
    int col;
    mpz_t count;
} Point;

typedef struct Node {
    Point data;
    struct Node *next;
} Node;

typedef struct Queue {
    Node *front;
    Node *back;
} Queue;

void initQueue(Queue *q) {
    q->front = NULL;
    q->back = NULL;
}

void enqueue(Queue *q, Point p) {
    Node *newNode = malloc(sizeof(Node));
    newNode->data.row = p.row;
    newNode->data.col = p.col;
    mpz_init_set(newNode->data.count, p.count);
    newNode->next = NULL;

    if (q->back == NULL) {
        q->front = newNode;
        q->back = newNode;
        return;
    }

    q->back->next = newNode;
    q->back = newNode;
}

bool dequeue(Queue *q, Point *p) {
    if (q->front == NULL) {
        return false;
    }

    Node *tmp = q->front;
    *p = tmp->data;  // Transfer ownership of mpz_t to caller
    q->front = q->front->next;

    if (q->front == NULL) {
        q->back = NULL;
    }

    free(tmp);
    return true;
}

bool isEmpty(Queue *q) { return q->front == NULL; }

bool entry(Queue *q, Point p) {
    if (q->front == NULL) {
        return false;
    }

    Node *tmp = q->front;

    while (tmp != NULL) {
        if (tmp->data.row == p.row && tmp->data.col == p.col) {
            mpz_add(tmp->data.count, tmp->data.count, p.count);
            return true;
        } else {
            tmp = tmp->next;
        }
    }

    return false;
}

int main() {

    FILE *f = fopen("day07_large.txt", "r");

    struct stat st;
    fstat(fileno(f), &st);
    size_t size = st.st_size;

    char *buf = malloc(size + 1);
    fread(buf, 1, size, f);
    buf[size] = '\0';

    // Part 1
    int width = strchr(buf, '\n') - buf + 1;
    int start = strchr(buf, 'S') - buf;
    int height = size / width;
    char *p = buf;

    printf("width %i height %i start %i\n", width, height, start);

    Point start_pt;
    start_pt.row = 0;
    start_pt.col = start;
    mpz_init_set_ui(start_pt.count, 1);

    Queue q;
    initQueue(&q);
    enqueue(&q, start_pt);
    int splits = 0;
    mpz_t timelines;
    mpz_init_set_ui(timelines, 0);

    while (!isEmpty(&q)) {
        Point current;
        if (!dequeue(&q, &current)) {
            printf("Can't dequeue");
        }

        current.row += 1;
        if (current.row == height) {
            mpz_add(timelines, timelines, current.count);
            continue;
        }

        bool isSplit = buf[current.row * width + current.col] == '^';

        if (isSplit) {
            if (current.col > 0) {
                current.col -= 1;
                if (!entry(&q, current)) {
                    enqueue(&q, current);
                }
                current.col += 1;
            }
            if (current.col < width - 1) {
                current.col += 1;
                if (!entry(&q, current)) {
                    enqueue(&q, current);
                }
            }
            splits++;
        } else {
            if (!entry(&q, current)) {
                enqueue(&q, current);
            }
        }
    }

    printf("Part 1: %i\n", splits);
    free(buf);
    gmp_printf("Part 2: %Zd\n", timelines);
    mpz_clear(timelines);
}
