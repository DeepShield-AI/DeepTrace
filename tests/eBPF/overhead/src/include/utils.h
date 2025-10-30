#ifndef DEEPTRACE_UTILS_H
#define DEEPTRACE_UTILS_H
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>
#include "config.h"

static inline void ensure_test_file() {
    struct stat st;
    if (stat("bin/test.data", &st) == 0 && st.st_size == (off_t)(BLOCK_SIZE * ITERATIONS)) return;
    
    char cmd[256];
    snprintf(cmd, sizeof(cmd), "mkdir -p bin && dd if=/dev/urandom of=bin/test.data bs=%d count=%d status=none", 
             BLOCK_SIZE, ITERATIONS);
    if (system(cmd) != 0) {
        perror("Failed to create test file");
        exit(EXIT_FAILURE);
    }
}
#endif // DEEPTRACE_UTILS_H