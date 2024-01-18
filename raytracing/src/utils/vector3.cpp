#include "../../include/utils/vector3.h"
#include <cmath>

Vector3::Vector3(double x, double y, double z)
{
    coord[0] = x;
    coord[1] = y;
    coord[2] = z;
}

const double Vector3::getX() const
{
    return coord[0];
}

const double Vector3::getY() const
{
    return coord[1];
}

const double Vector3::getZ() const
{
    return coord[2];
}

double Vector3::operator[](const int i) const
{
    return coord[i];
}

Vector3 Vector3::operator+(const Vector3 &other) const
{
    return Vector3(coord[0] + other[0], coord[1] + other[1], coord[2] + other[2]);
}

Vector3 Vector3::operator-(const Vector3 &other) const
{
    return Vector3(coord[0] - other[0], coord[1] - other[1], coord[2] - other[2]);
}

Vector3 Vector3::operator*(double a) const
{
    return Vector3(a * coord[0], a * coord[1], a * coord[2]);
}

Vector3 Vector3::operator/(double a) const
{
    return Vector3(coord[0] / a, coord[1] / a, coord[2] / a);
}

Vector3 Vector3::operator+=(const Vector3 &other)
{
    coord[0] += other[0];
    coord[1] += other[1];
    coord[2] += other[2];
    return *this;
}

// dot product
const double Vector3::dot(const Vector3 &other) const
{
    return coord[0] * other[0] + coord[1] * other[1] + coord[2] * other[2];
}

const double Vector3::norm() const
{
    return sqrt(norm2());
}

const double Vector3::norm2() const
{
    return pow(coord[0], 2) + pow(coord[1], 2) + pow(coord[2], 2);
}

Vector3 Vector3::normalize() const
{
    double n = norm();
    return *this / n;
}

std::ostream &operator<<(std::ostream &os, const Vector3 &vector3)
{
    os << "Vector3(" << vector3.coord[0] << ", " << vector3.coord[1] << ", " << vector3.coord[2] << ")";
    return os;
}