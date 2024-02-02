#pragma once
#include <random>

std::random_device randomDevice;
std::default_random_engine randomEngine(randomDevice());
std::uniform_real_distribution<double> randomDistribution = std::uniform_real_distribution<double>(0.0, 1.0);