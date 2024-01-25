#include "../../include/core/intersection_builder.h"

IntersectionBuilder &IntersectionBuilder::setHit(bool hit)
{
    this->hit = hit;
    return *this;
}

IntersectionBuilder &IntersectionBuilder::setPoint(const Vector3 &point)
{
    this->point = point;
    return *this;
}

IntersectionBuilder &IntersectionBuilder::setNormal(const Vector3 &normal)
{
    this->normal = normal;
    return *this;
}

IntersectionBuilder &IntersectionBuilder::setDistance(double distance)
{
    this->distance = distance;
    return *this;
}

IntersectionBuilder &IntersectionBuilder::setOpaque(bool opaque)
{
    this->opaque = opaque;
    return *this;
}

IntersectionBuilder &IntersectionBuilder::setAlbedo(const Vector3 &albedo)
{
    this->albedo = albedo;
    return *this;
}

IntersectionBuilder &IntersectionBuilder::setReflected(bool reflected)
{
    this->reflected = reflected;
    return *this;
}

IntersectionBuilder &IntersectionBuilder::setReflectedRay(const Ray &reflectedRay)
{
    this->reflectedRay = reflectedRay;
    return *this;
}

IntersectionBuilder &IntersectionBuilder::setRefracted(bool refracted)
{
    this->refracted = refracted;
    return *this;
}

IntersectionBuilder &IntersectionBuilder::setRefractedRay(const Ray &refractedRay)
{
    this->refractedRay = refractedRay;
    return *this;
}

Intersection IntersectionBuilder::build() const
{
    Intersection intersection;
    intersection.setHit(hit);
    if (point.has_value())
        intersection.setPoint(point.value());
    if (normal.has_value())
        intersection.setNormal(normal.value());
    intersection.setDistance(distance);
    intersection.setOpaque(opaque);
    if (albedo.has_value())
        intersection.setAlbedo(albedo.value());
    intersection.setReflected(reflected);
    if (reflectedRay.has_value())
        intersection.setReflectedRay(reflectedRay.value());
    intersection.setRefracted(refracted);
    if (refractedRay.has_value())
        intersection.setRefractedRay(refractedRay.value());
    return intersection;
}