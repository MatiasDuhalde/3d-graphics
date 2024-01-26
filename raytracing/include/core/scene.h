#pragma once
#include "intersectable_object.h"
#include "intersection.h"
#include "light_source.h"
#include <vector>

/**
 * @brief Describes a scene with intersectable objects and light sources
 *
 */
class Scene
{
  private:
    std::vector<IntersectableObject *> intersectableObjects;
    std::vector<LightSource *> lightSources;

    const bool lightSourceReachesPoint(const LightSource &lightSource, const Vector3 &point) const;
    const Vector3 calculateLambertianShading(const LightSource &lightSource,
                                             const Intersection &diffuseIntersection) const;
    const Vector3 calculateColorRecursive(const Intersection &intersection, int depth, bool refracting) const;

  public:
    Scene();

    Scene &addIntersectableObject(IntersectableObject &intersectableObject);
    Scene &addLightSource(LightSource &lightSource);

    /**
     * @brief Intersect all objects in the scene with the given ray
     *
     * @param ray
     * @return const Intersection The closest intersection with the ray
     */
    const Intersection intersect(const Ray &ray) const;

    /**
     * @brief Calculate the shading of the given intersection
     *
     * @param intersection
     * @return const Vector3 The shading of the intersection
     */
    const Vector3 calculateColor(const Intersection &intersection) const;
};
