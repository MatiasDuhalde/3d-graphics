#include <cmath>
#include <random>

#include "../../include/core/scene.h"
#include "../../include/utils/constants.h"

std::random_device rd;
std::default_random_engine generator(rd());
std::uniform_real_distribution<double> randomDistribution = std::uniform_real_distribution<double>(0.0, 1.0);

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
        {
            intersection = currentIntersection;
        }
    }

    return intersection;
}

const bool Scene::lightSourceReachesPoint(const LightSource &lightSource, const Vector3 &point) const
{
    const Vector3 lightSourcePosition = lightSource.getPosition();

    const Vector3 lightDirection = (lightSourcePosition - point);
    const Vector3 normalizedLightDirection = lightDirection.normalize();

    Ray rayToLightSource(point, normalizedLightDirection);
    rayToLightSource.addOffset();

    const Intersection lightIntersection = intersect(rayToLightSource);

    if (lightIntersection.isHit())
    {
        const double lightSourceDistance = (lightSourcePosition - point).norm();
        if (lightIntersection.getDistance() < lightSourceDistance)
        {
            return false;
        }
    }
    return true;
}

const Vector3 Scene::calculateLambertianShading(const LightSource &lightSource,
                                                const Intersection &diffuseIntersection) const
{
    const Vector3 lightDirection = lightSource.getPosition() - diffuseIntersection.getPoint();
    double d2 = lightDirection.norm2();

    const Vector3 intersectionNormal = diffuseIntersection.getNormal();
    const Vector3 intersectionAlbedo = diffuseIntersection.getAlbedo();

    const double surfacePower = lightSource.getIntensity() / (4. * M_PI * d2);

    const double k = surfacePower * std::max(0., intersectionNormal.dot(lightDirection.normalize())) / M_PI;
    return intersectionAlbedo * k;
}

const Vector3 Scene::calculateColorRecursive(const Intersection &intersection, int depth, bool refracting = false) const
{
    if (depth > MAX_RECURSION_DEPTH || !intersection.isHit())
    {
        return Vector3(0., 0., 0.);
    }

    if (intersection.isOpaque())
    {
        const LightSource lightSource = *lightSources[0];
        if (!lightSourceReachesPoint(lightSource, intersection.getPoint()))
        {
            return Vector3(0., 0., 0.);
        }
        return calculateLambertianShading(lightSource, intersection);
    }

    if (intersection.isReflected())
    {
        const Ray reflectedRay = intersection.getReflectedRay();
        const Intersection reflectedIntersection = intersect(reflectedRay);
        return calculateColorRecursive(reflectedIntersection, depth + 1);
    }

    if (intersection.isRefracted())
    {
        int repetitions = REFRACTION_RAYS;
        if (refracting)
        {
            repetitions = 1;
        }

        double reflectionCoefficient = intersection.getReflectionCoefficient();

        Vector3 averageVector = Vector3(0., 0., 0.);
        for (int i = 0; i < repetitions; i++)
        {
            if (randomDistribution(generator) < reflectionCoefficient)
            {
                const Ray reflectedRay = intersection.getReflectedRay();
                const Intersection reflectedIntersection = intersect(reflectedRay);
                averageVector += calculateColorRecursive(reflectedIntersection, depth + 1, true);
            }
            else
            {
                const Ray refractedRay = intersection.getRefractedRay();
                const Intersection refractedIntersection = intersect(refractedRay);
                averageVector += calculateColorRecursive(refractedIntersection, depth + 1, true);
            }
        }
        return averageVector / repetitions;
    }

    if (intersection.isHit())
        const int i = 123;

    return Vector3(0., 0., 0.);
}

const Vector3 Scene::calculateColor(const Intersection &intersection) const
{
    if (lightSources.empty() || !intersection.isHit())
    {
        return Vector3(0., 0., 0.);
    }

    return calculateColorRecursive(intersection, 0);
}
