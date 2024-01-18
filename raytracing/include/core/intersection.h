#pragma once
#include <iostream>
#include <optional>

#include "../utils/vector3.h"

class Intersection
{
  private:
    bool hit;
    std::optional<Vector3> point;
    std::optional<Vector3> normal;
    double distance;
    std::optional<Vector3> albedo;

  public:
    Intersection();
    Intersection(const bool hit, const Vector3 &point, const Vector3 &normal, const double distance,
                 const Vector3 &albedo);
    const bool isHit() const;
    const Vector3 &getPoint() const;
    const Vector3 &getNormal() const;
    const double getDistance() const;
    const Vector3 &getAlbedo() const;

    friend std::ostream &operator<<(std::ostream &os, const Intersection &intersection);
};

class UnsetIntersectionException : public std::exception
{
  public:
    const char *what() const throw()
    {
        return "This intersection does not intersect with anything";
    }
};
