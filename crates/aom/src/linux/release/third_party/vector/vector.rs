/*
The MIT License(MIT)
Copyright(c) 2016 Peter Goldsborough

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions :

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

// Mechanically translated from upstream/aom/third_party/vector/vector.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector {
    pub size: size_t,
    pub capacity: size_t,
    pub element_size: size_t,
    pub data: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Iterator_0 {
    pub pointer: *mut core::ffi::c_void,
    pub element_size: size_t,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const VECTOR_MINIMUM_CAPACITY: core::ffi::c_int = 2 as core::ffi::c_int;
pub const VECTOR_ERROR: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const VECTOR_SUCCESS: core::ffi::c_int = 0 as core::ffi::c_int;
unsafe extern "C-unwind" fn _vector_should_grow(mut vector: *mut Vector) -> bool {
    return (*vector).size == (*vector).capacity;
}
unsafe extern "C-unwind" fn _vector_offset(
    mut vector: *mut Vector,
    mut index: size_t,
) -> *mut core::ffi::c_void {
    return ((*vector).data as *mut core::ffi::c_uchar)
        .offset(index.wrapping_mul((*vector).element_size) as isize)
        as *mut core::ffi::c_void;
}
unsafe extern "C-unwind" fn _vector_assign(
    mut vector: *mut Vector,
    mut index: size_t,
    mut element: *mut core::ffi::c_void,
) {
    let mut offset: *mut core::ffi::c_void = _vector_offset(vector, index);
    memcpy(offset, element, (*vector).element_size);
}
unsafe extern "C-unwind" fn _vector_reallocate(
    mut vector: *mut Vector,
    mut new_capacity: size_t,
) -> core::ffi::c_int {
    let mut new_capacity_in_bytes: size_t = 0;
    let mut old: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    if new_capacity < VECTOR_MINIMUM_CAPACITY as size_t {
        if (*vector).capacity > VECTOR_MINIMUM_CAPACITY as size_t {
            new_capacity = VECTOR_MINIMUM_CAPACITY as size_t;
        } else {
            return VECTOR_SUCCESS
        }
    }
    new_capacity_in_bytes = new_capacity.wrapping_mul((*vector).element_size);
    old = (*vector).data;
    (*vector).data = malloc(new_capacity_in_bytes);
    if ((*vector).data).is_null() {
        return VECTOR_ERROR;
    }
    memcpy((*vector).data, old, aom_vector_byte_size(vector));
    (*vector).capacity = new_capacity;
    free(old);
    return VECTOR_SUCCESS;
}
unsafe extern "C-unwind" fn _vector_adjust_capacity(
    mut vector: *mut Vector,
) -> core::ffi::c_int {
    return _vector_reallocate(
        vector,
        if 1 as size_t > ((*vector).size).wrapping_mul(2 as size_t) {
            1 as size_t
        } else {
            ((*vector).size).wrapping_mul(2 as size_t)
        },
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_vector_setup(
    mut vector: *mut Vector,
    mut capacity: size_t,
    mut element_size: size_t,
) -> core::ffi::c_int {
    if vector.is_null() {
        return VECTOR_ERROR;
    }
    (*vector).size = 0 as size_t;
    (*vector).capacity = if 2 as size_t > capacity { 2 as size_t } else { capacity };
    (*vector).element_size = element_size;
    (*vector).data = malloc(((*vector).capacity).wrapping_mul(element_size));
    return if ((*vector).data).is_null() { VECTOR_ERROR } else { VECTOR_SUCCESS };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_vector_destroy(
    mut vector: *mut Vector,
) -> core::ffi::c_int {
    if vector.is_null() {
        return VECTOR_ERROR;
    }
    free((*vector).data);
    (*vector).data = NULL;
    return VECTOR_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_vector_push_back(
    mut vector: *mut Vector,
    mut element: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    if _vector_should_grow(vector) {
        if _vector_adjust_capacity(vector) == VECTOR_ERROR {
            return VECTOR_ERROR;
        }
    }
    _vector_assign(vector, (*vector).size, element);
    (*vector).size = ((*vector).size).wrapping_add(1);
    return VECTOR_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_vector_is_initialized(mut vector: *const Vector) -> bool {
    return !((*vector).data).is_null();
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_vector_byte_size(mut vector: *const Vector) -> size_t {
    return ((*vector).size).wrapping_mul((*vector).element_size);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_vector_begin(mut vector: *mut Vector) -> Iterator_0 {
    return aom_vector_iterator(vector, 0 as size_t);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_vector_iterator(
    mut vector: *mut Vector,
    mut index: size_t,
) -> Iterator_0 {
    let mut iterator: Iterator_0 = {
        let mut init = Iterator_0 {
            pointer: NULL,
            element_size: 0 as size_t,
        };
        init
    };
    if vector.is_null() {
        return iterator;
    }
    if index > (*vector).size {
        return iterator;
    }
    if (*vector).element_size == 0 as size_t {
        return iterator;
    }
    iterator.pointer = _vector_offset(vector, index);
    iterator.element_size = (*vector).element_size;
    return iterator;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_iterator_get(
    mut iterator: *mut Iterator_0,
) -> *mut core::ffi::c_void {
    return (*iterator).pointer;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_iterator_increment(mut iterator: *mut Iterator_0) {
    (*iterator).pointer = ((*iterator).pointer as *mut core::ffi::c_uchar)
        .offset((*iterator).element_size as isize) as *mut core::ffi::c_void;
}

extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
