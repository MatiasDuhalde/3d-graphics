#pragma once
#include <cmath>
#include <exception>
#include <iostream>
#include <optional>

#include "../utils/vector3.h"
#include "ray.h"

class Intersection
{
  private:
    bool hit = false;
    std::optional<Vector3> point = Defaults::POINT;
    std::optional<Vector3> normal = Defaults::NORMAL;
    double distance = Defaults::DISTANCE;
    bool opaque = Defaults::OPAQUE;
    std::optional<Vector3> albedo = Defaults::ALBEDO;
    bool reflected = Defaults::REFLECTED;
    std::optional<Ray> reflectedRay = Defaults::REFLECTED_RAY;
    bool refracted = Defaults::REFRACTED;
    std::optional<Ray> refractedRay = Defaults::REFRACTED_RAY;

  public:
    struct Defaults
    {
        static constexpr bool HIT = false;
        static constexpr std::optional<Vector3> POINT = std::nullopt;
        static constexpr std::optional<Vector3> NORMAL = std::nullopt;
        static constexpr double DISTANCE = INFINITY;
        static constexpr bool OPAQUE = false;
        static constexpr std::optional<Vector3> ALBEDO = std::nullopt;
        static constexpr bool REFLECTED = false;
        static constexpr std::optional<Ray> REFLECTED_RAY = std::nullopt;
        static constexpr bool REFRACTED = false;
        static constexpr std::optional<Ray> REFRACTED_RAY = std::nullopt;
    };

    const bool isHit() const;
    const Intersection &setHit(const bool hit);
    const Vector3 &getPoint() const;
    const Intersection &setPoint(const Vector3 &point);
    const Vector3 &getNormal() const;
    const Intersection &setNormal(const Vector3 &normal);
    const double getDistance() const;
    const Intersection &setDistance(const double distance);
    const bool isOpaque() const;
    const Intersection &setOpaque(const bool opaque);
    const Vector3 &getAlbedo() const;
    const Intersection &setAlbedo(const Vector3 &albedo);
    const bool isReflected() const;
    const Intersection &setReflected(const bool reflected);
    const Ray &getReflectedRay() const;
    const Intersection &setReflectedRay(const Ray &reflectedRay);
    const bool isRefracted() const;
    const Intersection &setRefracted(const bool refracted);
    const Ray &getRefractedRay() const;
    const Intersection &setRefractedRay(const Ray &refractedRay);
    const double getReflectionCoefficient() const;

    friend std::ostream &operator<<(std::ostream &os, const Intersection &intersection);

    class Exception : public std::exception
    {
      private:
        std::string message;

      public:
        Exception(const std::string &message);
        const char *what() const noexcept override;
    };
};
