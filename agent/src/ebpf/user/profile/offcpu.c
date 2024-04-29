/*
 * Copyright (c) 2024 Yunshan Networks
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/*
 * TODO (@jiping)
 * There are some issues with aarch64 musl compilation, and the profiler
 * cannot be applied temporarily in scenarios where aarch64 is compiled
 * using musl.
 */
#ifndef AARCH64_MUSL
#include <sys/stat.h>
#include <bcc/perf_reader.h>
#include "../config.h"
#include "../utils.h"
#include "../common.h"
#include "../mem.h"
#include "../log.h"
#include "../types.h"
#include "../vec.h"
#include "../tracer.h"
#include "../socket.h"
#include "perf_profiler.h"

extern struct bpf_tracer *profiler_tracer;
extern volatile u64 profiler_stop;
static u64 offcpu_buf_lost_a_count;
static u64 offcpu_buf_lost_b_count;

static void offcpu_reader_work(void *arg)
{
	for (;;)
		sleep(10);
}

static void reader_lost_cb_a(void *cookie, u64 lost)
{
	struct bpf_tracer *tracer = profiler_tracer;
	atomic64_add(&tracer->lost, lost);
	offcpu_buf_lost_a_count++;
}

static void reader_lost_cb_b(void *cookie, u64 lost)
{
	struct bpf_tracer *tracer = profiler_tracer;
	atomic64_add(&tracer->lost, lost);
	offcpu_buf_lost_b_count++;
}

static void reader_raw_cb(void *cookie, void *raw, int raw_size)
{
	if (unlikely(profiler_stop == 1))
		return;

	struct reader_forward_info *fwd_info = cookie;
	if (unlikely(fwd_info->queue_id != 0)) {
		ebpf_warning("cookie(%d) error", (u64) cookie);
		return;
	}

	struct bpf_tracer *tracer = profiler_tracer;
#if 0
	struct stack_trace_key_t *v;
	struct bpf_tracer *tracer = profiler_tracer;
	v = (struct stack_trace_key_t *)raw;

	int ret = VEC_OK;
	vec_add1(raw_stack_data, *v, ret);
	if (ret != VEC_OK) {
		ebpf_warning("vec add failed\n");
	}
#endif
	atomic64_add(&tracer->recv, 1);
}

int extended_reader_create(struct bpf_tracer *tracer)
{
	struct bpf_perf_reader *reader_a, *reader_b;
	reader_a = create_perf_buffer_reader(tracer,
					     MAP_OFFCPU_BUF_A_NAME,
					     reader_raw_cb,
					     reader_lost_cb_a,
					     PROFILE_PG_CNT_DEF, 1,
					     PROFILER_READER_EPOLL_TIMEOUT);
	if (reader_a == NULL)
		return ETR_NORESOURCE;

	reader_b = create_perf_buffer_reader(tracer,
					     MAP_OFFCPU_BUF_B_NAME,
					     reader_raw_cb,
					     reader_lost_cb_b,
					     PROFILE_PG_CNT_DEF, 1,
					     PROFILER_READER_EPOLL_TIMEOUT);
	if (reader_b == NULL) {
		free_perf_buffer_reader(reader_a);
		return ETR_NORESOURCE;
	}

	printf("XXXXXXXXXXXXXXX \n");
	return 0;
}

#else /* defined AARCH64_MUSL */
int extended_reader_create(struct bpf_tracer *tracer)
{
	return 0;
}

#endif /* AARCH64_MUSL */
