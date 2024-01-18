#define _CRT_SECURE_NO_WARNINGS 1
#include <vector>
#include <cmath>

#include "include/utils/vector3.h"
#include "include/view/camera.h"
#include "include/view/image.h"

#include "include/core/scene.h"
#include "include/core/sphere.h"
#include "include/core/light_source.h"

int main()
{
    const int imageWidth = 512;
    const int imageHeight = 512;

    Image image(imageWidth, imageHeight);

    Scene scene;

    Sphere sphere(Vector3(0, 0, 0), 10, Vector3(1, 0, 0));
    scene.addIntersectableObject(sphere);

    LightSource lightSource(Vector3(-10, 20, 40), 1E7);
    scene.setLightSource(lightSource);

    Camera camera(Vector3(0, 0, 55), 60 * M_PI / 180.);
    image.setCamera(camera);

    image.setScene(scene);

    image.draw();
    image.save("image.png");

    return 0;
}