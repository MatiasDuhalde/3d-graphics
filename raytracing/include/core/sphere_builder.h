#include "../utils/vector3.h"
#include "sphere.h"
#include <optional>

class SphereBuilder
{
  private:
    std::optional<Vector3> center = Sphere::Defaults::CENTER;
    std::optional<double> radius = Sphere::Defaults::RADIUS;
    Vector3 color = Sphere::Defaults::COLOR;
    bool mirror = Sphere::Defaults::MIRROR;
    bool transparent = Sphere::Defaults::TRANSPARENT;
    double refractiveIndex = Sphere::Defaults::REFRACTIVE_INDEX;

  public:
    SphereBuilder &setCenter(const Vector3 &center);
    SphereBuilder &setRadius(const double radius);
    SphereBuilder &setColor(const Vector3 &color);
    SphereBuilder &setMirror(const bool mirror);
    SphereBuilder &setTransparent(const bool transparent);
    SphereBuilder &setRefractiveIndex(const double refractiveIndex);
    Sphere build();
    SphereBuilder &reset();

    class Exception : public std::exception
    {
      private:
        std::string message;

      public:
        Exception(const std::string &message);
        const char *what() const noexcept override;
    };
};
