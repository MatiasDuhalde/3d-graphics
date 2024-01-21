#include "../../include/core/light_source.h"

LightSource::LightSource(const Vector3 &position, const double intensity) : position(position), intensity(intensity)
{
}

const Vector3 &LightSource::getPosition() const
{
    return position;
}

const LightSource &LightSource::setPosition(const Vector3 &position)
{
    this->position = position;
    return *this;
}

const double LightSource::getIntensity() const
{
    return intensity;
}

const LightSource &LightSource::setIntensity(const double intensity)
{
    this->intensity = intensity;
    return *this;
}

std::ostream &operator<<(std::ostream &os, const LightSource &lightSource)
{
    os << "LightSource(position: " << lightSource.position << ", intensity: " << lightSource.intensity << ")";
    return os;
}
