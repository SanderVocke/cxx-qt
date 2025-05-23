// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qdate.h"

#include <cxx-qt-lib/assertion_utils.h>

// QDate has a single 64-Bit integer member
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v5.15.6-lts-lgpl#n176
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/time/qdatetime.h?h=v6.2.4#n147
assert_alignment_and_size(QDate, { ::std::int64_t a0; });

static_assert(::std::is_trivially_copyable<QDate>::value,
              "QDate must be trivially copyable!");

namespace rust {
namespace cxxqtlib1 {

qint64
qdateDaysTo(const QDate& date, QDate d)
{
  // In Qt 5 d is const-ref, in Qt 6 it is value
  return date.daysTo(d);
}

QDate
qdateFromString(const QString& string, const QString& format)
{
  return QDate::fromString(string, format);
}

QDate
qdateFromString(const QString& string, Qt::DateFormat format)
{
  return QDate::fromString(string, format);
}

QString
qdateToFormat(const QDate& date, const QString& format)
{
  return date.toString(format);
}

}
}
