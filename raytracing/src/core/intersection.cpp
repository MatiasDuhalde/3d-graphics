
#include "../../include/core/intersection.h"
#include <cmath>

Intersection::Intersection() : hit(false), point(), normal(), distance(INFINITY), albedo()
{
}

Intersection::Intersection(const bool hit, const Vector3 &point, const Vector3 &normal, const double distance,
                           const Vector3 &albedo)
    : hit(hit), point(point), normal(normal), distance(distance), albedo(albedo)
{
}

const bool Intersection::isHit() const
{
    return this->hit;
}

const Vector3 &Intersection::getPoint() const
{
    if (!this->point.has_value())
    {
        throw UnsetIntersectionException();
    }
    return this->point.value();
}

const Vector3 &Intersection::getNormal() const
{
    if (!this->normal.has_value())
    {
        throw UnsetIntersectionException();
    }
    return this->normal.value();
}

const double Intersection::getDistance() const
{
    return this->distance;
}

const Vector3 &Intersection::getAlbedo() const
{
    if (!this->albedo.has_value())
    {
        throw UnsetIntersectionException();
    }
    return this->albedo.value();
}

std::ostream &operator<<(std::ostream &os, const Intersection &intersection)
{
    os << "Intersection(hit: " << intersection.hit;
    if (intersection.point.has_value())
    {
        os << ", point: " << intersection.point.value();
    }
    if (intersection.normal.has_value())
    {
        os << ", normal: " << intersection.normal.value();
    }
    os << ", distance: " << intersection.distance;
    if (intersection.albedo.has_value())
    {
        os << ", albedo: " << intersection.albedo.value();
    }
    os << ")";

    return os;
}