#include <cmath>

#include "../../include/core/scene.h"

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
    if (lightSource == nullptr)
    {
        return Vector3(0., 0., 0.);
    }

    const Vector3 lightSourcePosition = lightSource->getPosition();
    const double lightSourceIntensity = lightSource->getIntensity();
    const Vector3 intersectionPoint = intersection.getPoint();
    const Vector3 intersectionNormal = intersection.getNormal();
    const Vector3 intersectionAlbedo = intersection.getAlbedo();

    const Vector3 lightDirection = (lightSourcePosition - intersectionPoint);
    double d2 = lightDirection.norm2();
    const Vector3 normalizedLightDirection = lightDirection.normalize();

    const double surfacePower = lightSourceIntensity / (4 * M_PI * d2);

    const double k = surfacePower * std::max(0., intersectionNormal.dot(normalizedLightDirection)) / M_PI;
    return intersectionAlbedo * k;
}