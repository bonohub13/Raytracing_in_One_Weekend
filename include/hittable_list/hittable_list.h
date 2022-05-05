#ifndef HITTABLE_LIST_H
#define HITTABLE_LIST_H

#include <stdbool.h>
#include "../hittable/hittable.h"
#include "../ray/ray.h"

enum data_type {
    NONE,
    SPHERE,
};

enum HittableList_state {
    NOT_VALID,
    IS_FIRST,
    IS_LAST,
    IS_IN_MIDDLE,
};

/* Implementation of vector containing objects with super class "Hittable" using linked list
 *  void *data      -> Data of current node
 *  int data_type   -> Data type of current node
 *  void *prev      -> Data of previous node
 *  void *next      -> Data of next node
 */
typedef struct {
    void *data;
    int data_type;
    void *prev;
    void *next;
} HittableList;

bool hit_HitttableList(
        const HittableList *hl,
        const Ray *r,
        double t_min, double t_max,
        HitRecord *rec
);

#endif //HITTABLE_LIST_H
