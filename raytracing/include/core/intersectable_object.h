#pragma once
#include "object.h"
#include "ray.h"
#include "intersection.h"

class IntersectableObject : public Object
{

public:
    virtual const Intersection intersect(const Ray &ray) const = 0;
};
