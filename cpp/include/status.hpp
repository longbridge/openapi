#pragma once

#include <optional>
#include <stdint.h>

typedef struct lb_error_t lb_error_t;

namespace longport {

enum class ErrorKind
{
  Http,
  OpenApi,
  Other,
};

class Status
{
private:
  const lb_error_t* err_;
  bool need_free_;

public:
  Status();
  Status(const lb_error_t* err);
  Status(lb_error_t* err);
  Status(Status&& status) noexcept;
  ~Status();

  inline operator bool() { return is_ok(); }

  /// Returns `true` if no errors occurs
  bool is_ok() const;

  /// Returns `true` if an errors occurs
  bool is_err() const;

  /// Returns the error kind if an error occurs
  std::optional<ErrorKind> kind() const;

  /// Returns the error code if an error occurs
  std::optional<int64_t> code() const;

  /// Returns the error message if an error occurs
  std::optional<const char*> message() const;
};

} // namespace longport