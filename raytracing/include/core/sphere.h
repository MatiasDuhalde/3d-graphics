#pragma once
#include <optional>

#include "../utils/vector3.h"
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

    Sphere(const Vector3 &center, const double radius);

    const Vector3 &getCenter() const;
    const Sphere &setCenter(const Vector3 &center);
    const double getRadius() const;
    const Sphere &setRadius(const double radius);

    /**
     * @brief Set the color of the sphere.
     *
     * @param color A vector with values between 0 and 1, representing the RGB color of the sphere.
     */
    const Sphere &setColor(const Vector3 &color);

    /**
     * @brief Set whether the sphere should act as a mirror or not.
     *
     * @param mirror
     */
    const Sphere &setMirror(const bool mirror);

    /**
     * @brief Set whether the sphere should be transparent or not.
     *
     * @param transparent
     */
    const Sphere &setTransparent(const bool transparent);

    /**
     * @brief Set the refractive index of the sphere.
     *
     * @param refractiveIndex Value greater than 1.
     */
    const Sphere &setRefractiveIndex(const double refractiveIndex);

    /**
     * @brief Calculate an intersection between the sphere and the given ray.
     *
     * @param ray
     * @return const Intersection
     */
    virtual const Intersection intersect(const Ray &ray) const;
};
