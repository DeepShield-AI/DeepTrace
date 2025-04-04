#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include "time_measure.h"
#include "utils.h"

double measure_read_time(int fd) {
    volatile char buffer[BLOCK_SIZE];

    time_measure_t tm;
    start(&tm);

    for (int i = 0; i < ITERATIONS; i++) {
        ssize_t bytes_read = read(fd, buffer, BLOCK_SIZE);
        if (bytes_read != BLOCK_SIZE) {
            if (bytes_read < 0) {
                perror("read failed");
            } else {
                fprintf(stderr, "Short read: %zd bytes\n", bytes_read);
            }
            exit(EXIT_FAILURE);
        }
    }
    end(&tm);

    return get_elapsed_ns(&tm) / ITERATIONS;
}

int main() {
    ensure_test_file();

    const char* filename = "bin/test.data";
    int fd = open(filename, O_RDONLY);
    if (fd == -1) {
        perror("open error");
        exit(EXIT_FAILURE);
    }

    double avg_time = 0;
    for (int i = 0; i < REPEAT; i++) {
        lseek(fd, 0, SEEK_SET);
        avg_time += measure_read_time(fd);
    }

    printf("Average read(128B) time: %.2f ns\n", avg_time / REPEAT);

    close(fd);
    return 0;
}