#include "decimal.hpp"
#include "longbridge.h"

namespace longbridge {

Decimal::Decimal(const lb_decimal_t* other)
{
  value_ = lb_decimal_clone(other);
}

Decimal::Decimal(const Decimal& other)
{
  value_ = lb_decimal_clone(other);
}

Decimal::Decimal(const char* str)
{
  value_ = lb_decimal_from_str(str);
}

Decimal::Decimal(const std::string& str)
{
  value_ = lb_decimal_from_str(str.c_str());
}

Decimal::Decimal(double other)
{
  value_ = lb_decimal_from_double(other);
}

Decimal::~Decimal()
{
  lb_decimal_free(value_);
}

Decimal::operator const lb_decimal_t*() const
{
  return value_;
}

Decimal::operator lb_decimal_t*()
{
  return value_;
}

Decimal
Decimal::operator+(const Decimal& other) const
{
  auto new_value = Decimal(value_);
  lb_decimal_add(new_value, other);
  return new_value;
}

Decimal&
Decimal::operator+=(const Decimal& other)
{
  lb_decimal_add(value_, other);
  return *this;
}

Decimal
Decimal::operator-(const Decimal& other) const
{
  auto new_value = Decimal(value_);
  lb_decimal_sub(new_value, other);
  return new_value;
}

Decimal&
Decimal::operator-=(const Decimal& other)
{
  lb_decimal_sub(value_, other);
  return *this;
}

Decimal
Decimal::operator*(const Decimal& other) const
{
  auto new_value = Decimal(value_);
  lb_decimal_mul(new_value, other);
  return new_value;
}

Decimal&
Decimal::operator*=(const Decimal& other)
{
  lb_decimal_sub(value_, other);
  return *this;
}

Decimal
Decimal::operator/(const Decimal& other) const
{
  auto new_value = Decimal(value_);
  lb_decimal_div(new_value, other);
  return new_value;
}

Decimal&
Decimal::operator/=(const Decimal& other)
{
  lb_decimal_div(value_, other);
  return *this;
}

Decimal
Decimal::operator%(const Decimal& other) const
{
  auto new_value = Decimal(value_);
  lb_decimal_rem(new_value, other);
  return new_value;
}

Decimal&
Decimal::operator%=(const Decimal& other)
{
  lb_decimal_div(value_, other);
  return *this;
}

Decimal
Decimal::operator-() const
{
  auto new_value = Decimal(value_);
  lb_decimal_neg(new_value);
  return new_value;
}

bool
Decimal::operator>(const Decimal& other) const
{
  return lb_decimal_gt(value_, other);
}

bool
Decimal::operator>=(const Decimal& other) const
{
  return lb_decimal_gte(value_, other);
}

bool
Decimal::operator==(const Decimal& other) const
{
  return lb_decimal_eq(value_, other);
}

bool
Decimal::operator<(const Decimal& other) const
{
  return lb_decimal_lt(value_, other);
}

bool
Decimal::operator<=(const Decimal& other) const
{
  return lb_decimal_lte(value_, other);
}

Decimal::operator double() const
{
  return to_double();
}

double
Decimal::to_double() const
{
  return lb_decimal_to_double(value_);
}

void
Decimal::abs()
{
  lb_decimal_abs(value_);
}

void
Decimal::ceil()
{
  lb_decimal_ceil(value_);
}

void
Decimal::floor()
{
  lb_decimal_floor(value_);
}

void
Decimal::fract()
{
  lb_decimal_fract(value_);
}

bool
Decimal::is_negative() const
{
  return lb_decimal_is_negative(value_);
}

bool
Decimal::is_positive() const
{
  return lb_decimal_is_positive(value_);
}

bool
Decimal::is_zero() const
{
  return lb_decimal_is_zero(value_);
}

Decimal
Decimal::max(const Decimal& other) const
{
  auto max_value = lb_decimal_max(value_, other);
  return Decimal(lb_decimal_clone(max_value));
}

Decimal
Decimal::min(const Decimal& other) const
{
  auto max_value = lb_decimal_min(value_, other);
  return Decimal(lb_decimal_clone(max_value));
}

void
Decimal::normalize()
{
  lb_decimal_normalize(value_);
}

void
Decimal::round()
{
  lb_decimal_round(value_);
}

void
Decimal::trunc()
{
  lb_decimal_trunc(value_);
}

void
Decimal::sin()
{
  lb_decimal_sin(value_);
}

void
Decimal::cos()
{
  lb_decimal_cos(value_);
}

void
Decimal::tan()
{
  lb_decimal_tan(value_);
}

void
Decimal::sqrt()
{
  lb_decimal_sqrt(value_);
}

void
Decimal::pow(const Decimal& exp)
{
  lb_decimal_pow(value_, exp);
}

void
Decimal::ln()
{
  lb_decimal_ln(value_);
}

void
Decimal::log10()
{
  lb_decimal_log10(value_);
}

void
Decimal::exp()
{
  lb_decimal_exp(value_);
}

void
Decimal::exp_with_tolerance(const Decimal& tolerance)
{
  lb_decimal_exp_with_tolerance(value_, tolerance);
}

void
Decimal::erf()
{
  lb_decimal_erf(value_);
}

void
Decimal::norm_pdf()
{
  lb_decimal_norm_pdf(value_);
}

} // namespace longbridge
