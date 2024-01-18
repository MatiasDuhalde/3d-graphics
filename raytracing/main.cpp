#define _CRT_SECURE_NO_WARNINGS 1
#include <cmath>
#include <vector>

#include "include/utils/vector3.h"
#include "include/view/camera.h"
#include "include/view/image.h"

#include "include/core/light_source.h"
#include "include/core/scene.h"
#include "include/core/sphere.h"

int main()
{
    const int imageWidth = 512;
    const int imageHeight = 512;

    Image image(imageWidth, imageHeight);

    Scene scene;

    Sphere smallSphere(Vector3(0, 0, 0), 10, Vector3(1, 0, 0));

    // Sphere sphere1(Vector3(0, 1000, 0), 940, Vector3(1, 0, 0));
    // Sphere sphere2(Vector3(0, 0, 1000), 940, Vector3(0, 1, 0));
    // Sphere sphere3(Vector3(0, -1000, 0), 990, Vector3(0, 0, 1));
    Sphere sphere4(Vector3(0, 0, -1000), 940, Vector3(1, 0, 1));
    Sphere bottomSphere(Vector3(0, -1000, 0), 960, Vector3(1, 0, 1));

    // scene.addIntersectableObject(sphere1);
    // scene.addIntersectableObject(sphere2);
    // scene.addIntersectableObject(sphere3);
    // scene.addIntersectableObject(sphere4);
    scene.addIntersectableObject(smallSphere);
    scene.addIntersectableObject(bottomSphere);

    LightSource lightSource(Vector3(-10, 20, 40), 1E7);
    scene.setLightSource(lightSource);

    Camera camera(Vector3(0, 0, 55), 60 * M_PI / 180.);
    image.setCamera(camera);

    image.setScene(scene);

    image.draw();
    image.save("image.png");

    return 0;
}