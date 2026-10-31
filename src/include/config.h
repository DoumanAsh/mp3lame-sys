#ifndef LAME_CONFIG_H
#define LAME_CONFIG_H

/* ------------------------------------------------------------------ */
/*  Compiler / platform helpers                                        */
/* ------------------------------------------------------------------ */

#if defined(_MSC_VER)
  /* Silence MSVC CRT deprecation warnings for standard C functions */
# ifndef _CRT_SECURE_NO_WARNINGS
#  define _CRT_SECURE_NO_WARNINGS 1
# endif
# ifndef _CRT_NONSTDC_NO_WARNINGS
#  define _CRT_NONSTDC_NO_WARNINGS 1
# endif

  /* MSVC cannot bother to be POSIX compliant */
# define HAVE_UNISTD_H        0
# define HAVE_STRINGS_H       0
# define HAVE_SYS_TIME_H      0
# define HAVE_SYS_TYPES_H     1
# define HAVE_ALLOCA_H        0
# define HAVE_DLFCN_H         0
#else
  /* Non-MSVC (GCC, Clang, ICC, etc.) – assume POSIX environment */
# define HAVE_UNISTD_H        1
# define HAVE_STRINGS_H       1
# define HAVE_SYS_TIME_H      1
# define HAVE_SYS_TYPES_H     1
# define HAVE_ALLOCA_H        1
# define HAVE_DLFCN_H         1
#endif

/* ------------------------------------------------------------------ */
/*  Fixed-width integer types – prefer <stdint.h>                      */
/* ------------------------------------------------------------------ */

#define HAVE_STDINT_H 1

#if defined(HAVE_STDINT_H) && HAVE_STDINT_H
# include <stdint.h>
#endif

/* Tell LAME that the fixed-width types are available */
#define HAVE_INT8_T    1
#define HAVE_INT16_T   1
#define HAVE_INT32_T   1
#define HAVE_INT64_T   1
#define HAVE_UINT8_T   1
#define HAVE_UINT16_T  1
#define HAVE_UINT32_T  1
#define HAVE_UINT64_T  1

/* Safety fallbacks (should never be needed with a C99+ compiler) */
#ifndef int8_t
  typedef signed char          int8_t;
#endif
#ifndef int16_t
  typedef short                int16_t;
#endif
#ifndef int32_t
  typedef int                  int32_t;
#endif
#ifndef int64_t
  typedef long long            int64_t;
#endif
#ifndef uint8_t
  typedef unsigned char        uint8_t;
#endif
#ifndef uint16_t
  typedef unsigned short       uint16_t;
#endif
#ifndef uint32_t
  typedef unsigned int         uint32_t;
#endif
#ifndef uint64_t
  typedef unsigned long long   uint64_t;
#endif

/* ------------------------------------------------------------------ */
/*  IEEE floating-point types                                          */
/* ------------------------------------------------------------------ */

#ifndef HAVE_IEEE754_FLOAT32_T
  typedef float                ieee754_float32_t;
#endif

#ifndef HAVE_IEEE754_FLOAT64_T
  typedef double               ieee754_float64_t;
#endif

#ifndef HAVE_IEEE854_FLOAT80_T
  typedef long double          ieee854_float80_t;
#endif

/* ------------------------------------------------------------------ */
/*  Type sizes – generated with sizeof (evaluated at compile time)     */
/* ------------------------------------------------------------------ */

#define SIZEOF_SHORT                sizeof(short)
#define SIZEOF_UNSIGNED_SHORT       sizeof(unsigned short)
#define SIZEOF_INT                  sizeof(int)
#define SIZEOF_UNSIGNED_INT         sizeof(unsigned int)
#define SIZEOF_LONG                 sizeof(long)
#define SIZEOF_UNSIGNED_LONG        sizeof(unsigned long)
#define SIZEOF_LONG_LONG            sizeof(long long)
#define SIZEOF_UNSIGNED_LONG_LONG   sizeof(unsigned long long)
#define SIZEOF_FLOAT                sizeof(float)
#define SIZEOF_DOUBLE               sizeof(double)
#define SIZEOF_LONG_DOUBLE          sizeof(long double)

