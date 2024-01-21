#include "../../include/core/sphere.h"
#include "../../include/core/intersection_builder.h"
#include "../../include/utils/constants.h"
#include <cmath>
#include <iostream>

Sphere::Sphere(const Vector3 &center, const double radius) : center(center), radius(radius)
{
}

const Vector3 &Sphere::getCenter() const
{
    return center;
}

const Sphere &Sphere::setCenter(const Vector3 &center)
{
    this->center = center;
    return *this;
}

const double Sphere::getRadius() const
{
    return radius;
}

const Sphere &Sphere::setRadius(const double radius)
{
    this->radius = radius;
    return *this;
}

const Sphere &Sphere::setColor(const Vector3 &color)
{
    this->color = color;
    return *this;
}

const Sphere &Sphere::setMirror(const bool mirror)
{
    this->mirror = mirror;
    return *this;
}

const Sphere &Sphere::setTransparent(const bool transparent)
{
    this->transparent = transparent;
    return *this;
}

const Sphere &Sphere::setRefractiveIndex(const double refractiveIndex)
{
    this->refractiveIndex = refractiveIndex;
    return *this;
}

const Intersection Sphere::intersect(const Ray &ray) const
{
    const Vector3 rayOrigin = ray.getOrigin();
    const Vector3 rayDirection = ray.getDirection();

    const Vector3 centerToOrigin = rayOrigin - center;

    const double distanceDot = rayDirection.dot(centerToOrigin);

    const double determinant = pow(distanceDot, 2) - centerToOrigin.norm2() + pow(radius, 2);

    if (determinant < 0)
    {
        return Intersection();
    }

    const double t1 = -distanceDot - sqrt(determinant);
    const double t2 = -distanceDot + sqrt(determinant);

    if (t2 < 0)
    {
        // Sphere is behind the ray
        return Intersection();
    }

    double distance = t1 < 0 ? t2 : t1;

    const Vector3 intersectionPoint = rayOrigin + rayDirection * distance;

    const Vector3 normal = (intersectionPoint - center).normalize();

    IntersectionBuilder intersectionBuilder;

    intersectionBuilder.setHit(true);
    intersectionBuilder.setPoint(intersectionPoint);
    intersectionBuilder.setNormal(normal);
    intersectionBuilder.setDistance(distance);

    if (mirror)
    {
        intersectionBuilder.setReflected(true);
        intersectionBuilder.setReflectedRay(ray.calculateReflectedRay(intersectionPoint, normal).addOffset());
    }
    else if (transparent)
    {

        // Analyze direction of normal compared to the incident ray
        int direction = normal.dot(rayDirection) > 0 ? 1 : -1;
        // FIXME: Here we assume that the ray is exiting into the air
        double n2 = normal.dot(rayDirection) > 0 ? 1 : refractiveIndex;

        const double n = ray.getRefractiveIndex() / n2;
        const double rayDotNormal = rayDirection.dot(normal);
        const Vector3 refractedTangent = (rayDirection - normal * rayDotNormal) * n;
        const double cosIncident = rayDirection.dot(normal);
        const double sin2Transmitted = n * n * (1.0 - cosIncident * cosIncident);

        if (sin2Transmitted > 1.0)
        {
            // Total internal reflection
            intersectionBuilder.setReflected(true);
            intersectionBuilder.setReflectedRay(ray.calculateReflectedRay(intersectionPoint, normal).addOffset());
        }
        else
        {
            const double cosTransmitted = sqrt(1.0 - sin2Transmitted);

            const Vector3 refractedNormal = normal * direction * cosTransmitted;

            const Vector3 refractedDirection = (refractedTangent + refractedNormal).normalize();

            Ray refractedRay(intersectionPoint, refractedDirection, n2);
            refractedRay.addOffset();

            intersectionBuilder.setRefracted(true);
            intersectionBuilder.setRefractedRay(refractedRay);
        }
    }
    else
    {
        intersectionBuilder.setAlbedo(color);
    }

    return intersectionBuilder.build();
}