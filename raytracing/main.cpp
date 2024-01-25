#define _CRT_SECURE_NO_WARNINGS 1
#include <cmath>
#include <vector>

#include "include/core/light_source.h"
#include "include/core/scene.h"
#include "include/core/sphere.h"
#include "include/core/sphere_builder.h"
#include "include/utils/vector3.h"
#include "include/view/camera.h"
#include "include/view/image.h"

int main()
{
    constexpr int imageWidth = 512;
    constexpr int imageHeight = 512;

    Image image(imageWidth, imageHeight);

    Scene scene;

    SphereBuilder sphereBuilder;

    Sphere mirrorSphere = sphereBuilder.setCenter(Vector3(-25, 0, 0)).setRadius(10).setMirror(true).build();
    sphereBuilder.reset();
    Sphere solidSphere = sphereBuilder.setCenter(Vector3(0, 0, 0)).setRadius(10).setColor(Vector3(1, 0, 0)).build();
    sphereBuilder.reset();
    Sphere transparentSphere =
        sphereBuilder.setCenter(Vector3(25, 0, 0)).setRadius(10).setTransparent(true).setRefractiveIndex(1.5).build();
    sphereBuilder.reset();

    Sphere leftSphere = sphereBuilder.setCenter(Vector3(-1000, 0, 0)).setRadius(940).setColor(Vector3(0, 1, 1)).build();
    sphereBuilder.reset();
    Sphere rightSphere = sphereBuilder.setCenter(Vector3(1000, 0, 0)).setRadius(940).setColor(Vector3(1, 1, 0)).build();
    sphereBuilder.reset();
    Sphere upSphere = sphereBuilder.setCenter(Vector3(0, 1000, 0)).setRadius(940).setColor(Vector3(1, 0, 0)).build();
    sphereBuilder.reset();
    Sphere downSphere = sphereBuilder.setCenter(Vector3(0, -1000, 0)).setRadius(990).setColor(Vector3(0, 0, 1)).build();
    sphereBuilder.reset();
    Sphere frontSphere = sphereBuilder.setCenter(Vector3(0, 0, 1000)).setRadius(940).setColor(Vector3(0, 1, 0)).build();
    sphereBuilder.reset();
    Sphere backSphere = sphereBuilder.setCenter(Vector3(0, 0, -1000)).setRadius(940).setColor(Vector3(1, 0, 1)).build();
    sphereBuilder.reset();

    constexpr double lightSourceIntensity = 5E9;

    LightSource lightSource(Vector3(-10, 20, 40), lightSourceIntensity);

    scene.addIntersectableObject(solidSphere)
        .addIntersectableObject(transparentSphere)
        .addIntersectableObject(mirrorSphere)
        .addIntersectableObject(leftSphere)
        .addIntersectableObject(rightSphere)
        .addIntersectableObject(upSphere)
        .addIntersectableObject(downSphere)
        .addIntersectableObject(frontSphere)
        .addIntersectableObject(backSphere)
        .addLightSource(lightSource);

    constexpr double cameraFov = 75 * M_PI / 180.;

    Camera camera(Vector3(0, 0, 55), cameraFov);

    image.setCamera(camera).setScene(scene);

    image.draw();
    image.save("image.png");

    return 0;
}