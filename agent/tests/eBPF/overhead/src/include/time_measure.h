#ifndef TIME_MEASURE_H
#define TIME_MEASURE_H

#include <time.h>

typedef struct {
    struct timespec start;
    struct timespec end;
} time_measure_t;

static inline void start(time_measure_t *tm) {
    clock_gettime(CLOCK_MONOTONIC, &tm->start);
}

static inline void end(time_measure_t *tm) {
    clock_gettime(CLOCK_MONOTONIC, &tm->end);
}

static inline double get_elapsed_ns(time_measure_t *tm) {
    return (tm->end.tv_sec - tm->start.tv_sec) * 1e9 + 
           (tm->end.tv_nsec - tm->start.tv_nsec);
}

#endif