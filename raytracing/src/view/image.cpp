
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "../../libs/stb_image_write.h"

#define STB_IMAGE_IMPLEMENTATION
#include "../../libs/stb_image.h"

#include "../../include/view/image.h"

#include "../../include/utils/constants.h"
#include "../../include/utils/vector3.h"

Image::Image(const int width, const int height, Camera &camera, Scene &scene)
    : width(width), height(height), camera(&camera), scene(&scene)
{
    Image(width, height);
}

Image::Image(const int width, const int height) : width(width), height(height)
{
    this->image = new unsigned char[width * height * colorChannels];
    this->camera = nullptr;
    this->scene = nullptr;
}

Image::~Image()
{
    delete[] this->image;
}

void Image::setCamera(Camera &camera)
{
    this->camera = &camera;
}

void Image::setScene(Scene &scene)
{
    this->scene = &scene;
}

void Image::draw() const
{
    if (camera == nullptr)
    {
        throw NoCameraSetException();
    }
    if (scene == nullptr)
    {
        throw NoSceneSetException();
    }

#pragma omp parallel for
    for (int i = 0; i < height; i++)
    {
        for (int j = 0; j < width; j++)
        {
            Ray ray = calculatePixelRay(i, j);
            Intersection intersection = scene->intersect(ray);
            renderPixel(i, j, intersection);
        }
    }
}

void Image::renderPixel(const int i, const int j, const Intersection &intersection) const
{
    const Vector3 lambertianShading = scene->calculateLambertianShading(intersection);

    image[(i * width + j) * 3 + 0] = std::min(255., std::max(0., pow(lambertianShading[0], GAMMA_CORRECTION))); // RED
    image[(i * width + j) * 3 + 1] = std::min(255., std::max(0., pow(lambertianShading[1], GAMMA_CORRECTION))); // GREEN
    image[(i * width + j) * 3 + 2] = std::min(255., std::max(0., pow(lambertianShading[2], GAMMA_CORRECTION))); // BLUE
}

Ray Image::calculatePixelRay(const int i, const int j) const
{
    const Vector3 cameraOrigin = camera->getOrigin();

    const Vector3 pixelPosition = calculatePixelPosition(i, j);

    const Vector3 rayDirection = (pixelPosition - cameraOrigin).normalize();

    return Ray(cameraOrigin, rayDirection);
}

Vector3 Image::calculatePixelPosition(const int i, const int j) const
{
    const Vector3 cameraOrigin = camera->getOrigin();

    return Vector3(cameraOrigin[0] + j + 0.5 - width / 2., cameraOrigin[1] - i - 0.5 + height / 2.,
                   cameraOrigin[2] - width / (2. * tan(camera->getFov() / 2.)));
}

void Image::save(const std::string filename) const
{
    stbi_write_png(filename.c_str(), width, height, colorChannels, &image[0], 0);
}