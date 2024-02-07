
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "../../libs/stb_image_write.h"

#define STB_IMAGE_IMPLEMENTATION
#include "../../libs/stb_image.h"

#include "../../include/utils/constants.h"
#include "../../include/utils/random.h"
#include "../../include/utils/vector3.h"
#include "../../include/view/image.h"

Image::Image(const int width, const int height) : width(width), height(height)
{
    image = new unsigned char[width * height * colorChannels];
}

Image::~Image()
{
    delete[] image;
}

Image &Image::setCamera(Camera &camera)
{
    this->camera = &camera;
    return *this;
}

Image &Image::setScene(Scene &scene)
{
    this->scene = &scene;
    return *this;
}

void Image::draw() const
{
    if (camera == nullptr)
        throw Exception("Camera is not set");
    if (scene == nullptr)
        throw Exception("Scene is not set");

#pragma omp parallel for schedule(dynamic, 1)
    for (int i = 0; i < height; i++)
    {
        for (int j = 0; j < width; j++)
        {
            renderPixel(i, j);
        }
    }
}

void Image::renderPixel(const int i, const int j) const
{
    const int repetitions = ENABLE_ANTIALIASING ? ANTIALIASING_RAYS : 1;

    Vector3 color = Vector3(0, 0, 0);
    for (int k = 0; k < repetitions; k++)
    {
        const Ray ray = ENABLE_ANTIALIASING ? calculateRandomPixelRay(i, j) : calculatePixelRay(i, j);
        const Intersection intersection = scene->intersect(ray);
        color += scene->calculateColor(intersection, ENABLE_ANTIALIASING);
    }
    color /= repetitions;

    image[(i * width + j) * 3 + 0] = std::min(255., std::max(0., pow(color[0], GAMMA_CORRECTION))); // RED
    image[(i * width + j) * 3 + 1] = std::min(255., std::max(0., pow(color[1], GAMMA_CORRECTION))); // GREEN
    image[(i * width + j) * 3 + 2] = std::min(255., std::max(0., pow(color[2], GAMMA_CORRECTION))); // BLUE
}

const Ray Image::calculatePixelRay(const int i, const int j) const
{
    const Vector3 cameraOrigin = camera->getOrigin();

    const Vector3 pixelPosition = calculatePixelPosition(i, j);

    const Vector3 rayDirection = (pixelPosition - cameraOrigin).normalize();

    return Ray(cameraOrigin, rayDirection);
}

const Ray Image::calculateRandomPixelRay(const int i, const int j) const
{
    const Vector3 cameraOrigin = camera->getOrigin();

    const Vector3 randomOffset = boxMuller(0.25);

    const Vector3 randomPixelPosition = calculatePixelPosition(i, j) + randomOffset;

    const Vector3 rayDirection = (randomPixelPosition - cameraOrigin).normalize();

    return Ray(cameraOrigin, rayDirection);
}

const Vector3 Image::calculatePixelPosition(const int i, const int j) const
{
    const Vector3 cameraOrigin = camera->getOrigin();

    return Vector3(cameraOrigin[0] + j + 0.5 - width / 2., cameraOrigin[1] - i - 0.5 + height / 2.,
                   cameraOrigin[2] - width / (2. * tan(camera->getFov() / 2.)));
}

void Image::save(const std::string filename) const
{
    stbi_write_png(filename.c_str(), width, height, colorChannels, &image[0], 0);
}

Image::Exception::Exception(const std::string &message) : message(message)
{
}

const char *Image::Exception::what() const noexcept
{
    return message.c_str();
}