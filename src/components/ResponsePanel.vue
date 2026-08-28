<script setup>
import { computed } from "vue";

const props = defineProps({
    response: { type: Object, required: true },
});

const formattedBody = computed(() => {
    if (!props.response.body) return "";

    try {
        return JSON.stringify(JSON.parse(props.response.body), null, 2);
    } catch {
        return props.response.body;
    }
});

function statusClass(status) {
    if (status < 300) return "text-emerald-700 bg-emerald-50 ring-emerald-200";
    if (status < 400) return "text-amber-700 bg-amber-50 ring-amber-200";
    return "text-rose-700 bg-rose-50 ring-rose-200";
}
</script>

<template>
    <section class="mt-6 border border-slate-300 bg-white shadow-sm">
        <div
            class="flex flex-wrap items-center gap-3 border-b border-slate-200 px-4 py-4 sm:px-6"
        >
            <h2
                class="mr-auto text-xs font-semibold tracking-wider text-slate-600"
            >
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

        <div
            class="grid divide-y divide-slate-200 lg:grid-cols-3 lg:divide-x lg:divide-y-0"
        >
            <div class="p-4 sm:p-6">
                <h3
                    class="mb-3 text-xs font-semibold tracking-wider text-slate-600"
                >
                    RESPONSE HEADERS
                </h3>
                <dl class="space-y-2 font-mono text-xs leading-5">
                    <div
                        v-for="(header, index) in response.headers"
                        :key="`${header.name}-${header.value}-${index}`"
                        class="break-all"
                    >
                        <dt class="text-slate-500">{{ header.name }}</dt>
                        <dd class="text-slate-800">{{ header.value }}</dd>
                    </div>
                </dl>
            </div>
            <div class="min-w-0 p-4 sm:col-span-2 sm:p-6 lg:col-span-2">
                <h3
                    class="mb-3 text-xs font-semibold tracking-wider text-slate-600"
                >
                    RESPONSE BODY
                </h3>
                <pre
                    class="max-h-[32rem] overflow-auto whitespace-pre-wrap break-all bg-slate-950 p-4 font-mono text-xs leading-6 text-slate-100"
                    >{{ formattedBody || "响应正文为空" }}</pre>
            </div>
        </div>
    </section>
</template>
