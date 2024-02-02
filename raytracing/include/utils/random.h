#pragma once
#include <random>

static std::random_device randomDevice;
static std::default_random_engine randomEngine(randomDevice());
static std::uniform_real_distribution<double> randomDistribution = std::uniform_real_distribution<double>(0.0, 1.0);