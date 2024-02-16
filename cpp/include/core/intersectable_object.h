#pragma once
#include "intersection.h"
#include "object.h"
#include "ray.h"

/**
 * @brief Describes an object that can be intersected by a ray
 *
 */
class IntersectableObject : public Object
{
  public:
    virtual const Intersection intersect(const Ray &ray) const = 0;
};
