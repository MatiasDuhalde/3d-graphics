#pragma once
#include <cmath>
#include <exception>
#include <optional>

#include "../utils/random.h"
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
    std::optional<Ray> sourceRay = Defaults::SOURCE_RAY;
    bool reflected = Defaults::REFLECTED;
    std::optional<Ray> reflectedRay = Defaults::REFLECTED_RAY;
    bool refracted = Defaults::REFRACTED;
    std::optional<Ray> refractedRay = Defaults::REFRACTED_RAY;

  public:
    struct Defaults
    {
        static constexpr bool HIT = false;
        static constexpr std::optional<const Vector3> POINT = std::nullopt;
        static constexpr std::optional<const Vector3> NORMAL = std::nullopt;
        static constexpr double DISTANCE = INFINITY;
        static constexpr bool OPAQUE = false;
        static constexpr std::optional<const Vector3> ALBEDO = std::nullopt;
        static constexpr std::optional<const Ray> SOURCE_RAY = std::nullopt;
        static constexpr bool REFLECTED = false;
        static constexpr std::optional<const Ray> REFLECTED_RAY = std::nullopt;
        static constexpr bool REFRACTED = false;
        static constexpr std::optional<const Ray> REFRACTED_RAY = std::nullopt;
    };

    constexpr bool isHit() const;
    constexpr Intersection &setHit(const bool hit);
    const Vector3 &getPoint() const;
    constexpr Intersection &setPoint(const Vector3 &point);
    const Vector3 &getNormal() const;
    constexpr Intersection &setNormal(const Vector3 &normal);
    constexpr double getDistance() const;
    constexpr Intersection &setDistance(const double distance);
    constexpr bool isOpaque() const;
    constexpr Intersection &setOpaque(const bool opaque);
    const Vector3 &getAlbedo() const;
    constexpr Intersection &setAlbedo(const Vector3 &albedo);
    const Ray &getSourceRay() const;
    constexpr Intersection &setSourceRay(const Ray &sourceRay);
    constexpr bool isReflected() const;
    constexpr Intersection &setReflected(const bool reflected);
    const Ray &getReflectedRay() const;
    constexpr Intersection &setReflectedRay(const Ray &reflectedRay);
    constexpr bool isRefracted() const;
    constexpr Intersection &setRefracted(const bool refracted);
    const Ray &getRefractedRay() const;
    constexpr Intersection &setRefractedRay(const Ray &refractedRay);
    constexpr double getReflectionCoefficient() const;

    const Ray getRandomNormalHemisphereRay() const;

    class Exception : public std::exception
    {
      private:
        std::string message;

      public:
        Exception(const std::string &message);
        const char *what() const noexcept override;
    };
};

constexpr bool Intersection::isHit() const
{
    return hit;
}

constexpr Intersection &Intersection::setHit(const bool hit)
{
    this->hit = hit;
    return *this;
}

inline const Vector3 &Intersection::getPoint() const
{
    if (!point.has_value())
        throw Exception("Point is not set");
    return point.value();
}

constexpr Intersection &Intersection::setPoint(const Vector3 &point)
{
    this->point = point;
    return *this;
}

inline const Vector3 &Intersection::getNormal() const
{
    if (!normal.has_value())
        throw Exception("Normal is not set");
    return normal.value();
}

constexpr Intersection &Intersection::setNormal(const Vector3 &normal)
{
    this->normal = normal;
    return *this;
}

constexpr double Intersection::getDistance() const
{
    return distance;
}

constexpr Intersection &Intersection::setDistance(const double distance)
{
    this->distance = distance;
    return *this;
}

constexpr bool Intersection::isOpaque() const
{
    return opaque;
}

constexpr Intersection &Intersection::setOpaque(const bool opaque)
{
    this->opaque = opaque;
    return *this;
}

inline const Vector3 &Intersection::getAlbedo() const
{
    if (!albedo.has_value())
        throw Exception("Albedo is not set");
    return albedo.value();
}

constexpr Intersection &Intersection::setAlbedo(const Vector3 &albedo)
{
    this->albedo = albedo;
    return *this;
}

inline const Ray &Intersection::getSourceRay() const
{
    if (!sourceRay.has_value())
        throw Exception("Source ray is not set");
    return sourceRay.value();
}

constexpr Intersection &Intersection::setSourceRay(const Ray &sourceRay)
{
    this->sourceRay = sourceRay;
    return *this;
}

constexpr bool Intersection::isReflected() const
{
    return reflected;
}

constexpr Intersection &Intersection::setReflected(const bool reflected)
{
    this->reflected = reflected;
    return *this;
}

inline const Ray &Intersection::getReflectedRay() const
{
    if (!reflectedRay.has_value())
        throw Exception("Reflected ray is not set");
    return reflectedRay.value();
}

constexpr Intersection &Intersection::setReflectedRay(const Ray &reflectedRay)
{
    this->reflectedRay = reflectedRay;
    return *this;
}

constexpr bool Intersection::isRefracted() const
{
    return refracted;
}

constexpr Intersection &Intersection::setRefracted(const bool refracted)
{
    this->refracted = refracted;
    return *this;
}

inline const Ray &Intersection::getRefractedRay() const
{
    if (!refractedRay.has_value())
        throw Exception("Refracted ray is not set");
    return refractedRay.value();
}

constexpr Intersection &Intersection::setRefractedRay(const Ray &refractedRay)
{
    this->refractedRay = refractedRay;
    return *this;
}

constexpr double Intersection::getReflectionCoefficient() const
{
    const double n1 = getReflectedRay().getRefractiveIndex();
    const double n2 = getRefractedRay().getRefractiveIndex();

    return pow((n1 - n2) / (n1 + n2), 2);
}

inline const Ray Intersection::getRandomNormalHemisphereRay() const
{
    const Vector3 &normal = getNormal();
    const Vector3 &point = getPoint();
    const Ray &sourceRay = getSourceRay();
    const int orientation = normal.dot(sourceRay.getDirection()) > 0 ? -1 : 1;

    const double r1 = 2 * M_PI * randomDistribution(randomEngine);
    const double r2 = randomDistribution(randomEngine);
    const double k = sqrt(1 - r2);

    const double x = cos(r1) * k;
    const double y = sin(r1) * k;
    const double z = sqrt(r2);

    // FIXME: Edge case when normal is (0, 0, 1)
    const Vector3 t1 = normal.cross(Vector3(0, 0, 1)).normalize();
    const Vector3 t2 = normal.cross(t1).normalize();

    const Vector3 randomDirection = (t1 * x + t2 * y + normal * z).normalize();

    return Ray(point, randomDirection * orientation).addOffset();
}

inline Intersection::Exception::Exception(const std::string &message) : message(message)
{
}

inline const char *Intersection::Exception::what() const noexcept
{
    return message.c_str();
}
