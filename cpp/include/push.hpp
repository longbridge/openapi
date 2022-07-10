#pragma once

#include <functional>

namespace longbridge {

template<typename Ctx, typename T>
struct PushEvent
{
private:
  Ctx ctx_;
  const T* data_;

public:
  PushEvent(Ctx ctx, const T* data)
    : ctx_(ctx)
    , data_(data)
  {
  }

  inline const T* operator->() const { return data_; }

  inline const Ctx& context() { return ctx_; }
};

template<typename Ctx, typename T>
using PushCallback = std::function<void(PushEvent<Ctx, T>)>;

}