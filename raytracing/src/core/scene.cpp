#include <cmath>

#include "../../include/core/scene.h"
#include "../../include/utils/constants.h"
#include "../../include/utils/random.h"

Scene::Scene() : intersectableObjects(std::vector<IntersectableObject *>()), lightSources(std::vector<LightSource *>())
{
}

Scene &Scene::addIntersectableObject(IntersectableObject &intersectableObject)
{
    intersectableObjects.push_back(&intersectableObject);
    return *this;
}

Scene &Scene::addLightSource(LightSource &lightSource)
{
    lightSources.push_back(&lightSource);
    return *this;
}

const Intersection Scene::intersect(const Ray &ray) const
{
    Intersection intersection;

    for (IntersectableObject *intersectableObject : intersectableObjects)
    {
        Intersection currentIntersection = intersectableObject->intersect(ray);
        if (currentIntersection.getDistance() < intersection.getDistance())
            intersection = currentIntersection;
    }

    return intersection;
}

const bool Scene::lightSourceReachesPoint(const LightSource &lightSource, const Vector3 &point) const
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

const Vector3 Scene::calculateLambertianShading(const LightSource &lightSource,
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

const Vector3 Scene::calculateColorRecursive(const Intersection &intersection, int depth,
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

const Intersection Scene::calculateReflectedIntersection(const Intersection &intersection) const
{
    const Ray reflectedRay = intersection.getReflectedRay();
    return intersect(reflectedRay);
}

const Intersection Scene::calculateRefractedIntersection(const Intersection &intersection) const
{
    const Ray refractedRay = intersection.getRefractedRay();
    return intersect(refractedRay);
}

const Vector3 Scene::calculateFresnelColor(const Intersection &intersection, const int depth,
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

const Vector3 Scene::calculateIndirectLightingColor(const Intersection &intersection, const int depth,
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

const Vector3 Scene::calculateColor(const Intersection &intersection, const bool multiSampling = false) const
{
    if (lightSources.empty() || !intersection.isHit())
        return Vector3(0., 0., 0.);

    return calculateColorRecursive(intersection, 1, multiSampling);
}
