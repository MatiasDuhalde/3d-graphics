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
    const Intersection baseIntersection;
    const Intersection *intersection = &baseIntersection;

    for (IntersectableObject *intersectableObject : intersectableObjects)
    {
        Intersection currentIntersection = intersectableObject->intersect(ray);
        if (intersection == nullptr || currentIntersection.getDistance() < intersection->getDistance())
        {
            intersection = &currentIntersection;
        }
    }

    return *intersection;
}

const Vector3 Scene::calculateLambertianShading(const Intersection &intersection) const
{
    return Vector3(1., 1., 1.);
}