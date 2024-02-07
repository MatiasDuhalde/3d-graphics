#pragma once
#include <cmath>
#include <optional>

#include "../utils/constants.h"
#include "../utils/vector3.h"
#include "./intersection_builder.h"
#include "intersectable_object.h"
#include "intersection.h"
#include "ray.h"

class Sphere : public IntersectableObject
{
  private:
    Vector3 center;
    double radius;

    Vector3 color = Defaults::COLOR;
    bool mirror = Defaults::MIRROR;
    bool transparent = Defaults::TRANSPARENT;
    double refractiveIndex = Defaults::REFRACTIVE_INDEX;

  public:
    struct Defaults
    {
        static constexpr std::optional<Vector3> CENTER = std::nullopt;
        static constexpr std::optional<double> RADIUS = std::nullopt;
        static constexpr Vector3 COLOR{1, 1, 1};
        static constexpr bool MIRROR = false;
        static constexpr bool TRANSPARENT = false;
        static constexpr double REFRACTIVE_INDEX = 1;
    };

    constexpr Sphere(const Vector3 &center, const double radius);

    const Vector3 &getCenter() const;
    constexpr Sphere &setCenter(const Vector3 &center);
    constexpr double getRadius() const;
    constexpr Sphere &setRadius(const double radius);

    /**
     * @brief Set the color of the sphere.
     *
     * @param color A vector with values between 0 and 1, representing the RGB color of the sphere.
     */
    constexpr Sphere &setColor(const Vector3 &color);

    /**
     * @brief Set whether the sphere should act as a mirror or not.
     *
     * @param mirror
     */
    constexpr Sphere &setMirror(const bool mirror);

    /**
     * @brief Set whether the sphere should be transparent or not.
     *
     * @param transparent
     */
    constexpr Sphere &setTransparent(const bool transparent);

    /**
     * @brief Set the refractive index of the sphere.
     *
     * @param refractiveIndex Value greater than 1.
     */
    constexpr Sphere &setRefractiveIndex(const double refractiveIndex);

    /**
     * @brief Calculate an intersection between the sphere and the given ray.
     *
     * @param ray
     * @return const Intersection
     */
    virtual const Intersection intersect(const Ray &ray) const;
};

constexpr Sphere::Sphere(const Vector3 &center, const double radius) : center(center), radius(radius)
{
}

inline const Vector3 &Sphere::getCenter() const
{
    return center;
}

constexpr Sphere &Sphere::setCenter(const Vector3 &center)
{
    this->center = center;
    return *this;
}

constexpr double Sphere::getRadius() const
{
    return radius;
}

constexpr Sphere &Sphere::setRadius(const double radius)
{
    this->radius = radius;
    return *this;
}

constexpr Sphere &Sphere::setColor(const Vector3 &color)
{
    this->color = color;
    return *this;
}

constexpr Sphere &Sphere::setMirror(const bool mirror)
{
    this->mirror = mirror;
    return *this;
}

constexpr Sphere &Sphere::setTransparent(const bool transparent)
{
    this->transparent = transparent;
    return *this;
}

constexpr Sphere &Sphere::setRefractiveIndex(const double refractiveIndex)
{
    this->refractiveIndex = refractiveIndex;
    return *this;
}

inline const Intersection Sphere::intersect(const Ray &ray) const
{
    const Vector3 rayOrigin = ray.getOrigin();
    const Vector3 rayDirection = ray.getDirection();

    const Vector3 centerToOrigin = rayOrigin - center;

    const double distanceDot = rayDirection.dot(centerToOrigin);

    const double determinant = pow(distanceDot, 2) - centerToOrigin.norm2() + pow(radius, 2);

    if (determinant < 0)
        return Intersection();

    const double t1 = -distanceDot - sqrt(determinant);
    const double t2 = -distanceDot + sqrt(determinant);

    // Sphere is behind the ray
    if (t2 < 0)
        return Intersection();

    double distance = t1 < 0 ? t2 : t1;

    const Vector3 intersectionPoint = rayOrigin + rayDirection * distance;

    const Vector3 normal = (intersectionPoint - center).normalize();

    IntersectionBuilder intersectionBuilder;

    intersectionBuilder.setHit(true);
    intersectionBuilder.setPoint(intersectionPoint);
    intersectionBuilder.setNormal(normal);
    intersectionBuilder.setDistance(distance);
    intersectionBuilder.setSourceRay(ray);

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

            const Ray refractedRay = Ray(intersectionPoint, refractedDirection, n2).addOffset();

            const Ray reflectedRay = ray.calculateReflectedRay(intersectionPoint, normal).addOffset();

            intersectionBuilder.setRefracted(true);
            intersectionBuilder.setRefractedRay(refractedRay);
            intersectionBuilder.setReflectedRay(reflectedRay);
        }
    }
    else
    {
        intersectionBuilder.setOpaque(true);
        intersectionBuilder.setAlbedo(color);
    }

    return intersectionBuilder.build();
}