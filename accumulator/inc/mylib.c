#include "mylib.h"
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

void write_log(const char* msg) {
    write(1, msg, strlen(msg));
}

Accumulator createAccumulator() {
    Accumulator accumulator;
    accumulator.accumulatedCount = 0;
    accumulator.accumulatedSum = 0;
    accumulator.maxAccumulated = 0;
    accumulator.callback = NULL;
    accumulator.callbackData = NULL;
    return accumulator;
}

void setMaxAccumulated(Accumulator* accumulator, int maxAccumulated) {
    accumulator->maxAccumulated = maxAccumulated;
}

void registerCallback(Accumulator* accumulator, OnAccumulated callback, void* callbackData) {
    accumulator->callback = callback;
    accumulator->callbackData = callbackData;
}
void accumulate(Accumulator* accumulator, int number) {
    accumulator->accumulatedCount += 1;
    accumulator->accumulatedSum += number;
    write_log("Before check\n");
    if (accumulator->accumulatedCount >= accumulator->maxAccumulated) {
        if (accumulator->callback != NULL) {
            write_log("Invocating callback\n");
            accumulator->callback(accumulator->accumulatedSum, accumulator->callbackData);
            write_log("Callback invoked\n");
        }
        accumulator->accumulatedSum = 0;
        accumulator->accumulatedCount = 0;
    }
}

