#pragma once
#include <optional>

#include "../utils/vector3.h"
#include "sphere.h"

class SphereBuilder
{
  private:
    std::optional<Vector3> center = Sphere::Defaults::CENTER;
    std::optional<double> radius = Sphere::Defaults::RADIUS;
    Vector3 color = Sphere::Defaults::COLOR;
    bool mirror = Sphere::Defaults::MIRROR;
    bool transparent = Sphere::Defaults::TRANSPARENT;
    double refractiveIndex = Sphere::Defaults::REFRACTIVE_INDEX;

  public:
    constexpr SphereBuilder &setCenter(const Vector3 &center);
    constexpr SphereBuilder &setRadius(const double radius);
    constexpr SphereBuilder &setColor(const Vector3 &color);
    constexpr SphereBuilder &setMirror(const bool mirror);
    constexpr SphereBuilder &setTransparent(const bool transparent);
    constexpr SphereBuilder &setRefractiveIndex(const double refractiveIndex);
    constexpr Sphere build() const;
    constexpr SphereBuilder &reset();

    class Exception : public std::exception
    {
      private:
        std::string message;

      public:
        Exception(const std::string &message);
        const char *what() const noexcept override;
    };
};

constexpr SphereBuilder &SphereBuilder::setCenter(const Vector3 &center)
{
    this->center = center;
    return *this;
}

constexpr SphereBuilder &SphereBuilder::setRadius(const double radius)
{
    this->radius = radius;
    return *this;
}

constexpr SphereBuilder &SphereBuilder::setColor(const Vector3 &color)
{
    this->color = color;
    return *this;
}

constexpr SphereBuilder &SphereBuilder::setMirror(const bool mirror)
{
    this->mirror = mirror;
    return *this;
}

constexpr SphereBuilder &SphereBuilder::setTransparent(const bool transparent)
{
    this->transparent = transparent;
    return *this;
}

constexpr SphereBuilder &SphereBuilder::setRefractiveIndex(const double refractiveIndex)
{
    this->refractiveIndex = refractiveIndex;
    return *this;
}

constexpr Sphere SphereBuilder::build() const
{
    if (!center.has_value())
        throw Exception("Center is not set");
    if (!radius.has_value())
        throw Exception("Radius is not set");

    Sphere sphere(center.value(), radius.value());
    sphere.setColor(color);
    sphere.setMirror(mirror);
    sphere.setTransparent(transparent);
    sphere.setRefractiveIndex(refractiveIndex);
    return sphere;
}

constexpr SphereBuilder &SphereBuilder::reset()
{
    center = Sphere::Defaults::CENTER;
    radius = Sphere::Defaults::RADIUS;
    color = Sphere::Defaults::COLOR;
    mirror = Sphere::Defaults::MIRROR;
    transparent = Sphere::Defaults::TRANSPARENT;
    refractiveIndex = Sphere::Defaults::REFRACTIVE_INDEX;
    return *this;
}

inline SphereBuilder::Exception::Exception(const std::string &message) : message(message)
{
}

inline const char *SphereBuilder::Exception::what() const noexcept
{
    return message.c_str();
}