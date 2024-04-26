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

#ifndef DF_USER_OFFCPU_H
#define DF_USER_OFFCPU_H

#define TP_HOOK_NAME "tracepoint/sched/sched_switch"
#define KP_HOOK_NAME "finish_task_switch"

#undef CP_PROFILE_SET_PROBES
// tracepoint/sched/sched_process_exec
// tracepoint/syscalls/sys_exit_mmap
// tracepoint/syscalls/sys_enter_munmap
#define CP_PROFILE_SET_PROBES(T)                        \
do {                                                    \
        int index = 0, curr_idx;                        \
        tps_set_symbol((T), "tracepoint/sched/sched_process_exec");              \
	tps_set_symbol((T), "tracepoint/syscalls/sys_exit_mmap");              \
	tps_set_symbol((T), "tracepoint/syscalls/sys_enter_mmap");              \
	tps_set_symbol((T), "tracepoint/syscalls/sys_enter_munmap");              \
        (T)->tps_nr = index;                            \
} while(0)

#if 0
#define CP_PROFILE_SET_PROBES(T)			\
do {							\
	int index = 0, curr_idx;			\
	probes_set_enter_symbol((T), KP_HOOK_NAME);  	\
	tps->kprobes_nr = index;			\
	index = 0;					\
	tps_set_symbol((T), TP_HOOK_NAME);		\
	(T)->tps_nr = index;				\
} while(0)
#endif
#endif /*DF_USER_OFFCPU_H */
