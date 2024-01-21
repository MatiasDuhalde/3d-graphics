#include "../../include/core/sphere_builder.h"

SphereBuilder &SphereBuilder::setCenter(const Vector3 &center)
{
    this->center = center;
    return *this;
}

SphereBuilder &SphereBuilder::setRadius(const double radius)
{
    this->radius = radius;
    return *this;
}

SphereBuilder &SphereBuilder::setColor(const Vector3 &color)
{
    this->color = color;
    return *this;
}

SphereBuilder &SphereBuilder::setMirror(const bool mirror)
{
    this->mirror = mirror;
    return *this;
}

SphereBuilder &SphereBuilder::setTransparent(const bool transparent)
{
    this->transparent = transparent;
    return *this;
}

SphereBuilder &SphereBuilder::setRefractiveIndex(const double refractiveIndex)
{
    this->refractiveIndex = refractiveIndex;
    return *this;
}

Sphere SphereBuilder::build()
{
    if (!center.has_value())
    {
        throw Exception("Center is not set");
    }
    if (!radius.has_value())
    {
        throw Exception("Radius is not set");
    }

    Sphere sphere(center.value(), radius.value());
    sphere.setColor(color);
    sphere.setMirror(mirror);
    sphere.setTransparent(transparent);
    sphere.setRefractiveIndex(refractiveIndex);
    return sphere;
}

SphereBuilder &SphereBuilder::reset()
{
    center = Sphere::Defaults::CENTER;
    radius = Sphere::Defaults::RADIUS;
    color = Sphere::Defaults::COLOR;
    mirror = Sphere::Defaults::MIRROR;
    transparent = Sphere::Defaults::TRANSPARENT;
    refractiveIndex = Sphere::Defaults::REFRACTIVE_INDEX;
    return *this;
}

SphereBuilder::Exception::Exception(const std::string &message) : message(message)
{
}

const char *SphereBuilder::Exception::what() const noexcept
{
    return message.c_str();
}