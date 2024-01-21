#include "../../include/core/ray.h"
#include "../../include/utils/constants.h"

const Vector3 &Ray::getOrigin() const
{
    return origin;
}

const Vector3 &Ray::getDirection() const
{
    return direction;
}

const double Ray::getRefractiveIndex() const
{
    return refractiveIndex;
}

Ray Ray::calculateReflectedRay(const Vector3 &intersectionPoint, const Vector3 &normal) const
{
    return Ray(intersectionPoint, direction - normal * 2 * direction.dot(normal), refractiveIndex);
}

Ray &Ray::addOffset()
{
    origin += direction * RAY_OFFSET_EPSILON;
    return *this;
}

std::ostream &operator<<(std::ostream &os, const Ray &ray)
{
    os << "Ray(origin: " << ray.origin << ", direction: " << ray.direction << ")";
    return os;
}