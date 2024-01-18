#define _CRT_SECURE_NO_WARNINGS 1
#include <vector>
#include <cmath>

#include "include/utils/vector3.h"
#include "include/view/camera.h"
#include "include/view/image.h"

#include "include/core/scene.h"
#include "include/core/sphere.h"
#include "include/core/light_source.h"

// double sqrtdelta = sqrt(delta);
// double t2 = (-b + sqrtdelta) / (2 * a);
// if (t2 > 0) return false;
// double t1 = (-b - sqrtdelta) / (2 * a);
// double t = t1;
// if (t1 < 0) t = t2;
// P = r.origin + t * r.u;
// N = P - C
// N.normalize();

// return true;

int main()
{
    const int imageWidth = 512;
    const int imageHeight = 512;

    Image image(imageWidth, imageHeight);

    Scene scene;

    Sphere sphere(Vector3(0, 0, 0), 10);
    scene.addIntersectableObject(sphere);

    LightSource lightSource(Vector3(-10, 20, 40), 1E6);
    scene.setLightSource(lightSource);

    Camera camera(Vector3(0, 0, 55), 60 * M_PI / 180.);
    image.setCamera(camera);

    image.setScene(scene);

    image.draw();
    image.save("image.png");

    return 0;
}