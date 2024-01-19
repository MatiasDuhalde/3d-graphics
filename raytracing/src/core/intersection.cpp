
#include "../../include/core/intersection.h"
#include <cmath>

Intersection::Intersection()
    : hit(false), point(), normal(), distance(INFINITY), albedo(), reflected(false), reflectedRay()
{
}

Intersection::Intersection(const bool hit, const Vector3 &point, const Vector3 &normal, const double distance,
                           const Ray &reflectedRay)
    : hit(hit), point(point), normal(normal), distance(distance), albedo(), reflected(true), reflectedRay(reflectedRay)
{
}

Intersection::Intersection(const bool hit, const Vector3 &point, const Vector3 &normal, const double distance,
                           const Vector3 &albedo)
    : hit(hit), point(point), normal(normal), distance(distance), albedo(albedo), reflected(false), reflectedRay()
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
        throw UnsetIntersectionPointException(*this);
    }
    return this->point.value();
}

const Vector3 &Intersection::getNormal() const
{
    if (!this->normal.has_value())
    {
        throw UnsetIntersectionNormalException(*this);
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
        throw UnsetIntersectionAlbedoException(*this);
    }
    return this->albedo.value();
}

const bool Intersection::isReflected() const
{
    return this->reflected;
}

const Ray &Intersection::getReflectedRay() const
{
    return this->reflectedRay.value();
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
    os << ", reflected: " << intersection.reflected;
    if (intersection.reflectedRay.has_value())
    {
        os << ", reflectedRay: " << intersection.reflectedRay.value();
    }
    os << ")";

    return os;
}