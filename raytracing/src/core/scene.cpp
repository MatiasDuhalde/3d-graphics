#include <cmath>

#include "../../include/core/scene.h"
#include "../../include/utils/constants.h"

Scene::Scene() : intersectableObjects(std::vector<IntersectableObject *>()), lightSource(nullptr)
{
}

void Scene::addIntersectableObject(IntersectableObject &intersectableObject)
{
    intersectableObjects.push_back(&intersectableObject);
}

void Scene::setLightSource(LightSource &lightSource)
{
    this->lightSource = &lightSource;
}

const Intersection Scene::recursiveIntersect(const Ray &ray, const int depth) const
{
    if (depth >= MAX_RECURSION_DEPTH)
    {
        return Intersection();
    }

    Intersection intersection;

    for (IntersectableObject *intersectableObject : intersectableObjects)
    {
        Intersection currentIntersection = intersectableObject->intersect(ray);
        if (currentIntersection.getDistance() < intersection.getDistance())
        {
            intersection = currentIntersection;
        }
    }

    if (!intersection.isHit() || !intersection.isReflected())
    {
        return intersection;
    }

    Ray reflectedRay = intersection.getReflectedRay();

    return recursiveIntersect(reflectedRay, depth + 1);
}

const Intersection Scene::intersect(const Ray &ray) const
{
    return recursiveIntersect(ray, 0);
}

const Vector3 Scene::calculateLambertianShading(const Intersection &intersection) const
{
    if (lightSource == nullptr || !intersection.isHit())
    {
        return Vector3(0., 0., 0.);
    }

    const Vector3 intersectionPoint = intersection.getPoint();
    const Vector3 lightSourcePosition = lightSource->getPosition();

    const double lightSourceIntensity = lightSource->getIntensity();

    const Vector3 lightDirection = (lightSourcePosition - intersectionPoint);
    const Vector3 normalizedLightDirection = lightDirection.normalize();

    const Vector3 pointOverSurface = intersectionPoint + normalizedLightDirection * SURFACE_LIGHT_RAY_EPSILON;

    const Ray lightSourceRay(pointOverSurface, normalizedLightDirection);

    const Intersection lightIntersection = this->intersect(lightSourceRay);

    if (lightIntersection.isHit())
    {
        const double lightSourceDistance = (lightSourcePosition - intersectionPoint).norm();
        if (lightIntersection.getDistance() < lightSourceDistance)
        {
            return Vector3(0., 0., 0.);
        }
    }

    double d2 = lightDirection.norm2();

    const Vector3 intersectionNormal = intersection.getNormal();
    const Vector3 intersectionAlbedo = intersection.getAlbedo();

    const double surfacePower = lightSourceIntensity / (4. * M_PI * d2);

    const double k = surfacePower * std::max(0., intersectionNormal.dot(normalizedLightDirection)) / M_PI;
    return intersectionAlbedo * k;
}