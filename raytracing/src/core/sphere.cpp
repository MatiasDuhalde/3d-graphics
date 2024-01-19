#include "../../include/core/sphere.h"
#include "../../include/utils/constants.h"
#include <cmath>
#include <iostream>

Sphere::Sphere(const Vector3 &center, const double radius) : center(center), radius(radius), albedo(), mirror(true)
{
}

Sphere::Sphere(const Vector3 &center, const double radius, const Vector3 &albedo)
    : center(center), radius(radius), albedo(albedo), mirror(false)
{
}

const Vector3 &Sphere::getCenter() const
{
    return this->center;
}

const double Sphere::getRadius() const
{
    return this->radius;
}

const bool Sphere::isMirror() const
{
    return this->mirror;
}

void Sphere::setCenter(const Vector3 &center)
{
    this->center = center;
}

void Sphere::setRadius(const double radius)
{
    this->radius = radius;
}

const Intersection Sphere::intersect(const Ray &ray) const
{
    const Vector3 rayOrigin = ray.getOrigin();
    const Vector3 rayDirection = ray.getDirection();

    const Vector3 centerToOrigin = rayOrigin - this->center;

    const double distanceDot = rayDirection.dot(centerToOrigin);

    const double determinant = pow(distanceDot, 2) - centerToOrigin.norm2() + pow(this->radius, 2);

    if (determinant < 0)
    {
        return Intersection();
    }

    const double t1 = -distanceDot - sqrt(determinant);
    const double t2 = -distanceDot + sqrt(determinant);

    if (t2 < 0)
    {
        return Intersection();
    }

    double distance;

    if (t1 >= 0)
    {
        distance = t1;
    }
    else
    {
        distance = t2;
    }

    const Vector3 intersectionPoint = rayOrigin + rayDirection * distance;

    const Vector3 normal = (intersectionPoint - this->center).normalize();

    if (this->mirror)
    {
        const Vector3 reflectedDirection = rayDirection - normal * 2 * rayDirection.dot(normal);
        const Vector3 pointOverSurface = intersectionPoint + reflectedDirection * SURFACE_LIGHT_RAY_EPSILON;
        const Ray reflectedRay(pointOverSurface, reflectedDirection);

        return Intersection(true, intersectionPoint, normal, distance, reflectedRay);
    }

    return Intersection(true, intersectionPoint, normal, distance, albedo.value());
}