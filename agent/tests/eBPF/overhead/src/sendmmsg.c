#define _GNU_SOURCE
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/socket.h>
#include <errno.h>
#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include "time_measure.h"
#include "config.h"

double measure_sendmmsg_time(int sockfd) {
    struct mmsghdr message;
    struct iovec iovec[4] = {0};
    char buf[4][BLOCK_SIZE / 4] = {0};

    for (int i = 0; i < 4; i++)
    {
        iovec[i].iov_base = buf[i];
        iovec[i].iov_len = BLOCK_SIZE / 4;
    }
    message.msg_hdr.msg_iov = iovec;
    message.msg_hdr.msg_iovlen = 4;
    message.msg_hdr.msg_name = NULL;
    message.msg_hdr.msg_namelen = 0;
    message.msg_hdr.msg_control = NULL;
    message.msg_hdr.msg_controllen = 0;
    message.msg_hdr.msg_flags = 0;
    time_measure_t tm;
    start(&tm);
    for (int i = 0; i < ITERATIONS; i++) {
        sendmmsg(sockfd, &message, 4, 0);
    }
    end(&tm);
    return get_elapsed_ns(&tm) / ITERATIONS;
}

int main(int argc, const char *argv[])
{
    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_port = htons(21111);
    addr.sin_addr.s_addr = inet_addr("0.0.0.0");

    int sd = socket(PF_INET, SOCK_DGRAM, IPPROTO_UDP);
    if (sd < 0)
    {
        printf("socket() failed\n");
    }

    int one = 1;
    int r = setsockopt(sd, SOL_SOCKET, SO_REUSEADDR, (char *)&one,
                       sizeof(one));
    if (r < 0)
    {
        printf("setsockopt(SO_REUSEADDR) failed\n");
    }
    int flags = fcntl(sd, F_GETFL, 0);
    fcntl(sd, F_SETFL, flags | O_NONBLOCK);
    if (bind(sd, &addr, sizeof(addr)) < 0)
    {
        printf("bind() failed\n");
    }
    struct sockaddr_in addr_t;
    addr_t.sin_family = AF_INET;
    addr_t.sin_port = htons(21112);
    addr_t.sin_addr.s_addr = inet_addr("0.0.0.0");
    if (-1 == connect(sd, &addr_t, sizeof(addr_t)))
    {
        /* is non-blocking, so we don't get error at that point yet */
        if (EINPROGRESS != errno)
        {
            printf("connect() failed\n");
            return -1;
        }
    }
    double avg_time = 0;
    for (int i = 0; i < REPEAT; i++)
    {
        avg_time += measure_sendmmsg_time(sd);
    }
    
    printf("Average sendmmsg(128B) time: %.2f ns\n", avg_time / REPEAT);
    close(sd);
    return 0;
}