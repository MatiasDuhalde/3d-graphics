#pragma once
#include <iostream>

#include "../utils/constants.h"
#include "../utils/vector3.h"

/**
 * @brief Describes a ray with an origin and a direction
 *
 */
class Ray
{
  private:
    static constexpr double DEFAULT_REFRACTIVE_INDEX = 1;

    Vector3 origin;
    Vector3 direction;

    double refractiveIndex = DEFAULT_REFRACTIVE_INDEX;

  public:
    /**
     * @brief Construct a new Ray object
     *
     * @param origin
     * @param direction Normalized direction vector
     */
    constexpr Ray(const Vector3 &origin, const Vector3 &direction);

    /**
     * @brief Construct a new Ray object
     *
     * @param origin
     * @param direction Normalized direction vector
     * @param refractiveIndex Refractive index of the medium the ray is in
     */
    constexpr Ray(const Vector3 &origin, const Vector3 &direction, const double refractiveIndex);

    const Vector3 &getOrigin() const;
    const Vector3 &getDirection() const;
    constexpr double getRefractiveIndex() const;

    /**
     * @brief Calculate the reflected ray of this ray
     *
     * @param intersectionPoint
     * @param normal
     * @return const Ray
     */
    constexpr Ray calculateReflectedRay(const Vector3 &intersectionPoint, const Vector3 &normal) const;

    /**
     * @brief Add a small offset to the origin of the ray to prevent self-intersection
     *
     * @return Ray&
     */
    constexpr Ray &addOffset();

    friend constexpr std::ostream &operator<<(std::ostream &os, const Ray &ray);
};

constexpr Ray::Ray(const Vector3 &origin, const Vector3 &direction) : origin(origin), direction(direction)
{
}

constexpr Ray::Ray(const Vector3 &origin, const Vector3 &direction, const double refractiveIndex)
    : origin(origin), direction(direction), refractiveIndex(refractiveIndex)
{
}

inline const Vector3 &Ray::getOrigin() const
{
    return origin;
}

inline const Vector3 &Ray::getDirection() const
{
    return direction;
}

constexpr double Ray::getRefractiveIndex() const
{
    return refractiveIndex;
}

constexpr Ray Ray::calculateReflectedRay(const Vector3 &intersectionPoint, const Vector3 &normal) const
{
    return Ray(intersectionPoint, direction - normal * 2 * direction.dot(normal), refractiveIndex);
}

constexpr Ray &Ray::addOffset()
{
    origin += direction * RAY_OFFSET_EPSILON;
    return *this;
}

constexpr std::ostream &operator<<(std::ostream &os, const Ray &ray)
{
    os << "Ray(origin: " << ray.origin << ", direction: " << ray.direction << ")";
    return os;
}
