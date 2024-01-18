#pragma once
#include "intersection.h"
#include "object.h"
#include "ray.h"

class IntersectableObject : public Object
{
  public:
    virtual const Intersection intersect(const Ray &ray) const = 0;
};
