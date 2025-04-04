#define _GNU_SOURCE
#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>
#include <sys/socket.h>
#include <sys/ioctl.h>
#include <arpa/inet.h>
#include <fcntl.h>
#include "time_measure.h"
#include "config.h"

double measure_sendto_time(int sockfd)
{
    static int initialized = 0;
    char buf[BLOCK_SIZE + 1];
    static struct sockaddr_in server;
    if (!initialized) {
        server.sin_family = AF_INET;
        server.sin_port = htons(20011);
        server.sin_addr.s_addr = inet_addr("0.0.0.0");
        memset(buf, 0, sizeof(buf));
        for (int i = 0; i < BLOCK_SIZE; i++) {
            buf[i] = rand() % 256;
        }
        initialized = 1;
    }
    time_measure_t tm;
    start(&tm);
    for (int i = 0; i < ITERATIONS; i++)
    {
        size_t sent = sendto(sockfd, buf, BLOCK_SIZE, 0, (struct sockaddr *)&server, sizeof(struct sockaddr_in));
        if (sent != BLOCK_SIZE) {
            fprintf(stderr, "Incomplete write: %zd\n", sent);
            exit(EXIT_FAILURE);
        }
    }
    end(&tm);
    return get_elapsed_ns(&tm) / ITERATIONS;
}

int main()
{
    int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (sockfd == -1)
    {
        return -1;
    }
    double avg_time = 0;
    for (int i = 0; i < REPEAT; i++)
    {
        avg_time += measure_sendto_time(sockfd);
    }
    printf("Average sendto(128B) time: %.2f ns\n", avg_time / REPEAT);
    close(sockfd);
    return 0;
}
