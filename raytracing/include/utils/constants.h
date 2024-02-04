#pragma once

constexpr double RAY_OFFSET_EPSILON = 1E-6;
constexpr double GAMMA_CORRECTION = 1. / 2.2;
constexpr int MAX_RECURSION_DEPTH = 5;
constexpr int FRESNEL_RAYS = 4096;
constexpr int INDIRECT_LIGHTING_RAYS = 256;

constexpr bool ENABLE_FRESNEL = true;
constexpr bool ENABLE_INDIRECT_LIGHTING = true;
