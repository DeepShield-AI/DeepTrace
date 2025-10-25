#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>
#include "time_measure.h"
#include "config.h"

double measure_write_time(int fd) {
    static char buffer[BLOCK_SIZE];
    static int initialized = 0;
    
    if (!initialized) {
        memset(buffer, 'a', BLOCK_SIZE);
        initialized = 1;
    }

    time_measure_t tm;
    start(&tm);
    for (int i = 0; i < ITERATIONS; i++) {
        ssize_t bytes_written = write(fd, buffer, BLOCK_SIZE);
        if (bytes_written == -1) {
            perror("write failed");
            exit(EXIT_FAILURE);
        }
        if (bytes_written != BLOCK_SIZE) {
            fprintf(stderr, "Incomplete write: %zd\n", bytes_written);
            exit(EXIT_FAILURE);
        }
    }
    end(&tm);

    return get_elapsed_ns(&tm) / ITERATIONS;
}

int main() {
    const char* filename = "bin/test.data";
    int fd = open(filename, O_WRONLY | O_CREAT | O_TRUNC, 0644);
    if (fd == -1) {
        perror("open error");
        exit(EXIT_FAILURE);
    }

    double avg_time = 0;
    for (int i = 0; i < REPEAT; i++) {
        if (ftruncate(fd, 0) == -1) {
            perror("ftruncate failed");
            exit(EXIT_FAILURE);
        }
        avg_time += measure_write_time(fd);
    }

    printf("Average write(128B) time: %.2f ns\n", avg_time / REPEAT);

    close(fd);
    remove(filename);
    return 0;
}