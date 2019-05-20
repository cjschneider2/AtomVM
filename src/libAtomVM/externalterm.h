/***************************************************************************
 *   Copyright 2017 by Davide Bettio <davide@uninstall.it>                 *
 *                                                                         *
 *   This program is free software; you can redistribute it and/or modify  *
 *   it under the terms of the GNU Lesser General Public License as        *
 *   published by the Free Software Foundation; either version 2 of the    *
 *   License, or (at your option) any later version.                       *
 *                                                                         *
 *   This program is distributed in the hope that it will be useful,       *
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of        *
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the         *
 *   GNU General Public License for more details.                          *
 *                                                                         *
 *   You should have received a copy of the GNU General Public License     *
 *   along with this program; if not, write to the                         *
 *   Free Software Foundation, Inc.,                                       *
 *   51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA .        *
 ***************************************************************************/

/**
 * @file externalterm.h
 * @brief External term deserialization functions
 *
 * @details This header provides external term deserialization functions.
 */

#ifndef _EXTERNALTERM_H_
#define _EXTERNALTERM_H_

#include "term.h"

/**
 * @brief Gets a term from external term data.
 *
 * @details Deserialize an external term from external format and returns a term.
 * @param external_term the external term that will be deserialized.
 * @param ctx the context that owns the memory that will be allocated.
 * @returns a term.
 */
term externalterm_to_term(const void *external_term, Context *ctx);

#endif
