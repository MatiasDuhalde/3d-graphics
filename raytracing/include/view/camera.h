#pragma once
#include "../utils/vector3.h"

/**
 * @brief Describes the point of view of the scene
 *
 */
class Camera
{
  private:
    const Vector3 origin;
    const double fov;

  public:
    /**
     * @brief Construct a new Camera object
     *
     * @param origin
     * @param fov The field of view of the camera in radians. 0 < fov < pi.
     */
    constexpr Camera(const Vector3 &origin, const double fov);

    const Vector3 &getOrigin() const;
    constexpr double getFov() const;
};

constexpr Camera::Camera(const Vector3 &origin, const double fov) : origin(origin), fov(fov)
{
}

inline const Vector3 &Camera::getOrigin() const
{
    return origin;
}

constexpr double Camera::getFov() const
{
    return fov;
}