#pragma once
#include "../utils/vector3.h"
#include "ray.h"
#include "intersection.h"
#include "intersectable_object.h"

class Sphere : public IntersectableObject
{
private:
    Vector3 center;
    double radius;

    double albedo;

public:
    Sphere(const Vector3 &center, const double radius);

    const Vector3 &getCenter() const;
    const double getRadius() const;
    void setCenter(const Vector3 &center);
    void setRadius(const double radius);

    virtual const Intersection intersect(const Ray &ray) const;
};
