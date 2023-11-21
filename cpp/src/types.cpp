#include "types.hpp"
#include "longport.h"

namespace longport {
namespace quote {

SubFlags
SubFlags::QUOTE()
{
  return SubFlags(LB_SUBFLAGS_QUOTE);
}

SubFlags
SubFlags::DEPTH()
{
  return SubFlags(LB_SUBFLAGS_DEPTH);
}

SubFlags
SubFlags::BROKER()
{
  return SubFlags(LB_SUBFLAGS_BROKER);
}

SubFlags
SubFlags::TRADE()
{
  return SubFlags(LB_SUBFLAGS_TRADE);
}

bool
DerivativeType::has_option()
{
  return (value & LB_DERIVATIVE_TYPE_OPTION) > 0;
}

bool
DerivativeType::has_warrant()
{
  return (value & LB_DERIVATIVE_TYPE_WARRANT) > 0;
}

} // namespace quote

} // namespace longport