#pragma once
#include <exception>
#include <iostream>
#include <optional>

#include "../utils/vector3.h"
#include "ray.h"

class Intersection
{
  private:
    bool hit;
    std::optional<Vector3> point;
    std::optional<Vector3> normal;
    double distance;
    std::optional<Vector3> albedo;
    bool reflected;
    std::optional<Ray> reflectedRay;

  public:
    Intersection();
    Intersection(const bool hit, const Vector3 &point, const Vector3 &normal, const double distance,
                 const Ray &reflectedRay);
    Intersection(const bool hit, const Vector3 &point, const Vector3 &normal, const double distance,
                 const Vector3 &albedo);
    const bool isHit() const;
    const Vector3 &getPoint() const;
    const Vector3 &getNormal() const;
    const double getDistance() const;
    const Vector3 &getAlbedo() const;
    const bool isReflected() const;
    const Ray &getReflectedRay() const;

    friend std::ostream &operator<<(std::ostream &os, const Intersection &intersection);
};

class IntersectionException : public std::exception
{
  protected:
    const Intersection &intersection;

  public:
    IntersectionException(const Intersection &intersection) : intersection(intersection){};
};

class UnsetIntersectionPointException : public IntersectionException
{
    using IntersectionException::IntersectionException;

  public:
    const char *what() const throw()
    {
        if (!intersection.isHit())
        {
            return "This intersection does not have a point set because it is not a hit";
        }
        return "This intersection does not have a point set";
    }
};

class UnsetIntersectionNormalException : public IntersectionException
{
    using IntersectionException::IntersectionException;

  public:
    const char *what() const throw()
    {
        if (!intersection.isHit())
        {
            return "This intersection does not have a point set because it is not a hit";
        }
        return "This intersection does not have a point set";
    }
};

class UnsetIntersectionAlbedoException : public IntersectionException
{
    using IntersectionException::IntersectionException;

  public:
    const char *what() const throw()
    {
        if (!intersection.isHit())
        {
            return "This intersection does not have an albedo set because it is not a hit";
        }
        return "This intersection does not have an albedo set";
    }
};
