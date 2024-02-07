#pragma once
#include <cmath>
#include <iostream>

class Vector3
{
  private:
    double coord[3];

  public:
    explicit constexpr Vector3(const double x, const double y, const double z) : coord{x, y, z} {};

    constexpr double operator[](const int i) const;

    constexpr Vector3 operator+(const Vector3 &other) const;
    constexpr Vector3 operator-(const Vector3 &other) const;
    constexpr Vector3 operator*(const double a) const;
    constexpr Vector3 operator*(const Vector3 &other) const;
    constexpr Vector3 operator/(const double a) const;
    constexpr Vector3 &operator+=(const Vector3 &other);
    constexpr Vector3 &operator*=(const double a);
    constexpr Vector3 &operator/=(const double a);

    constexpr double dot(const Vector3 &other) const;

    constexpr Vector3 cross(const Vector3 &other) const;

    constexpr double norm() const;

    constexpr double norm2() const;

    constexpr Vector3 normalize() const;

    friend constexpr std::ostream &operator<<(std::ostream &os, const Vector3 &vector3);
};

constexpr double Vector3::operator[](const int i) const
{
    return coord[i];
}

constexpr Vector3 Vector3::operator+(const Vector3 &other) const
{
    return Vector3(coord[0] + other[0], coord[1] + other[1], coord[2] + other[2]);
}

constexpr Vector3 Vector3::operator-(const Vector3 &other) const
{
    return Vector3(coord[0] - other[0], coord[1] - other[1], coord[2] - other[2]);
}

constexpr Vector3 Vector3::operator*(const double a) const
{
    return Vector3(a * coord[0], a * coord[1], a * coord[2]);
}

constexpr Vector3 Vector3::operator*(const Vector3 &other) const
{
    return Vector3(coord[0] * other[0], coord[1] * other[1], coord[2] * other[2]);
}

constexpr Vector3 Vector3::operator/(const double a) const
{
    return Vector3(coord[0] / a, coord[1] / a, coord[2] / a);
}

constexpr Vector3 &Vector3::operator+=(const Vector3 &other)
{
    coord[0] += other[0];
    coord[1] += other[1];
    coord[2] += other[2];
    return *this;
}

constexpr Vector3 &Vector3::operator*=(const double a)
{
    coord[0] *= a;
    coord[1] *= a;
    coord[2] *= a;
    return *this;
}

constexpr Vector3 &Vector3::operator/=(const double a)
{
    coord[0] /= a;
    coord[1] /= a;
    coord[2] /= a;
    return *this;
}

constexpr double Vector3::dot(const Vector3 &other) const
{
    return coord[0] * other[0] + coord[1] * other[1] + coord[2] * other[2];
}

constexpr Vector3 Vector3::cross(const Vector3 &other) const
{
    return Vector3(coord[1] * other[2] - coord[2] * other[1], coord[2] * other[0] - coord[0] * other[2],
                   coord[0] * other[1] - coord[1] * other[0]);
}

constexpr double Vector3::norm() const
{
    return sqrt(norm2());
}

constexpr double Vector3::norm2() const
{
    return pow(coord[0], 2) + pow(coord[1], 2) + pow(coord[2], 2);
}

constexpr Vector3 Vector3::normalize() const
{
    return *this / norm();
}

constexpr std::ostream &operator<<(std::ostream &os, const Vector3 &vector3)
{
    os << "Vector3(" << vector3.coord[0] << ", " << vector3.coord[1] << ", " << vector3.coord[2] << ")";
    return os;
}
