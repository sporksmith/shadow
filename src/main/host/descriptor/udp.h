/*
 * The Shadow Simulator
 * Copyright (c) 2010-2011, Rob Jansen
 * See LICENSE for licensing information
 */

#ifndef SHD_UDP_H_
#define SHD_UDP_H_

#include <glib.h>

#include "main/core/support/definitions.h"

typedef struct _UDP UDP;

UDP* udp_new(const Host* host, guint receiveBufferSize, guint sendBufferSize);
gint udp_shutdown(UDP* udp, gint how);

#endif /* SHD_UDP_H_ */
