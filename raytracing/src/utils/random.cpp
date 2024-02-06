#include "../../include/utils/random.h"

void boxMuller(double const stdev, double &x, double &y)
{
    double u1 = randomDistribution(randomEngine);
    double u2 = randomDistribution(randomEngine);
    x = sqrt(-2 * log(u1)) * cos(2 * M_PI * u2) * stdev;
    y = sqrt(-2 * log(u1)) * sin(2 * M_PI * u2) * stdev;
}