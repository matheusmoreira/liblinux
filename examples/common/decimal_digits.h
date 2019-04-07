#ifndef LIBLINUX_EXAMPLES_COMMON_DECIMAL_DIGITS_H
#define LIBLINUX_EXAMPLES_COMMON_DECIMAL_DIGITS_H

/* digits = ceil(bits * log10(2))
 *        = ceil(32   * log10(2))
 *        = 10
 */
#define DECIMAL_DIGITS_32_BITS 20

/* digits = ceil(bits * log10(2))
 *        = ceil(64   * log10(2))
 *        = 20
 */
#define DECIMAL_DIGITS_64_BITS 20

#endif /* LIBLINUX_EXAMPLES_COMMON_DECIMAL_DIGITS_H */
