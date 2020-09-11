typedef void (*OnAccumulated)(int accumulated, void* data);

typedef struct {
    int accumulatedCount;
    int accumulatedSum;
    int maxAccumulated;
    OnAccumulated callback;
    void* callbackData;
} Accumulator;

Accumulator createAccumulator();
void setMaxAccumulated(Accumulator* accumulator, int maxAccumulated);
void registerCallback(Accumulator* accumulator, OnAccumulated callback, void* callbackData);
void accumulate(Accumulator* accumulator, int number);
