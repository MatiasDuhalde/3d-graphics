#pragma once
#include "../utils/vector3.h"
#include "intersectable_object.h"
#include "intersection.h"
#include "ray.h"
#include <optional>

class Sphere : public IntersectableObject
{
  private:
    Vector3 center;
    double radius;

    std::optional<Vector3> albedo;
    bool mirror;

  public:
    Sphere(const Vector3 &center, const double radius);
    Sphere(const Vector3 &center, const double radius, const Vector3 &albedo);

    const Vector3 &getCenter() const;
    const double getRadius() const;
    const bool isMirror() const;
    void setCenter(const Vector3 &center);
    void setRadius(const double radius);

    virtual const Intersection intersect(const Ray &ray) const;
};
