#include "status.hpp"
#include "convert.hpp"
#include "longport.h"

namespace longport {

Status::Status()
{
  err_ = nullptr;
  need_free_ = false;
}

Status::Status(const lb_error_t* err)
{
  err_ = err;
  need_free_ = false;
}

Status::Status(lb_error_t* err)
{
  err_ = err;
  need_free_ = true;
}

Status::Status(Status&& status) noexcept
{
  err_ = status.err_;
  need_free_ = status.need_free_;
  status.err_ = nullptr;
  status.need_free_ = false;
}

Status::~Status()
{
  if (err_ && need_free_) {
    lb_error_free((lb_error_t*)err_);
  }
}

/// Returns `true` if no errors occurs
bool
Status::is_ok() const
{
  return err_ == nullptr;
}

/// Returns `true` if an errors occurs
bool
Status::is_err() const
{
  return err_ != nullptr;
}

std::optional<ErrorKind>
Status::kind() const
{
  return err_ ? std::make_optional(
                  convert::convert(lb_error_kind(err_)))
              : std::nullopt;
}

std::optional<int64_t>
Status::code() const
{
  return err_ ? std::make_optional(lb_error_code(err_)) : std::nullopt;
}

std::optional<const char*>
Status::message() const
{
  return err_ ? std::make_optional(lb_error_message(err_)) : std::nullopt;
}

} // namespace longport