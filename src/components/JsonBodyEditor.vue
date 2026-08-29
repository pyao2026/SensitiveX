<script setup>
defineProps({
    method: { type: String, required: true },
    body: { type: String, required: true },
});

defineEmits(["update:body"]);
</script>

<template>
    <div class="p-4 sm:p-6">
        <div class="mb-3 flex items-center justify-between">
            <h2 class="text-xs font-semibold tracking-wider text-slate-600">
                JSON BODY
            </h2>
            <span class="text-xs text-slate-400">
                {{
                    method !== "GET"
                        ? "application/json"
                        : `${method} 请求不发送正文`
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
            @input="$emit('update:body', $event.target.value)"
        />
    </div>
</template>
