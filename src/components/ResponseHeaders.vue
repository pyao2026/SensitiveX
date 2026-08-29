<script setup>
defineProps({
    headers: { type: Array, required: true },
});
</script>

<template>
    <div class="p-4 sm:p-6">
        <h3 class="mb-3 text-xs font-semibold tracking-wider text-slate-600">
            RESPONSE HEADERS
        </h3>
        <dl class="space-y-2 font-mono text-xs leading-5">
            <div
                v-for="(header, index) in headers"
                :key="`${header.name}-${header.value}-${index}`"
                class="break-all"
            >
                <dt class="text-slate-500">{{ header.name }}</dt>
                <dd class="text-slate-800">{{ header.value }}</dd>
            </div>
        </dl>
    </div>
</template>
