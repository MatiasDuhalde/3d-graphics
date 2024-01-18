#pragma once
#include <iostream>

#include "../utils/vector3.h"

class Ray
{
private:
    const Vector3 origin;
    // Unit vector
    const Vector3 direction;

public:
    Ray(const Vector3 &origin, const Vector3 &direction);

    const Vector3 &getOrigin() const;
    const Vector3 &getDirection() const;

    friend std::ostream &operator<<(std::ostream &os, const Ray &ray);
};
