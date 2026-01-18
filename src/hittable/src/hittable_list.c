#include <string.h>

#include "hittable_list.h"

static bool hittable_list_hit(const void * const p_obj,
        const st_ray_t * const p_ray, const st_interval_t * const p_ray_t,
        st_hit_record_t * const p_rec);

static const st_hittable_t s_hittable_list = {
    .p_hit = hittable_list_hit,
};

st_hittable_t * create_hittable_list(void) {
    return (st_hittable_t*)&s_hittable_list;
}

static bool hittable_list_hit(const void * const p_obj,
        const st_ray_t * const p_ray, const st_interval_t * const p_ray_t,
        st_hit_record_t * const p_rec) {
    st_hittable_list_t * p_hittable_list = (st_hittable_list_t*)p_obj;
    void * p_current_data;
    st_hittable_t * p_current_object;
    st_interval_t closest_so_far = *p_ray_t;
    st_hit_record_t tmp_rec = { 0 };
    bool hit_anything = false;
    size_t i = 0;

    for (i = 0; i < p_hittable_list->capacity; i++) {
        p_current_data = p_hittable_list->pp_datas[i];
        p_current_object = p_hittable_list->pp_objects[i];
        if (p_current_object->p_hit(
                    p_current_data,
                    p_ray,
                    &closest_so_far,
                    &tmp_rec)) {
            hit_anything = true;
            closest_so_far.max = tmp_rec.t;
            *p_rec = tmp_rec;
        }
    }

    return hit_anything;
}
