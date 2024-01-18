#include "../../include/view/camera.h"

Camera::Camera(const Vector3 &origin, double fov) : origin(origin), fov(fov)
{
}

const Vector3 &Camera::getOrigin() const
{
    return this->origin;
}

const double Camera::getFov() const
{
    return this->fov;
}
