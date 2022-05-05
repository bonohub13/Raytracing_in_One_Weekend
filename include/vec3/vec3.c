#include <math.h>
#include "vec3.h"

Color color(double e0, double e1, double e2) {
    Color v;
    
    v.e[0] = e0;
    v.e[1] = e1;
    v.e[2] = e2;

    return v;
}

Point3 point3(double e0, double e1, double e2) {
    Point3 v;
    
    v.e[0] = e0;
    v.e[1] = e1;
    v.e[2] = e2;

    return v;
}

Vec3 vec3(double e0, double e1, double e2) {
    Vec3 v;
    
    v.e[0] = e0;
    v.e[1] = e1;
    v.e[2] = e2;

    return v;
}

double x(const Vec3 *v) { return v->e[0]; }
double y(const Vec3 *v) { return v->e[1]; }
double z(const Vec3 *v) { return v->e[2]; }

// Vector manipulation
Vec3 addVec3(const Vec3 *a, const Vec3 *b) {
    Vec3 retval;
    int i;

    for (i=0; i<3; i++) retval.e[i] = a->e[i] + b->e[i];

    return retval;
}

void addToVec3(Vec3 *a, const Vec3 *b) {
    int i;

    for (i=0; i<3; i++) a->e[i] += b->e[i];
}

Vec3 substractVec3(const Vec3 *a, const Vec3 *b) {
    Vec3 retval;
    int i;

    for (i=0; i<3; i++) retval.e[i] = a->e[i] - b->e[i];

    return retval;
}

void substractFromVec3(Vec3 *a, const Vec3 *b) {
    int i;

    for (i=0; i<3; i++) a->e[i] -= b->e[i];
}

Vec3 multVec3(const Vec3 *a, const double t) {
    Vec3 retval;
    int i;

    for (i=0; i<3; i++) retval.e[i] = a->e[i] * t;

    return retval;
}

void multToVec3(Vec3 *a, const double t) {
    int i;

    for (i=0; i<3; i++) a->e[i] *= t;
}

Vec3 divVec3(const Vec3 *a, const double t) {
    return multVec3(a, 1/t);
}

void divToVec3(Vec3 *a, const double t) {
    multToVec3(a, 1/t);
}

Vec3 mult2Vec3(const Vec3 *u, const Vec3 *v) {
    Vec3 retval;
    int i;

    for (i=0; i<3; i++) retval.e[i] = u->e[i] * v->e[i];

    return retval;
}

Vec3 negVec3(const Vec3 *v) {
    Vec3 retval;
    int i;

    for (i=0; i<3; i++) retval.e[i] = -v->e[i];

    return retval;
}

void neg(Vec3 *v) {
    int i;

    for (i=0; i<3; i++) v->e[i] = -v->e[i];
}

// Methods for Vec3, Point3, Color
double length(const Vec3 *v) {
    return sqrt(lenght_squared(v));
}

double lenght_squared(const Vec3 *v) {
    double retval = 0;
    int i;

    for (i=0; i<3; i++) retval += v->e[i] * v->e[i];

    return retval;
}

double dot(const Vec3 *u, const Vec3 *v) {
    return u->e[0] * v->e[0]
        + u->e[1] * v->e[1]
        + u->e[2] * v->e[2];
}

Vec3 cross(const Vec3 *u, const Vec3 *v) {
    Vec3 retval;
    int i;

    for (i=0; i<3; i++)
        retval.e[i] = u->e[(i+1)%3] * v->e[(i+2)%3]
                     - u->e[(i+2)%3] * v->e[(i+1)%3];

    return retval;
}

Vec3 unit_vector(const Vec3 *v) {
    return divVec3(v, length(v));
}
