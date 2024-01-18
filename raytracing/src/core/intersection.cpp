
#include <cmath>
#include "../../include/core/intersection.h"

Intersection::Intersection() : hit(false), point(nullptr), distance(INFINITY)
{
}

Intersection::Intersection(const bool hit, const Vector3 &point, const Vector3 &normal, const double distance) : hit(hit), point(&point), normal(&normal), distance(distance)
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

const double Intersection::getDistance() const
{
    return this->distance;
}

std::ostream &operator<<(std::ostream &os, const Intersection &intersection)
{
    os << "Intersection(hit: " << intersection.hit << ", point: " << intersection.point << ", distance: " << intersection.distance << ")";
    return os;
}