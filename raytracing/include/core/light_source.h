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
    const double getIntensity() const;
    void setPosition(const Vector3 &position);
    void setIntensity(const double intensity);
    friend std::ostream &operator<<(std::ostream &os, const LightSource &lightSource);
};
