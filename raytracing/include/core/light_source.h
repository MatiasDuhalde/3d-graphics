#pragma once
#include <iostream>

#include "../utils/vector3.h"

class LightSource
{
  private:
    Vector3 position;
    double intensity;

  public:
    LightSource(const Vector3 &position, const double intensity);

    const Vector3 &getPosition() const;
    const LightSource &setPosition(const Vector3 &position);

    const double getIntensity() const;

    /**
     * @brief Set the intensity value
     *
     * @param intensity Light luminosity in watts
     */
    const LightSource &setIntensity(const double intensity);

    friend std::ostream &operator<<(std::ostream &os, const LightSource &lightSource);
};