/* ------------------------------------------------------------------ */
/*  Endianness – attempt at detection for modern compilers            */
/* ------------------------------------------------------------------ */

/*
 * Prefer the GCC/Clang built-in macros, then fall back to a set of well-known architecture / OS defines.
 * Should cover most common compilers
 * MSVC doesn't have anything so we can assume it is LE always for now
 * At the last step we can fallback to endian.h
 */
#if defined(__BYTE_ORDER__) && defined(__ORDER_BIG_ENDIAN__)
# if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
#  define WORDS_BIGENDIAN 1
# endif
#elif defined(_BIG_ENDIAN)      || defined(__BIG_ENDIAN__)   || \
      defined(__ARMEB__)        || defined(__THUMBEB__)      || \
      defined(__AARCH64EB__)    || defined(_MIPSEB)          || \
      defined(__MIPSEB)         || defined(__MIPSEB__)       || \
      (defined(__BIG_ENDIAN__)  && __BIG_ENDIAN__ == 1)      || \
      (defined(_BIG_ENDIAN)     && _BIG_ENDIAN == 1)         || \
      (defined(__BYTE_ORDER) && (__BYTE_ORDER == __BIG_ENDIAN))
# define WORDS_BIGENDIAN 1
#else
#  if defined(__linux) || defined(__HAIKU__)
#    include <endian.h>
#  elif defined(__FreeBSD__) || defined(__NetBSD__) || defined(__OpenBSD__) || defined(__DragonFly__)
#    include <sys/endian.h>
#  elif defined(__APPLE__)
#    include <machine/endian.h>
#  if defined(__BYTE_ORDER) && defined(__BIG_ENDIAN)
#    if __BYTE_ORDER == __BIG_ENDIAN
#      define WORDS_BIGENDIAN 1
#    endif
#  endif
#  elif defined(BYTE_ORDER) && defined(BIG_ENDIAN)
#    if BYTE_ORDER == BIG_ENDIAN
#      define WORDS_BIGENDIAN 1
#    endif
#  endif
#endif

/* Apple universal binary helper */
#if defined(AC_APPLE_UNIVERSAL_BUILD)
# if defined(__BIG_ENDIAN__)
#  define WORDS_BIGENDIAN 1
# endif
#endif

/* ------------------------------------------------------------------ */
/*  Common feature / header detection                                  */
/* ------------------------------------------------------------------ */

/* #undef ABORTFP */
/* #undef AC_APPLE_UNIVERSAL_BUILD */
/* #undef C_ALLOCA */
/* #undef DEBUG */
/* #undef DECODE_ON_THE_FLY */
/* #undef FLOAT */

#define HAVE_ALLOCA            1
#define HAVE_ERRNO_H           1
#define HAVE_FCNTL_H           1
#define HAVE_GETTIMEOFDAY      1
/* #undef HAVE_ICONV */
#define HAVE_INTTYPES_H        1
/* #undef HAVE_LANGINFO_H */
#define HAVE_LIMITS_H          1
/* #undef HAVE_LINUX_SOUNDCARD_H */
#define HAVE_LONG_DOUBLE       1
#define HAVE_LONG_DOUBLE_WIDER 1
/* #undef HAVE_MINIX_CONFIG_H */
/* #undef HAVE_MPG123 */
/* #undef HAVE_NASM */
/* #undef HAVE_NCURSES_TERMCAP_H */
/* #undef HAVE_NL_LANGINFO */
#define HAVE_SOCKET            1
#define HAVE_STDIO_H           1
#define HAVE_STDLIB_H          1
#define HAVE_STRING_H          1
#define HAVE_STRTOL            1
/* #undef HAVE_SYS_SOUNDCARD_H */
#define HAVE_SYS_STAT_H        1
/* #undef HAVE_TERMCAP */
/* #undef HAVE_TERMCAP_H */
#define HAVE_WCHAR_H           1
/* #undef HAVE_XMMINTRIN_H */

/* #undef ICONV_CONST */
/* #undef IPV6 */

#define LAME_LIBRARY_BUILD     1

/* #undef LIBSNDFILE */
#define LT_OBJDIR              ".libs/"

