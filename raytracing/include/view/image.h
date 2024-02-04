#pragma once
#include <string>

#include "../core/ray.h"
#include "../core/scene.h"
#include "camera.h"

/**
 * @brief Describes an image that can be rendered
 *
 */
class Image
{
  private:
    const int width;
    const int height;
    unsigned char *image;
    const int colorChannels = 3;
    Camera *camera = nullptr;
    Scene *scene = nullptr;

    const Ray calculatePixelRay(const int i, const int j) const;
    const Vector3 calculatePixelPosition(const int i, const int j) const;
    void renderPixel(const int i, const int j, const Intersection &intersection) const;

  public:
    Image(const int width, const int height);
    ~Image();
    Image &setCamera(Camera &camera);
    Image &setScene(Scene &scene);

    /**
     * @brief Calculate the pixels of the image
     *
     */
    void draw() const;

    /**
     * @brief Save the image to a file
     *
     * @param filename The name of the file to save the image to
     */
    void save(const std::string filename) const;

    class Exception : public std::exception
    {
      private:
        std::string message;

      public:
        Exception(const std::string &message);
        const char *what() const noexcept override;
    };
};
