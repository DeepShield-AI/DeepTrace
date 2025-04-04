#define _GNU_SOURCE
#include <stdio.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <string.h>
#include "time_measure.h"
#include "config.h"

double measure_sendmsg_time(int sockfd) {
    struct msghdr msg = {0};
    struct iovec iov[4];
    char buffer[4][BLOCK_SIZE / 4];
    for (int i = 0; i < 4; i++) {
        iov[i].iov_base = buffer[i];
        iov[i].iov_len = BLOCK_SIZE / 4;
    }
    msg.msg_iov = iov;
    msg.msg_iovlen = 4;

    for (int i = 0; i < 4; i++) {
        memset(buffer[i], 'a', BLOCK_SIZE / 4);
    }
    time_measure_t tm;
    start(&tm);
    for (int i = 0; i < ITERATIONS; i++) {
        sendmsg(sockfd, &msg, 0);
    }
    end(&tm);
    return get_elapsed_ns(&tm) / ITERATIONS;
}

int main() {
    int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    struct sockaddr_in addr = {
        .sin_family = AF_INET,
        .sin_port = htons(20111),
        .sin_addr.s_addr = inet_addr("0.0.0.0")
    };
    if (bind(sockfd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        perror("bind");
        close(sockfd);
        return -1;
    }
    struct sockaddr_in addr_t = {
        .sin_family = AF_INET,
        .sin_port = htons(20112),
        .sin_addr.s_addr = inet_addr("0.0.0.0")
    };
    if (-1 == connect(sockfd, &addr_t, sizeof(addr_t)))
    {
        /* is non-blocking, so we don't get error at that point yet */
        printf("connect error\n");
    }
    double avg_time = 0;
    for (int i = 0; i < REPEAT; i++) {
        avg_time += measure_sendmsg_time(sockfd);
    }

    printf("Average sendmsg(128B) time: %.2f ns\n", avg_time / REPEAT);
    close(sockfd);
    return 0;
}