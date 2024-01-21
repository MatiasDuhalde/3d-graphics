#pragma once
#include <iostream>

class Vector3
{
  private:
    double coord[3];

  public:
    explicit constexpr Vector3(const double x, const double y, const double z) : coord{x, y, z} {};

    const double getX() const;

    const double getY() const;

    const double getZ() const;

    double operator[](const int i) const;

    Vector3 operator+(const Vector3 &other) const;
    Vector3 operator-(const Vector3 &other) const;
    Vector3 operator*(double a) const;
    Vector3 operator/(double a) const;
    Vector3 operator+=(const Vector3 &other);

    const double dot(const Vector3 &other) const;

    const double norm() const;

    const double norm2() const;

    Vector3 normalize() const;

    friend std::ostream &operator<<(std::ostream &os, const Vector3 &vector3);
};