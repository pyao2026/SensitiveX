<script setup>
import ResponseBody from "./ResponseBody.vue";
import ResponseHeaders from "./ResponseHeaders.vue";
import ResponseSummary from "./ResponseSummary.vue";

defineProps({
    response: { type: Object, required: true },
});
</script>

<template>
    <section class="mt-6 border border-slate-300 bg-white shadow-sm">
        <ResponseSummary :response="response" />
        <div
            class="grid divide-y divide-slate-200 lg:grid-cols-3 lg:divide-x lg:divide-y-0"
        >
            <ResponseHeaders :headers="response.headers" />
            <ResponseBody :response="response" />
        </div>
    </section>
</template>
