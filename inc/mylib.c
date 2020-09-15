#include "mylib.h"
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

void write_log(const char *msg) {
    int message_length = strlen(msg);
    write(1, msg, message_length);
}

struct Accumulator {
    int accumulatedCount;
    int accumulatedSum;
    int maxAccumulated;
    int maxLimit;
    int limitCounter;
    struct AccumulatorCallback *callback;
};

struct AccumulatorCallback {
    void *userData;
    OnAccumulated onAccumulated;
    OnLimitReached  onLimitReached;
};

struct AccumulatorCallback *createCallback(void *userData) {
    struct AccumulatorCallback *callback = (struct AccumulatorCallback*) malloc(sizeof(struct AccumulatorCallback));
    if (callback == NULL) {
        return NULL;
    }
    callback->userData = userData;
    callback->onAccumulated = NULL;
    callback->onLimitReached = NULL;
    return callback;
}

void registerAccumulatedCallback(struct AccumulatorCallback* callback, OnAccumulated onAccumulated) {
    if (callback != NULL) {
        callback->onAccumulated = onAccumulated;
    }
}

void registerLimitCallback(struct AccumulatorCallback* callback, OnLimitReached onLimitReached) {
    if (callback != NULL) {
        callback->onLimitReached = onLimitReached;
    }
}

void freeCallback(struct AccumulatorCallback *callback) {
    if (callback != NULL) {
        free(callback);
    }
    callback = NULL;
}

struct Accumulator *createAccumulator() {
    struct Accumulator *accumulator = (struct Accumulator *) malloc(sizeof(struct Accumulator));
    if (accumulator == NULL) {
        return NULL;
    }
    accumulator->accumulatedCount = 0;
    accumulator->accumulatedSum = 0;
    accumulator->maxAccumulated = 0;
    accumulator->limitCounter = 0;
    accumulator->maxLimit = 0;
    accumulator->callback = NULL;
    return accumulator;
}

void setLimit(struct Accumulator* accumulator, int limit) {
    if (accumulator != NULL) {
        accumulator->maxLimit = limit;
    }
}


void setMaxAccumulated(struct Accumulator *accumulator, int maxAccumulated) {
    if (accumulator != NULL) {
        accumulator->maxAccumulated = maxAccumulated;
    }
}

void setCallback(struct Accumulator* accumulator, struct AccumulatorCallback* callback) {
    if (accumulator != NULL) {
        accumulator->callback = callback;
    }
}

void accumulate(struct Accumulator *accumulator, int number) {
    if (accumulator == NULL) {
        return;
    }
    accumulator->limitCounter += 1;
    accumulator->accumulatedCount += 1;
    accumulator->accumulatedSum += number;
    write_log("Before check\n");
    if (accumulator->accumulatedCount >= accumulator->maxAccumulated) {
        struct AccumulatorCallback* callback = accumulator->callback;
        if (callback != NULL) {
            OnAccumulated onAccumulated = callback->onAccumulated;
            if (onAccumulated != NULL) {
                write_log("Invoking accumulated callback\n");
                onAccumulated(accumulator->accumulatedSum, accumulator->callback->userData);
                write_log("Accumulated callback invoked\n");
            }
        }
        accumulator->accumulatedSum = 0;
        accumulator->accumulatedCount = 0;
    }

    if (accumulator->limitCounter >= accumulator->maxLimit) {
        struct AccumulatorCallback* callback = accumulator->callback;
        if (callback != NULL) {
            OnLimitReached onLimitReached = callback->onLimitReached;
            if (onLimitReached != NULL) {
                write_log("Invoking limit callback\n");
                onLimitReached(callback->userData);
                write_log("Limit callback invoked\n");
            }
        }
    }

}

void freeAccumulator(struct Accumulator *accumulator) {
    if (accumulator != NULL) {
        free(accumulator);
    }
    accumulator = NULL;
}