
#include "../../include/core/intersection.h"
#include <cmath>

const bool Intersection::isHit() const
{
    return hit;
}

const Intersection &Intersection::setHit(const bool hit)
{
    this->hit = hit;
    return *this;
}

const Vector3 &Intersection::getPoint() const
{
    if (!point.has_value())
    {
        throw Exception("Point is not set");
    }
    return point.value();
}

const Intersection &Intersection::setPoint(const Vector3 &point)
{
    this->point = point;
    return *this;
}

const Vector3 &Intersection::getNormal() const
{
    if (!normal.has_value())
    {
        throw Exception("Normal is not set");
    }
    return normal.value();
}

const Intersection &Intersection::setNormal(const Vector3 &normal)
{
    this->normal = normal;
    return *this;
}

const double Intersection::getDistance() const
{
    return distance;
}

const Intersection &Intersection::setDistance(const double distance)
{
    this->distance = distance;
    return *this;
}

const bool Intersection::isOpaque() const
{
    return opaque;
}

const Intersection &Intersection::setOpaque(const bool opaque)
{
    this->opaque = opaque;
    return *this;
}

const Vector3 &Intersection::getAlbedo() const
{
    if (!albedo.has_value())
    {
        throw Exception("Albedo is not set");
    }
    return albedo.value();
}

const Intersection &Intersection::setAlbedo(const Vector3 &albedo)
{
    this->albedo = albedo;
    return *this;
}

const bool Intersection::isReflected() const
{
    return reflected;
}

const Intersection &Intersection::setReflected(const bool reflected)
{
    this->reflected = reflected;
    return *this;
}

const Ray &Intersection::getReflectedRay() const
{
    if (!reflectedRay.has_value())
    {
        throw Exception("Reflected ray is not set");
    }
    return reflectedRay.value();
}

const Intersection &Intersection::setReflectedRay(const Ray &reflectedRay)
{
    this->reflectedRay = reflectedRay;
    return *this;
}

const bool Intersection::isRefracted() const
{
    return refracted;
}

const Intersection &Intersection::setRefracted(const bool refracted)
{
    this->refracted = refracted;
    return *this;
}

const Ray &Intersection::getRefractedRay() const
{
    if (!refractedRay.has_value())
    {
        throw Exception("Refracted ray is not set");
    }
    return refractedRay.value();
}

const Intersection &Intersection::setRefractedRay(const Ray &refractedRay)
{
    this->refractedRay = refractedRay;
    return *this;
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

Intersection::Exception::Exception(const std::string &message) : message(message)
{
}

const char *Intersection::Exception::what() const noexcept
{
    return message.c_str();
}
