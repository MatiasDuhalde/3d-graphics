#pragma once
#include "intersectable_object.h"
#include "intersection.h"
#include "light_source.h"
#include <vector>

class Scene
{
  private:
    std::vector<IntersectableObject *> intersectableObjects;
    LightSource *lightSource;

  public:
    Scene();

    void addIntersectableObject(IntersectableObject &intersectableObject);

    void setLightSource(LightSource &lightSource);

    const Intersection intersect(const Ray &ray) const;

    // calculate lambertian shading
    const Vector3 calculateLambertianShading(const Intersection &intersection) const;
};
