typedef void (*OnAccumulated)(int accumulated, void* data);
typedef void (*OnLimitReached)(void* data);

struct Accumulator;
struct AccumulatorCallback;

struct AccumulatorCallback* createCallback(void* userData);
void registerAccumulatedCallback(struct AccumulatorCallback* callback, OnAccumulated onAccumulated);
void registerLimitCallback(struct AccumulatorCallback* callback, OnLimitReached onLimitReached);

struct Accumulator* createAccumulator();
void setMaxAccumulated(struct Accumulator* accumulator, int maxAccumulated);
void setLimit(struct Accumulator* accumulator, int limit);
void setCallback(struct Accumulator *accumulator, struct AccumulatorCallback* callback);
void accumulate(struct Accumulator* accumulator, int number);

void freeCallback(struct AccumulatorCallback* callback);
void freeAccumulator(struct Accumulator* accumulator);