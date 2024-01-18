
#include "../../include/core/intersection.h"
#include <cmath>

Intersection::Intersection() : hit(false), point(nullptr), normal(nullptr), distance(INFINITY), albedo(nullptr)
{
}

Intersection::Intersection(const bool hit, const Vector3 &point, const Vector3 &normal, const double distance,
                           const Vector3 &albedo)
    : hit(hit), point(&point), normal(&normal), distance(distance), albedo(&albedo)
{
}

const bool Intersection::isHit() const
{
    return this->hit;
}

const Vector3 &Intersection::getPoint() const
{
    return *this->point;
}

const Vector3 &Intersection::getNormal() const
{
    return *this->normal;
}

const double Intersection::getDistance() const
{
    return this->distance;
}

const Vector3 &Intersection::getAlbedo() const
{
    return *this->albedo;
}

std::ostream &operator<<(std::ostream &os, const Intersection &intersection)
{
    os << "Intersection(hit: " << intersection.hit << ", point: " << intersection.point
       << ", distance: " << intersection.distance << ")";
    return os;
}