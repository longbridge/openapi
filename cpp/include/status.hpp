#pragma once

typedef struct lb_error_t lb_error_t;

namespace longbridge {
class Status
{
private:
  const lb_error_t* err_;
  bool need_free_;

public:
  Status();
  Status(const lb_error_t* err);
  Status(lb_error_t* err);
  Status(Status&& status);
  ~Status();

  inline operator bool() { return is_ok(); }

  /// Returns `true` if no errors occurs
  bool is_ok() const;

  /// Returns `true` if an errors occurs
  bool is_err() const;

  /// Returns the error code
  int code() const;

  /// Returns the error message
  const char* message() const;
};

} // namespace longbridge