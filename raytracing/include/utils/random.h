#pragma once
#include "vector3.h"
#include <random>

// FIXME: std::default_random_engine is not thread safe
static std::random_device randomDevice;
static std::default_random_engine randomEngine(randomDevice());
static std::uniform_real_distribution<double> randomDistribution = std::uniform_real_distribution<double>(0.0, 1.0);

inline const Vector3 boxMuller(double const stdev)
{
    double u1 = sqrt(-2 * log(randomDistribution(randomEngine))) * stdev;
    double u2 = 2 * M_PI * randomDistribution(randomEngine);
    return Vector3(u1 * cos(u2), u1 * sin(u2), 0);
}