/* #undef MMX_choose_table */
/* #undef NOANALYSIS */

#define PACKAGE                "lame"
#define PACKAGE_BUGREPORT      "lame-dev@lists.sourceforge.net"
#define PACKAGE_NAME           "lame"
#define PACKAGE_STRING         "lame 4.0"
#define PACKAGE_TARNAME        "lame"
#define PACKAGE_URL            ""
#define PACKAGE_VERSION        "4.0"

/* #undef STACK_DIRECTION */

#define STDC_HEADERS           1
#define TAKEHIRO_IEEE754_HACK  1
#define TIME_WITH_SYS_TIME     1
#define USE_FAST_LOG           1

/* Feature-test macros */
#ifndef _ALL_SOURCE
/* # undef _ALL_SOURCE */
#endif
#ifndef _COSMO_SOURCE
/* # undef _COSMO_SOURCE */
#endif
#ifndef _DARWIN_C_SOURCE
/* # undef _DARWIN_C_SOURCE */
#endif
#ifndef __EXTENSIONS__
/* # undef __EXTENSIONS__ */
#endif
#ifndef _GNU_SOURCE
# define _GNU_SOURCE 1
#endif
#ifndef _HPUX_ALT_XOPEN_SOCKET_API
/* # undef _HPUX_ALT_XOPEN_SOCKET_API */
#endif
#ifndef _MINIX
/* # undef _MINIX */
#endif
#ifndef _NETBSD_SOURCE
/* # undef _NETBSD_SOURCE */
#endif
#ifndef _OPENBSD_SOURCE
/* # undef _OPENBSD_SOURCE */
#endif
#ifndef _POSIX_SOURCE
/* # undef _POSIX_SOURCE */
#endif
#ifndef _POSIX_1_SOURCE
/* # undef _POSIX_1_SOURCE */
#endif
#ifndef _POSIX_PTHREAD_SEMANTICS
/* # undef _POSIX_PTHREAD_SEMANTICS */
#endif
#ifndef __STDC_WANT_IEC_60559_ATTRIBS_EXT__
/* # undef __STDC_WANT_IEC_60559_ATTRIBS_EXT__ */
#endif
#ifndef __STDC_WANT_IEC_60559_BFP_EXT__
/* # undef __STDC_WANT_IEC_60559_BFP_EXT__ */
#endif
#ifndef __STDC_WANT_IEC_60559_DFP_EXT__
/* # undef __STDC_WANT_IEC_60559_DFP_EXT__ */
#endif
#ifndef __STDC_WANT_IEC_60559_EXT__
/* # undef __STDC_WANT_IEC_60559_EXT__ */
#endif
#ifndef __STDC_WANT_IEC_60559_FUNCS_EXT__
/* # undef __STDC_WANT_IEC_60559_FUNCS_EXT__ */
#endif
#ifndef __STDC_WANT_IEC_60559_TYPES_EXT__
/* # undef __STDC_WANT_IEC_60559_TYPES_EXT__ */
#endif
#ifndef __STDC_WANT_LIB_EXT2__
/* # undef __STDC_WANT_LIB_EXT2__ */
#endif
#ifndef __STDC_WANT_MATH_SPEC_FUNCS__
/* # undef __STDC_WANT_MATH_SPEC_FUNCS__ */
#endif
#ifndef _TANDEM_SOURCE
/* # undef _TANDEM_SOURCE */
#endif
#ifndef _XOPEN_SOURCE
/* # undef _XOPEN_SOURCE */
#endif

#define VERSION                "4.0"

/* #undef WITH_DMALLOC */

/* #undef _FILE_OFFSET_BITS */
/* #undef _LARGE_FILES */
/* #undef _TIME_BITS */
/* #undef __DECALPHA__ */
/* #undef __MINGW_USE_VC2005_COMPAT */
/* #undef __NO_MATH_INLINES */
/* #undef const */

/* #undef size_t */

#if (defined(_M_X64) || defined(MIN_ARCH_SSE)) && !defined(HAVE_XMMINTRIN_H)
    #define HAVE_XMMINTRIN_H
#endif

#endif /* LAME_CONFIG_H */
