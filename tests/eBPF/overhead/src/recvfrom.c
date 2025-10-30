#define _GNU_SOURCE
#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <fcntl.h>
#include "time_measure.h"
#include "config.h"
#define ERROR -1

double measure_recvfrom_time(int sockfd)
{
    char buf[BLOCK_SIZE + 1];
    struct sockaddr_in client;
    int client_len = sizeof(client);
    time_measure_t tm;
    start(&tm);
    for (int i = 0; i < ITERATIONS; i++)
    {
        recvfrom(sockfd, buf, sizeof(buf) - 1, 0, (struct sockaddr *)&client, &client_len);
    }
    end(&tm);
    return get_elapsed_ns(&tm) / ITERATIONS;
}

int main()
{
    struct sockaddr_in server;
    int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (sockfd == -1)
    {
        fprintf(stderr, "create socket error[%d][%s]\n", errno, strerror(errno));
        return ERROR;
    }
    memset(&server, 0, sizeof(server));
    server.sin_family = AF_INET;
    server.sin_port = htons(20011);
    server.sin_addr.s_addr = htonl(INADDR_ANY);
    if (bind(sockfd, (struct sockaddr *)&server, sizeof(struct sockaddr)) == -1)
    {
        fprintf(stderr, "bind socket error[%d][%s]", errno, strerror(errno));
        return ERROR;
    }
    double avg_time = 0;
    for (int i = 0; i < REPEAT; i++) {
        avg_time += measure_recvfrom_time(sockfd);
    }

    printf("Average recvfrom(128B) time: %.2f ns\n", avg_time / REPEAT);
    close(sockfd);
    return 0;
}
