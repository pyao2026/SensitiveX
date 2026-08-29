<script setup>
defineProps({
    method: { type: String, required: true },
    url: { type: String, required: true },
    isSending: { type: Boolean, default: false },
});

defineEmits(["update:method", "update:url", "submit"]);
</script>

<template>
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
                @change="$emit('update:method', $event.target.value)"
            >
                <option value="GET">GET</option>
                <option value="POST">POST</option>
                <option value="PUT">PUT</option>
                <option value="PATCH">PATCH</option>
                <option value="DELETE">DELETE</option>
            </select>
            <input
                id="request-url"
                :value="url"
                class="h-11 min-w-0 flex-1 border border-slate-300 px-3 font-mono text-sm outline-none placeholder:text-slate-400 focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
                type="url"
                placeholder="https://api.example.com/v1/resource"
                autocomplete="url"
                @input="$emit('update:url', $event.target.value)"
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
</template>
