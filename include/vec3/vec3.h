#ifndef VEC3_H
#define VEC3_H

typedef struct {
    double e[3];
} Vec3;

typedef Vec3 Point3;
typedef Vec3 Color;

// Constructor
Color color(double e0, double e1, double e2);
Point3 point3(double e0, double e1, double e2);
Vec3 vec3(double e0, double e1, double e2);

double x(const Vec3 *v);
double y(const Vec3 *v);
double z(const Vec3 *v);

// Vector manipulation
Vec3 addVec3(const Vec3 *a, const Vec3 *b);
void addToVec3(Vec3 *a, const Vec3 *b);
Vec3 substractVec3(const Vec3 *a, const Vec3 *b);
void substractFromVec3(Vec3 *a, const Vec3 *b);
Vec3 multVec3(const Vec3 *a, const double t);
void multToVec3(Vec3 *a, const double t);
Vec3 divVec3(const Vec3 *a, const double t);
void divToVec3(Vec3 *a, const double t);
Vec3 mult2Vec3(const Vec3 *u, const Vec3 *v);

// methods
double length(const Vec3 *v);
double lenght_squared(const Vec3 *v);
double dot(const Vec3 *u, const Vec3 *v);
Vec3 cross(const Vec3 *u, const Vec3 *v);
Vec3 unit_vector(const Vec3 *v);

#endif
