#pragma once
#include <iostream>

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
    constexpr Ray(const Vector3 &origin, const Vector3 &direction) : origin(origin), direction(direction)
    {
    }

    /**
     * @brief Construct a new Ray object
     *
     * @param origin
     * @param direction Normalized direction vector
     * @param refractiveIndex Refractive index of the medium the ray is in
     */
    Ray(const Vector3 &origin, const Vector3 &direction, const double refractiveIndex)
        : origin(origin), direction(direction), refractiveIndex(refractiveIndex)
    {
    }

    const Vector3 &getOrigin() const;
    const Vector3 &getDirection() const;
    const double getRefractiveIndex() const;

    /**
     * @brief Calculate the reflected ray of this ray
     *
     * @param intersectionPoint
     * @param normal
     * @return const Ray
     */
    Ray calculateReflectedRay(const Vector3 &intersectionPoint, const Vector3 &normal) const;

    /**
     * @brief Add a small offset to the origin of the ray to prevent self-intersection
     *
     * @return Ray&
     */
    Ray &addOffset();

    friend std::ostream &operator<<(std::ostream &os, const Ray &ray);
};
