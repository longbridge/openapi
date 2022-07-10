#pragma once

#include <memory>

#include "async_result.hpp"
#include "push.hpp"

namespace longbridge {
namespace callback {

template<typename Ctx, typename Res>
PushCallback<Ctx, Res>*
get_push_callback(void* p)
{
  return (PushCallback<Ctx, Res>*)p;
}

template<typename Ctx, typename Res>
std::unique_ptr<AsyncCallback<Ctx, Res>>
get_async_callback(void* p)
{
  auto callback_ptr = (AsyncCallback<Ctx, Res>*)p;
  return std::unique_ptr<AsyncCallback<Ctx, Res>>(callback_ptr);
}

} // namespace callback
} // namespace longbridge