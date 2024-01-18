#pragma once
#include <string>

#include "camera.h"
#include "../core/ray.h"
#include "../core/scene.h"

class Image
{
private:
    const int width;
    const int height;
    unsigned char *image;
    const int colorChannels = 3;
    Camera *camera;
    Scene *scene;

    Ray calculatePixelRay(const int i, const int j) const;
    Vector3 calculatePixelPosition(const int i, const int j) const;
    void renderPixel(const int i, const int j, const Intersection &intersection) const;

public:
    Image(const int width, const int height);
    Image(const int width, const int height, Camera &camera, Scene &scene);
    ~Image();
    void setCamera(Camera &camera);
    void setScene(Scene &scene);
    void draw() const;
    void save(const std::string filename) const;
};

class NoCameraSetException : public std::exception
{
public:
    const char *what() const throw()
    {
        return "No camera has been set";
    }
};

class NoSceneSetException : public std::exception
{
public:
    const char *what() const throw()
    {
        return "No scene has been set";
    }
};
