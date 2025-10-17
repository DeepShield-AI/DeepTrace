#ifndef __SHIM_SHIM_H__
#define __SHIM_SHIM_H__

#include "types.h"
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>

// this just a simple C macro to make easier shim definition
// the macro prefix the function name by "shim_" so that doing we can
// easily filter the shim functions to bindgen.
#define _SHIM_GETTER(ret, proto, accessed_member)                \
    __attribute__((always_inline)) ret proto {                   \
        return __builtin_preserve_access_index(accessed_member); \
    }

#define _SHIM_GETTER_BPF_CORE_READ(ret, proto, _struct, member) \
    __attribute__((always_inline)) ret proto {                  \
        return BPF_CORE_READ(_struct, member);                  \
    }

#define _SHIM_GETTER_BPF_CORE_READ_BITFIELD(ret, proto, _struct, member) \
    __attribute__((always_inline)) ret proto {                           \
        return BPF_CORE_READ_BITFIELD_PROBED(_struct, member);           \
    }

#define _SHIM_GETTER_BPF_CORE_READ_USER(ret, proto, _struct, member) \
    __attribute__((always_inline)) ret proto {                       \
        return BPF_CORE_READ_USER(_struct, member);                  \
    }

#define _SHIM_GETTER_BPF_CORE_READ_RECAST(ret, proto, old_struct, new_struct, memb) \
    __attribute__((always_inline)) ret proto {                                      \
        struct old_struct *old = (void *) new_struct;                               \
        return BPF_CORE_READ(old, memb);                                            \
    }

// macro used to define a function to check if a field exists
#define _FIELD_EXISTS_DEF(_struct, member, member_name)                                                       \
    __attribute__((always_inline)) _Bool shim_##_struct##_##member_name##_##exists(struct _struct *_struct) { \
        return bpf_core_field_exists(_struct->member);                                                        \
    }

#define SHIM_BITFIELD(_struct, member)                                                                                                               \
    _SHIM_GETTER_BPF_CORE_READ_BITFIELD(typeof(((struct _struct *) 0)->member), shim_##_struct##_##member(struct _struct *_struct), _struct, member) \
    _FIELD_EXISTS_DEF(_struct, member, member)

#define SHIM(_struct, member)                                                                                                                           \
    _SHIM_GETTER_BPF_CORE_READ(typeof(((struct _struct *) 0)->member), shim_##_struct##_##member(struct _struct *_struct), _struct, member)             \
    _SHIM_GETTER_BPF_CORE_READ_USER(typeof(((struct _struct *) 0)->member), shim_##_struct##_##member##_user(struct _struct *_struct), _struct, member) \
    _FIELD_EXISTS_DEF(_struct, member, member)

#define SHIM_WITH_NAME(_struct, member, member_name)                                                                                                         \
    _SHIM_GETTER_BPF_CORE_READ(typeof(((struct _struct *) 0)->member), shim_##_struct##_##member_name(struct _struct *_struct), _struct, member)             \
    _SHIM_GETTER_BPF_CORE_READ_USER(typeof(((struct _struct *) 0)->member), shim_##_struct##_##member_name##_user(struct _struct *_struct), _struct, member) \
    _FIELD_EXISTS_DEF(_struct, member, member_name)

#define SHIM_REF(_struct, member)                                                                                                          \
    _SHIM_GETTER(typeof(&(((struct _struct *) 0)->member)), shim_##_struct##_##member(struct _struct *_struct), &(_struct->member))        \
    _SHIM_GETTER(typeof(&(((struct _struct *) 0)->member)), shim_##_struct##_##member##_user(struct _struct *_struct), &(_struct->member)) \
    _FIELD_EXISTS_DEF(_struct, member, member)

#define ARRAY_SHIM(_struct, member)                                                                                                              \
    _SHIM_GETTER(typeof(&(((struct _struct *) 0)->member[0])), shim_##_struct##_##member(struct _struct *_struct), &(_struct->member[0]))        \
    _SHIM_GETTER(typeof(&(((struct _struct *) 0)->member[0])), shim_##_struct##_##member##_user(struct _struct *_struct), &(_struct->member[0])) \
    _FIELD_EXISTS_DEF(_struct, member, member)

#define ARRAY_SHIM_WITH_NAME(_struct, member, member_name)                                                                                            \
    _SHIM_GETTER(typeof(&(((struct _struct *) 0)->member[0])), shim_##_struct##_##member_name(struct _struct *_struct), &(_struct->member[0]))        \
    _SHIM_GETTER(typeof(&(((struct _struct *) 0)->member[0])), shim_##_struct##_##member_name##_user(struct _struct *_struct), &(_struct->member[0])) \
    _FIELD_EXISTS_DEF(_struct, member, member_name)

#define SHIM_ENUM_VALUE(enum_type, enum_value)                                        \
    __attribute__((always_inline)) unsigned int shim_##enum_type##_##enum_value() {   \
        return bpf_core_enum_value(enum enum_type, enum_value);                       \
    }                                                                                 \
    __attribute__((always_inline)) _Bool shim_##enum_type##_##enum_value##_exists() { \
        return bpf_core_enum_value_exists(enum enum_type, enum_value);                \
    }
#endif // __SHIM_SHIM_H__
