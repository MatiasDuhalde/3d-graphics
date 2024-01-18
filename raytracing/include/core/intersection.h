#pragma once
#include <iostream>

#include "../utils/vector3.h"

class Intersection
{
private:
    const bool hit;
    const Vector3 *point;
    const Vector3 *normal;
    const double distance;

public:
    Intersection();
    Intersection(const bool hit, const Vector3 &point, const Vector3 &normal, const double distance);
    const bool isHit() const;
    const Vector3 &getPoint() const;
    const double getDistance() const;

    friend std::ostream &operator<<(std::ostream &os, const Intersection &intersection);
};
