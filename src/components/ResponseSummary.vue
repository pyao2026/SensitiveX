<script setup>
defineProps({
    response: { type: Object, required: true },
});

function statusClass(status) {
    if (status < 300) return "text-emerald-700 bg-emerald-50 ring-emerald-200";
    if (status < 400) return "text-amber-700 bg-amber-50 ring-amber-200";
    return "text-rose-700 bg-rose-50 ring-rose-200";
}
</script>

<template>
    <div
        class="flex flex-wrap items-center gap-3 border-b border-slate-200 px-4 py-4 sm:px-6"
    >
        <h2 class="mr-auto text-xs font-semibold tracking-wider text-slate-600">
            RESPONSE
        </h2>
        <span
            class="rounded-full px-3 py-1 font-mono text-sm font-semibold ring-1"
            :class="statusClass(response.status)"
        >
            {{ response.status }} {{ response.statusText }}
        </span>
        <span class="font-mono text-sm text-slate-600"
            >{{ response.elapsedMs }} ms</span
        >
    </div>
</template>
