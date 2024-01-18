#pragma once
#include "../utils/vector3.h"

class Camera
{
  private:
    Vector3 origin;
    // field of view in radians
    double fov;

  public:
    Camera(const Vector3 &origin, const double fov);

    const Vector3 &getOrigin() const;
    const double getFov() const;
};
