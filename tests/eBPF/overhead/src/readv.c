#include <sys/uio.h>
#include <errno.h>
#include <stdio.h>
#include "time_measure.h"
#include "utils.h"

#define IOV_CNT (BLOCK_SIZE / 32)

double measure_readv_time(int fd) {
    time_measure_t tm;
    
    volatile char buf[IOV_CNT][32];
    struct iovec iov[IOV_CNT];
    
    for (int i = 0; i < IOV_CNT; i++) {
        iov[i].iov_base = (void *)buf[i];
        iov[i].iov_len = sizeof(buf[i]);
    }

    start(&tm);
    for (int i = 0; i < ITERATIONS; i++) {
        ssize_t bytes_read = readv(fd, iov, IOV_CNT);
        if (bytes_read != BLOCK_SIZE) {
            if (bytes_read < 0 && errno == EAGAIN) continue;
            perror("readv failed");
            exit(EXIT_FAILURE);
        }
    }
    end(&tm);

    return get_elapsed_ns(&tm) / ITERATIONS;
}

int main() {
    ensure_test_file();
    
    int fd = open("bin/test.data", O_RDONLY);
    if (fd == -1) {
        perror("open error");
        exit(EXIT_FAILURE);
    }

    double avg_time = 0;
    for (int i = 0; i < REPEAT; i++) {
        lseek(fd, 0, SEEK_SET);
        avg_time += measure_readv_time(fd);
    }

    printf("Average readv(%dB) time: %.2f ns\n", BLOCK_SIZE, avg_time / REPEAT);
    close(fd);
    return 0;
}