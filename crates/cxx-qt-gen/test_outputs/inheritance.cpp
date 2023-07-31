#include "cxx-qt-gen/inheritance.cxxqt.h"

MyObject::~MyObject() {}

MyObjectRust const&
MyObject::unsafeRust() const
{
  return *m_rustObj;
}

MyObjectRust&
MyObject::unsafeRustMut()
{
  return *m_rustObj;
}

QVariant
MyObject::data(QModelIndex const& _index, ::std::int32_t _role) const
{
  const auto guard = unsafeRustLock();
  return dataWrapper(_index, _role);
}

bool
MyObject::hasChildren(QModelIndex const& _parent) const
{
  const auto guard = unsafeRustLock();
  return hasChildrenWrapper(_parent);
}

MyObject::MyObject(QObject* parent)
  : QAbstractItemModel(parent)
  , m_rustObj(::cxx_qt_my_object::createRs())
  , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
{
}

::std::lock_guard<::std::recursive_mutex>
MyObject::unsafeRustLock() const
{
  return ::std::lock_guard<::std::recursive_mutex>(*m_rustObjMutex);
}
