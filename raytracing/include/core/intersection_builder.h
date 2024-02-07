#pragma once
#include "intersection.h"

class IntersectionBuilder
{
  private:
    bool hit = Intersection::Defaults::HIT;
    std::optional<Vector3> point = Intersection::Defaults::POINT;
    std::optional<Vector3> normal = Intersection::Defaults::NORMAL;
    double distance = Intersection::Defaults::DISTANCE;
    bool opaque = Intersection::Defaults::OPAQUE;
    std::optional<Vector3> albedo = Intersection::Defaults::ALBEDO;
    std::optional<Ray> sourceRay = Intersection::Defaults::SOURCE_RAY;
    bool reflected = Intersection::Defaults::REFLECTED;
    std::optional<Ray> reflectedRay = Intersection::Defaults::REFLECTED_RAY;
    bool refracted = Intersection::Defaults::REFRACTED;
    std::optional<Ray> refractedRay = Intersection::Defaults::REFRACTED_RAY;

  public:
    constexpr IntersectionBuilder &setHit(const bool hit);
    constexpr IntersectionBuilder &setPoint(const Vector3 &point);
    constexpr IntersectionBuilder &setNormal(const Vector3 &normal);
    constexpr IntersectionBuilder &setDistance(const double distance);
    constexpr IntersectionBuilder &setOpaque(const bool opaque);
    constexpr IntersectionBuilder &setAlbedo(const Vector3 &albedo);
    constexpr IntersectionBuilder &setSourceRay(const Ray &sourceRay);
    constexpr IntersectionBuilder &setReflected(const bool reflected);
    constexpr IntersectionBuilder &setReflectedRay(const Ray &reflectedRay);
    constexpr IntersectionBuilder &setRefracted(const bool refracted);
    constexpr IntersectionBuilder &setRefractedRay(const Ray &refractedRay);
    constexpr Intersection build() const;
};

constexpr IntersectionBuilder &IntersectionBuilder::setHit(const bool hit)
{
    this->hit = hit;
    return *this;
}

constexpr IntersectionBuilder &IntersectionBuilder::setPoint(const Vector3 &point)
{
    this->point = point;
    return *this;
}

constexpr IntersectionBuilder &IntersectionBuilder::setNormal(const Vector3 &normal)
{
    this->normal = normal;
    return *this;
}

constexpr IntersectionBuilder &IntersectionBuilder::setDistance(const double distance)
{
    this->distance = distance;
    return *this;
}

constexpr IntersectionBuilder &IntersectionBuilder::setOpaque(const bool opaque)
{
    this->opaque = opaque;
    return *this;
}

constexpr IntersectionBuilder &IntersectionBuilder::setAlbedo(const Vector3 &albedo)
{
    this->albedo = albedo;
    return *this;
}

constexpr IntersectionBuilder &IntersectionBuilder::setSourceRay(const Ray &sourceRay)
{
    this->sourceRay = sourceRay;
    return *this;
}

constexpr IntersectionBuilder &IntersectionBuilder::setReflected(const bool reflected)
{
    this->reflected = reflected;
    return *this;
}

constexpr IntersectionBuilder &IntersectionBuilder::setReflectedRay(const Ray &reflectedRay)
{
    this->reflectedRay = reflectedRay;
    return *this;
}

constexpr IntersectionBuilder &IntersectionBuilder::setRefracted(const bool refracted)
{
    this->refracted = refracted;
    return *this;
}

constexpr IntersectionBuilder &IntersectionBuilder::setRefractedRay(const Ray &refractedRay)
{
    this->refractedRay = refractedRay;
    return *this;
}

constexpr Intersection IntersectionBuilder::build() const
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
    if (sourceRay.has_value())
        intersection.setSourceRay(sourceRay.value());
    intersection.setReflected(reflected);
    if (reflectedRay.has_value())
        intersection.setReflectedRay(reflectedRay.value());
    intersection.setRefracted(refracted);
    if (refractedRay.has_value())
        intersection.setRefractedRay(refractedRay.value());
    return intersection;
}
