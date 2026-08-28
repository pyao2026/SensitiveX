<script setup>
defineProps({
    method: { type: String, required: true },
    url: { type: String, required: true },
    headers: { type: Array, required: true },
    body: { type: String, required: true },
    isSending: { type: Boolean, default: false },
});

const emit = defineEmits([
    "update:method",
    "update:url",
    "update:body",
    "add-header",
    "remove-header",
    "submit",
]);
</script>

<template>
    <form
        class="border border-slate-300 bg-white shadow-sm"
        @submit.prevent="emit('submit')"
    >
        <section class="border-b border-slate-200 p-4 sm:p-6">
            <label
                class="mb-2 block text-xs font-semibold tracking-wider text-slate-600"
                for="request-url"
            >
                REQUEST
            </label>
            <div class="flex flex-col gap-2 sm:flex-row">
                <select
                    :value="method"
                    class="h-11 shrink-0 border border-slate-300 bg-slate-50 px-3 font-mono text-sm font-semibold outline-none focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
                    aria-label="请求方法"
                    @change="emit('update:method', $event.target.value)"
                >
                    <option value="GET">GET</option>
                    <option value="POST">POST</option>
                </select>
                <input
                    id="request-url"
                    :value="url"
                    class="h-11 min-w-0 flex-1 border border-slate-300 px-3 font-mono text-sm outline-none placeholder:text-slate-400 focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
                    type="url"
                    placeholder="https://api.example.com/v1/resource"
                    autocomplete="url"
                    @input="emit('update:url', $event.target.value)"
                />
                <button
                    class="h-11 bg-teal-700 px-5 text-sm font-semibold text-white transition hover:bg-teal-800 disabled:cursor-not-allowed disabled:bg-slate-400"
                    type="submit"
                    :disabled="isSending"
                >
                    {{ isSending ? "请求中..." : "发送请求" }}
                </button>
            </div>
        </section>

        <section
            class="grid divide-y divide-slate-200 lg:grid-cols-2 lg:divide-x lg:divide-y-0"
        >
            <div class="p-4 sm:p-6">
                <div class="mb-3 flex items-center justify-between">
                    <h2
                        class="text-xs font-semibold tracking-wider text-slate-600"
                    >
                        HEADERS
                    </h2>
                    <button
                        class="text-sm font-medium text-teal-700 hover:text-teal-900"
                        type="button"
                        @click="emit('add-header')"
                    >
                        添加请求头
                    </button>
                </div>
                <div class="space-y-2">
                    <div
                        v-for="(header, index) in headers"
                        :key="index"
                        class="flex gap-2"
                    >
                        <input
                            v-model="header.name"
                            class="h-10 min-w-0 flex-1 border border-slate-300 px-3 font-mono text-sm outline-none focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
                            type="text"
                            placeholder="Header name"
                            :aria-label="`请求头 ${index + 1} 名称`"
                        />
                        <input
                            v-model="header.value"
                            class="h-10 min-w-0 flex-1 border border-slate-300 px-3 font-mono text-sm outline-none focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
                            type="text"
                            placeholder="Value"
                            :aria-label="`请求头 ${index + 1} 值`"
                        />
                        <button
                            v-if="headers.length > 1"
                            class="h-10 w-10 shrink-0 border border-slate-300 text-lg text-slate-500 hover:border-rose-300 hover:bg-rose-50 hover:text-rose-700"
                            type="button"
                            :aria-label="`删除请求头 ${index + 1}`"
                            @click="emit('remove-header', index)"
                        >
                            ×
                        </button>
                    </div>
                </div>
            </div>

            <div class="p-4 sm:p-6">
                <div class="mb-3 flex items-center justify-between">
                    <h2
                        class="text-xs font-semibold tracking-wider text-slate-600"
                    >
                        JSON BODY
                    </h2>
                    <span class="text-xs text-slate-400">
                        {{
                            method === "POST"
                                ? "application/json"
                                : "GET 请求不发送正文"
                        }}
                    </span>
                </div>
                <textarea
                    :value="body"
                    class="h-40 w-full resize-y border border-slate-300 bg-slate-50 p-3 font-mono text-sm leading-6 outline-none placeholder:text-slate-400 focus:border-teal-600 focus:ring-2 focus:ring-teal-100 disabled:cursor-not-allowed disabled:bg-slate-100 disabled:text-slate-400"
                    :disabled="method === 'GET'"
                    placeholder="{ }"
                    aria-label="JSON 请求体"
                    spellcheck="false"
                    @input="emit('update:body', $event.target.value)"
                />
            </div>
        </section>
    </form>
</template>
