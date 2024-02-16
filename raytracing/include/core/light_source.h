#pragma once
#include <iostream>

#include "../utils/vector3.h"

class LightSource
{
  private:
    Vector3 position;
    double intensity;

  public:
    constexpr LightSource(const Vector3 &position, const double intensity);

    const Vector3 &getPosition() const;
    constexpr LightSource &setPosition(const Vector3 &position);

    constexpr double getIntensity() const;

    /**
     * @brief Set the intensity value
     *
     * @param intensity Light luminosity in watts
     */
    constexpr LightSource &setIntensity(const double intensity);

    friend constexpr std::ostream &operator<<(std::ostream &os, const LightSource &lightSource);
};

constexpr LightSource::LightSource(const Vector3 &position, const double intensity)
    : position(position), intensity(intensity)
{
}

inline const Vector3 &LightSource::getPosition() const
{
    return position;
}

constexpr LightSource &LightSource::setPosition(const Vector3 &position)
{
    this->position = position;
    return *this;
}

constexpr double LightSource::getIntensity() const
{
    return intensity;
}

constexpr LightSource &LightSource::setIntensity(const double intensity)
{
    this->intensity = intensity;
    return *this;
}

constexpr std::ostream &operator<<(std::ostream &os, const LightSource &lightSource)
{
    os << "LightSource(position: " << lightSource.position << ", intensity: " << lightSource.intensity << ")";
    return os;
}
