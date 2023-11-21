#pragma once

#include <algorithm>
#include <iterator>
#include <string>
#include <vector>

namespace longport {
namespace utils {

inline std::vector<const char*>
get_cstring_vector(const std::vector<std::string>& values)
{
  std::vector<const char*> cstr_values;
  std::transform(values.cbegin(),
                 values.cend(),
                 std::back_inserter(cstr_values),
                 [](auto& value) { return value.c_str(); });
  return cstr_values;
}

} // namespace utils
} // namespace longport