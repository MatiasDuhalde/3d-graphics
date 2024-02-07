#pragma once
#include <cmath>
#include <vector>

#include "../utils/constants.h"
#include "../utils/random.h"
#include "intersectable_object.h"
#include "intersection.h"
#include "light_source.h"

/**
 * @brief Describes a scene with intersectable objects and light sources
 *
 */
class Scene
{
  private:
    std::vector<const IntersectableObject *> intersectableObjects;
    std::vector<const LightSource *> lightSources;

    constexpr bool lightSourceReachesPoint(const LightSource &lightSource, const Vector3 &point) const;
    constexpr Vector3 calculateLambertianShading(const LightSource &lightSource,
                                                 const Intersection &diffuseIntersection) const;
    constexpr Vector3 calculateColorRecursive(const Intersection &intersection, int depth) const;
    constexpr Vector3 calculateColorRecursive(const Intersection &intersection, int depth, bool multiSampling) const;
    constexpr Intersection calculateReflectedIntersection(const Intersection &intersection) const;
    constexpr Intersection calculateRefractedIntersection(const Intersection &intersection) const;
    constexpr Vector3 calculateFresnelColor(const Intersection &intersection, const int depth,
                                            const bool multiSampling) const;
    constexpr Vector3 calculateIndirectLightingColor(const Intersection &intersection, const int depth,
                                                     const bool multiSampling) const;

  public:
    Scene();

    constexpr Scene &addIntersectableObject(const IntersectableObject &intersectableObject);
    constexpr Scene &addLightSource(const LightSource &lightSource);

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
     * @param multiSampling Whether the calculation starts multi-sampling
     * @return const Vector3 The shading of the intersection
     */
    constexpr Vector3 calculateColor(const Intersection &intersection) const;
    constexpr Vector3 calculateColor(const Intersection &intersection, const bool multiSampling = false) const;
};

inline Scene::Scene()
    : intersectableObjects(std::vector<const IntersectableObject *>()), lightSources(std::vector<const LightSource *>())
{
}

constexpr Scene &Scene::addIntersectableObject(const IntersectableObject &intersectableObject)
{
    intersectableObjects.push_back(&intersectableObject);
    return *this;
}

constexpr Scene &Scene::addLightSource(const LightSource &lightSource)
{
    lightSources.push_back(&lightSource);
    return *this;
}

inline const Intersection Scene::intersect(const Ray &ray) const
{
    Intersection intersection;

    for (const IntersectableObject *intersectableObject : intersectableObjects)
    {
        Intersection currentIntersection = intersectableObject->intersect(ray);
        if (currentIntersection.getDistance() < intersection.getDistance())
            intersection = currentIntersection;
    }

    return intersection;
}

constexpr bool Scene::lightSourceReachesPoint(const LightSource &lightSource, const Vector3 &point) const
{
    const Vector3 lightSourcePosition = lightSource.getPosition();

    const Vector3 lightDirection = (lightSourcePosition - point);
    const Vector3 normalizedLightDirection = lightDirection.normalize();

    const Ray rayToLightSource = Ray(point, normalizedLightDirection).addOffset();

    const Intersection lightIntersection = intersect(rayToLightSource);

    if (lightIntersection.isHit())
        return lightIntersection.getDistance() >= (lightSourcePosition - point).norm();
    return true;
}

constexpr Vector3 Scene::calculateLambertianShading(const LightSource &lightSource,
                                                    const Intersection &diffuseIntersection) const
{
    const Vector3 lightDirection = lightSource.getPosition() - diffuseIntersection.getPoint();
    const double d2 = lightDirection.norm2();

    const Vector3 intersectionNormal = diffuseIntersection.getNormal();
    const Vector3 intersectionAlbedo = diffuseIntersection.getAlbedo();

    const double surfacePower = lightSource.getIntensity() / (4. * M_PI * d2);

    const double k = surfacePower * std::max(0., intersectionNormal.dot(lightDirection.normalize())) / M_PI;
    return intersectionAlbedo * k;
}

constexpr Vector3 Scene::calculateColorRecursive(const Intersection &intersection, int depth,
                                                 bool multiSampling = false) const
{
    if (depth > MAX_RECURSION_DEPTH || !intersection.isHit())
        return Vector3(0., 0., 0.);

    if (intersection.isOpaque())
    {
        const LightSource lightSource = *lightSources[0];
        Vector3 directLighting(0., 0., 0.);
        if (lightSourceReachesPoint(lightSource, intersection.getPoint()))
            directLighting = calculateLambertianShading(lightSource, intersection);

        if (ENABLE_INDIRECT_LIGHTING)
        {
            const Vector3 indirectLighting = calculateIndirectLightingColor(intersection, depth, multiSampling);
            return directLighting + indirectLighting;
        }

        return directLighting;
    }

    if (intersection.isReflected())
    {
        const Intersection reflectedIntersection = calculateReflectedIntersection(intersection);
        return calculateColorRecursive(reflectedIntersection, depth + 1, multiSampling);
    }

    if (intersection.isRefracted())
    {
        if (ENABLE_FRESNEL)
            return calculateFresnelColor(intersection, depth, multiSampling);

        const Intersection refractedIntersection = calculateRefractedIntersection(intersection);
        return calculateColorRecursive(refractedIntersection, depth + 1, multiSampling);
    }

    return Vector3(0., 0., 0.);
}

constexpr Intersection Scene::calculateReflectedIntersection(const Intersection &intersection) const
{
    const Ray reflectedRay = intersection.getReflectedRay();
    return intersect(reflectedRay);
}

constexpr Intersection Scene::calculateRefractedIntersection(const Intersection &intersection) const
{
    const Ray refractedRay = intersection.getRefractedRay();
    return intersect(refractedRay);
}

constexpr Vector3 Scene::calculateFresnelColor(const Intersection &intersection, const int depth,
                                               const bool multiSampling) const
{
    int repetitions = multiSampling ? 1 : FRESNEL_RAYS;

    double reflectionCoefficient = intersection.getReflectionCoefficient();

    Vector3 aggregateVector = Vector3(0., 0., 0.);
    for (int i = 0; i < repetitions; i++)
    {
        if (randomDistribution(randomEngine) < reflectionCoefficient)
        {
            const Intersection reflectedIntersection = calculateReflectedIntersection(intersection);
            aggregateVector += calculateColorRecursive(reflectedIntersection, depth + 1, true);
        }
        else
        {
            const Intersection refractedIntersection = calculateRefractedIntersection(intersection);
            aggregateVector += calculateColorRecursive(refractedIntersection, depth + 1, true);
        }
    }
    return aggregateVector / repetitions;
}

constexpr Vector3 Scene::calculateIndirectLightingColor(const Intersection &intersection, const int depth,
                                                        const bool multiSampling) const
{
    int repetitions = multiSampling ? 1 : INDIRECT_LIGHTING_RAYS;

    Vector3 aggregateVector = Vector3(0., 0., 0.);

    for (int i = 0; i < repetitions; i++)
    {
        const Ray randomRay = intersection.getRandomNormalHemisphereRay();
        const Intersection randomIntersection = intersect(randomRay);
        aggregateVector += calculateColorRecursive(randomIntersection, depth + 1, true);
    }

    const Vector3 indirectLighting = intersection.getAlbedo() * (aggregateVector / repetitions);

    return indirectLighting;
}

constexpr Vector3 Scene::calculateColor(const Intersection &intersection, const bool multiSampling) const
{
    if (lightSources.empty() || !intersection.isHit())
        return Vector3(0., 0., 0.);

    return calculateColorRecursive(intersection, 1, multiSampling);
}
