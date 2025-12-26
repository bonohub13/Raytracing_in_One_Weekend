#ifndef RTIOW_HITTABLE_HITTABLE_LIST_H
#define RTIOW_HITTABLE_HITTABLE_LIST_H

#include <stddef.h>

#include "hittable.h"

typedef struct hittable_list {
    void ** pp_datas;
    st_hittable_t ** pp_objects;
    size_t capacity;
} st_hittable_list_t;

extern st_hittable_t * create_hittable_list(void);

#endif /* RTIOW_HITTABLE_HITTABLE_LIST_H */
