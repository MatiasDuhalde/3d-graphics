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
    constexpr int imageWidth = 512;
    constexpr int imageHeight = 512;

    Image image(imageWidth, imageHeight);

    Scene scene;

    Sphere smallSphere(Vector3(0, 0, 0), 10, Vector3(1, 0, 0));

    Sphere leftSphere(Vector3(-1000, 0, 0), 940, Vector3(0, 1, 1));
    Sphere rightSphere(Vector3(1000, 0, 0), 940, Vector3(1, 1, 0));
    Sphere upSphere(Vector3(0, 1000, 0), 940, Vector3(1, 0, 0));
    Sphere downSphere(Vector3(0, -1000, 0), 990, Vector3(0, 0, 1));
    Sphere frontSphere(Vector3(0, 0, 1000), 940, Vector3(0, 1, 0));
    Sphere backSphere(Vector3(0, 0, -1000), 940, Vector3(1, 0, 1));

    scene.addIntersectableObject(smallSphere);

    scene.addIntersectableObject(leftSphere);
    scene.addIntersectableObject(rightSphere);
    scene.addIntersectableObject(upSphere);
    scene.addIntersectableObject(downSphere);
    scene.addIntersectableObject(frontSphere);
    scene.addIntersectableObject(backSphere);

    constexpr double lightSourceIntensity = 5E9;

    LightSource lightSource(Vector3(-10, 20, 40), lightSourceIntensity);
    scene.setLightSource(lightSource);

    constexpr double cameraFov = 75 * M_PI / 180.;

    Camera camera(Vector3(0, 0, 55), cameraFov);
    image.setCamera(camera);

    image.setScene(scene);

    image.draw();
    image.save("image.png");

    return 0;
}