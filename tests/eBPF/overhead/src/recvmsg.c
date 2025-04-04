#include <stdio.h>
#include <stdlib.h>
#include <sys/socket.h>
#include <time.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <string.h>
#include "time_measure.h"
#include "config.h"


double measure_recvmsg_time(int sockfd) {
    struct msghdr msg = {0};
    struct iovec iov[4];
    char buffer[4][BLOCK_SIZE / 4];
    for (int i = 0; i < 4; i++) {
        iov[i].iov_base = buffer[i];
        iov[i].iov_len = BLOCK_SIZE / 4;
    }
    msg.msg_iov = iov;
    msg.msg_iovlen = 4;

    time_measure_t tm;
    start(&tm);
    for (int i = 0; i < ITERATIONS; i++) {
        recvmsg(sockfd, &msg, 0);
    }
    end(&tm);
    return get_elapsed_ns(&tm) / ITERATIONS;
}

int main() {
    int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    struct sockaddr_in addr = {
        .sin_family = AF_INET,
        .sin_port = htons(20112),
        .sin_addr.s_addr = inet_addr("0.0.0.0")
    };
    if (bind(sockfd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        perror("bind");
        close(sockfd);
        return -1;
    }
    
    double avg_time = 0;
    for (int i = 0; i < REPEAT; i++) {
        avg_time += measure_recvmsg_time(sockfd);
    }

    printf("Average recvmsg(128B) time: %.2f ns\n", avg_time / REPEAT);
    close(sockfd);
    return 0;
}