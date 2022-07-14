#pragma once

#include <string>
#include <utility>

typedef struct lb_decimal_t lb_decimal_t;

namespace longbridge {

class Decimal
{
private:
  lb_decimal_t* value_;

public:
  Decimal(const lb_decimal_t* other);
  Decimal(const Decimal& other);
  Decimal(const char* str);
  Decimal(const std::string& str);
  Decimal(double other);
  ~Decimal();

  operator const lb_decimal_t*() const;
  operator lb_decimal_t*();
  operator double() const;

  Decimal operator+(const Decimal& other) const;
  Decimal& operator+=(const Decimal& other);
  Decimal operator-(const Decimal& other) const;
  Decimal& operator-=(const Decimal& other);
  Decimal operator*(const Decimal& other) const;
  Decimal& operator*=(const Decimal& other);
  Decimal operator/(const Decimal& other) const;
  Decimal& operator/=(const Decimal& other);
  Decimal operator%(const Decimal& other) const;
  Decimal& operator%=(const Decimal& other);
  Decimal operator-() const;
  bool operator>(const Decimal& other) const;
  bool operator>=(const Decimal& other) const;
  bool operator==(const Decimal& other) const;
  bool operator<(const Decimal& other) const;
  bool operator<=(const Decimal& other) const;
  double to_double() const;

  /// Computes the absolute value.
  void abs();

  /// Returns the smallest integer greater than or equal to a number.
  void ceil();

  /// Returns the largest integer less than or equal to a number.
  void floor();

  /// Returns a new Decimal representing the fractional portion of the number.
  void fract();

  /// Returns `true` if the decimal is negative.
  bool is_negative() const;

  /// Returns `true` if the decimal is positive.
  bool is_positive() const;

  /// Returns `true` if this Decimal number is equivalent to zero.
  bool is_zero() const;

  /// Returns the maximum of the two numbers.
  Decimal max(const Decimal& other) const;

  /// Returns the minimum of the two numbers.
  Decimal min(const Decimal& other) const;

  /// Strips any trailing zero’s from a Decimal and converts `-0` to `0`.
  void normalize();

  /// Returns a new Decimal number with no fractional portion (i.e. an integer).
  /// Rounding currently follows “Bankers Rounding” rules. e.g. `6.5` -> `6`,
  /// `7.5` -> `8`
  void round();

  /// Returns a new Decimal integral with no fractional portion. This is a true
  /// truncation whereby no rounding is performed.
  void trunc();

  /// Computes the sine of a number (in radians)
  void sin();

  /// Computes the cosine of a number (in radians)
  void cos();

  /// Computes the tangent of a number (in radians). Panics upon overflow or
  /// upon approaching a limit.
  void tan();

  /// The square root of a Decimal. Uses a standard Babylonian method.
  void sqrt();

  /// Raise self to the given Decimal exponent: x<sup>y</sup>. If `exp` is not
  /// whole then the approximation e<sup>y*ln(x)</sup> is used.
  void pow(const Decimal& exp);

  /// Calculates the natural logarithm for a Decimal calculated using Taylor’s
  /// series.
  void ln();

  /// Calculates the base 10 logarithm of a specified Decimal number.
  void log10();

  /// The estimated exponential function, ex. Stops calculating when it is
  /// within tolerance of roughly `0.0000002`.
  void exp();

  /// The estimated exponential function, e<sup>x</sup> using the `tolerance`
  /// provided as a hint as to when to stop calculating. A larger
  /// tolerance will cause the number to stop calculating sooner at the
  /// potential cost of a slightly less accurate result.
  void exp_with_tolerance(const Decimal& tolerance);

  /// Abramowitz Approximation of Error Function from
  /// [wikipedia](https://en.wikipedia.org/wiki/Error_function#Numerical_approximations)
  void erf();

  /// The Probability density function for a Normal distribution.
  void norm_pdf();
};

} // namespace longbridge