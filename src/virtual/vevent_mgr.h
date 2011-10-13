/**
 * The Shadow Simulator
 *
 * Copyright (c) 2010-2011 Rob Jansen <jansen@cs.umn.edu>
 *
 * This file is part of Shadow.
 *
 * Shadow is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Shadow is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Shadow.  If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef VEVENT_MGR_H_
#define VEVENT_MGR_H_

#include <glib.h>
#include <glib-2.0/glib.h>

#include <event2/event_struct.h>
#include <event2/event.h>
#include <event2/util.h>
#include <event2/dns.h>
#include <event2/dns_compat.h>
#include <event2/dns_struct.h>

/* make libevent types slightly prettier */
typedef struct event event_t, *event_tp;
typedef struct event_base event_base_t, *event_base_tp;

typedef struct evdns_base evdns_base_t, *evdns_base_tp;
typedef struct evdns_request evdns_request_t, *evdns_request_tp;
typedef struct evdns_server_request evdns_server_request_t, *evdns_server_request_tp;
typedef struct evdns_server_port evdns_server_port_t, *evdns_server_port_tp;

typedef void (*vevent_mgr_timer_callback_fp)(gpointer arg);

/* holds all registered vevents and sockets */
typedef struct vevent_base_s {
	gint nextid;
	GHashTable *vevents_by_id;
	GHashTable *sockets_by_sd;
} vevent_base_t, *vevent_base_tp;

/* holds all event bases that the user creates (each holds pointer to a vevent base) */
typedef struct vevent_mgr_s {
	/* holds event_base_tp */
	GQueue *event_bases;
	GHashTable * base_conversion;
	vevent_mgr_timer_callback_fp loopexit_fp;
	gchar typebuf[80];
	gint id_counter;
} vevent_mgr_t, *vevent_mgr_tp;

/* public vevent api */
//void vevent_mgr_init(vevent_mgr_tp mgr);
//void vevent_mgr_uninit(vevent_mgr_tp mgr);
vevent_mgr_tp vevent_mgr_create();
void vevent_mgr_destroy(vevent_mgr_tp mgr);

void vevent_mgr_track_base(vevent_mgr_tp mgr, event_base_tp eb, vevent_base_tp veb);
void vevent_mgr_untrack_base(vevent_mgr_tp mgr, event_base_tp eb);
vevent_base_tp vevent_mgr_convert_base(vevent_mgr_tp mgr, event_base_tp eb);

void vevent_mgr_timer_create(vevent_mgr_tp mgr, gint milli_delay, CallbackFunc callback_function, gpointer cb_arg);
void vevent_mgr_set_loopexit_fn(vevent_mgr_tp mgr, vevent_mgr_timer_callback_fp fn);

//void vevent_mgr_wakeup_all(vevent_mgr_tp mgr);
void vevent_mgr_notify_can_read(vevent_mgr_tp mgr, gint sockfd);
void vevent_mgr_notify_can_write(vevent_mgr_tp mgr, gint sockfd);
void vevent_mgr_notify_signal_received(vevent_mgr_tp mgr, gint signal);

/* mostly for debugging purposes */
void vevent_mgr_print_stat(vevent_mgr_tp mgr, guint16 sockd);
void vevent_mgr_print_all(vevent_mgr_tp mgr);

#endif /* VEVENT_MGR_H_ */
