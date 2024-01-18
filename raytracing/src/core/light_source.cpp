#include "../../include/core/light_source.h"

LightSource::LightSource(const Vector3 &position, const double intensity) : position(position), intensity(intensity)
{
}

const Vector3 &LightSource::getPosition() const
{
    return this->position;
}

const double LightSource::getIntensity() const
{
    return this->intensity;
}

void LightSource::setPosition(const Vector3 &position)
{
    this->position = position;
}

void LightSource::setIntensity(const double intensity)
{
    this->intensity = intensity;
}

std::ostream &operator<<(std::ostream &os, const LightSource &lightSource)
{
    os << "LightSource(position: " << lightSource.position << ", intensity: " << lightSource.intensity << ")";
    return os;
}
