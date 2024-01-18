#pragma once
#include <iostream>

class Vector3
{
private:
    double coord[3];

public:
    explicit Vector3(double x, double y, double z);

    const double getX() const;

    const double getY() const;

    const double getZ() const;

    double operator[](const int i) const;

    Vector3 operator+(const Vector3 &other) const;

    Vector3 operator-(const Vector3 &other) const;

    Vector3 operator*(double a) const;

    Vector3 operator+=(const Vector3 &other);

    const double dot(const Vector3 &other) const;

    const double norm2() const;

    void normalize();

    friend std::ostream &operator<<(std::ostream &os, const Vector3 &vector3);
};