#define _GNU_SOURCE
#include <sys/uio.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>
#include "time_measure.h"
#include "config.h"

#define IOV_CNT (BLOCK_SIZE / 32)

double measure_writev_time(int fd) {
    time_measure_t tm;
    static char buf[4][32];
    static struct iovec iov[4];
    static int initialized = 0;

    if (!initialized) {
        for (int i = 0; i < IOV_CNT; i++) {
            memset(buf[i], 'a', sizeof(buf[i]));
            iov[i].iov_base = buf[i];
            iov[i].iov_len = sizeof(buf[i]);
        }
        initialized = 1;
    }
    const int iovcnt = IOV_CNT;

    start(&tm);
    for (int i = 0; i < ITERATIONS; i++) {
        ssize_t nw = writev(fd, iov, iovcnt);
        if (nw == -1) {
            perror("writev failed");
            exit(EXIT_FAILURE);
        }
        if (nw != (ssize_t)(iov[0].iov_len * 4)) {
            fprintf(stderr, "Incomplete write: %zd\n", nw);
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
        avg_time += measure_writev_time(fd);
    }

    printf("Average writev(128B) time: %.2f ns\n", avg_time / REPEAT);

    close(fd);
    remove(filename);
    return 0;
}