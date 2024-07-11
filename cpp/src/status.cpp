#include "status.hpp"
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

/// Returns the error code
int64_t
Status::code() const
{
  return lb_error_code(err_);
}

/// Returns the error message
const char*
Status::message() const
{
  return err_ ? lb_error_message(err_) : "no error";
}

} // namespace longport