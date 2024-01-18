#include "../../include/core/ray.h"

Ray::Ray(const Vector3 &origin, const Vector3 &direction) : origin(origin), direction(direction)
{
}

const Vector3 &Ray::getOrigin() const
{
    return this->origin;
}

const Vector3 &Ray::getDirection() const
{
    return this->direction;
}

std::ostream &operator<<(std::ostream &os, const Ray &ray)
{
    os << "Ray(origin: " << ray.origin << ", direction: " << ray.direction << ")";
    return os;
}