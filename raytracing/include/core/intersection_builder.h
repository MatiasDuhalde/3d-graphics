#include "intersection.h"

class IntersectionBuilder
{
  private:
    bool hit = Intersection::Defaults::HIT;
    std::optional<Vector3> point = Intersection::Defaults::POINT;
    std::optional<Vector3> normal = Intersection::Defaults::NORMAL;
    double distance = Intersection::Defaults::DISTANCE;
    bool opaque = Intersection::Defaults::OPAQUE;
    std::optional<Vector3> albedo = Intersection::Defaults::ALBEDO;
    std::optional<Ray> sourceRay = Intersection::Defaults::SOURCE_RAY;
    bool reflected = Intersection::Defaults::REFLECTED;
    std::optional<Ray> reflectedRay = Intersection::Defaults::REFLECTED_RAY;
    bool refracted = Intersection::Defaults::REFRACTED;
    std::optional<Ray> refractedRay = Intersection::Defaults::REFRACTED_RAY;

  public:
    IntersectionBuilder &setHit(bool hit);
    IntersectionBuilder &setPoint(const Vector3 &point);
    IntersectionBuilder &setNormal(const Vector3 &normal);
    IntersectionBuilder &setDistance(double distance);
    IntersectionBuilder &setOpaque(bool opaque);
    IntersectionBuilder &setAlbedo(const Vector3 &albedo);
    IntersectionBuilder &setSourceRay(const Ray &sourceRay);
    IntersectionBuilder &setReflected(bool reflected);
    IntersectionBuilder &setReflectedRay(const Ray &reflectedRay);
    IntersectionBuilder &setRefracted(bool refracted);
    IntersectionBuilder &setRefractedRay(const Ray &refractedRay);
    Intersection build() const;
};
