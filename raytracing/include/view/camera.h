#pragma once
#include "../utils/vector3.h"

/**
 * @brief Describes the point of view of the scene
 *
 */
class Camera
{
  private:
    Vector3 origin;
    double fov;

  public:
    /**
     * @brief Construct a new Camera object
     *
     * @param origin
     * @param fov The field of view of the camera in radians. 0 < fov < pi.
     */
    Camera(const Vector3 &origin, const double fov);

    const Vector3 &getOrigin() const;
    const double getFov() const;
};
