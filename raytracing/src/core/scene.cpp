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

const Vector3 Scene::calculateLambertianShading(const Intersection &intersection) const
{
    Intersection diffuseIntersection;

    if (intersection.isReflected())
    {
        Intersection recursiveIntersection = intersection;

        int depth = 0;
        while (recursiveIntersection.isReflected())
        {
            const Ray recursiveRay = recursiveIntersection.getReflectedRay();
            recursiveIntersection = intersect(recursiveRay);
            if (depth > MAX_RECURSION_DEPTH)
            {
                return Vector3(0., 0., 0.);
            }
            depth++;
        }

        diffuseIntersection = recursiveIntersection;
    }
    else
    {
        diffuseIntersection = intersection;
    }

    if (lightSource == nullptr || !diffuseIntersection.isHit())
    {
        return Vector3(0., 0., 0.);
    }

    const Vector3 intersectionPoint = diffuseIntersection.getPoint();
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

    const Vector3 intersectionNormal = diffuseIntersection.getNormal();
    const Vector3 intersectionAlbedo = diffuseIntersection.getAlbedo();

    const double surfacePower = lightSourceIntensity / (4. * M_PI * d2);

    const double k = surfacePower * std::max(0., intersectionNormal.dot(normalizedLightDirection)) / M_PI;
    return intersectionAlbedo * k;
}
