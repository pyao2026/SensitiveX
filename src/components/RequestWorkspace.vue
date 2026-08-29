<script setup>
import ErrorAlert from "./ErrorAlert.vue";
import RequestEditor from "./RequestEditor.vue";
import ResponsePanel from "./ResponsePanel.vue";

defineProps({
    name: { type: String, required: true },
    method: { type: String, required: true },
    url: { type: String, required: true },
    headers: { type: Array, required: true },
    body: { type: String, required: true },
    isSending: { type: Boolean, default: false },
    error: { type: String, default: "" },
    response: { type: Object, default: null },
});

defineEmits([
    "update:name",
    "update:method",
    "update:url",
    "update:headers",
    "update:body",
    "submit",
]);
</script>

<template>
    <section class="min-w-0">
        <div class="mb-4 border border-slate-300 bg-white p-4 shadow-sm">
            <label
                class="mb-2 block text-xs font-semibold tracking-wider text-slate-600"
                for="request-name"
            >
                接口名称
            </label>
            <input
                id="request-name"
                :value="name"
                class="h-10 w-full border border-slate-300 px-3 text-sm outline-none focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
                placeholder="未命名请求"
                @input="$emit('update:name', $event.target.value)"
            />
        </div>
        <RequestEditor
            :method="method"
            :url="url"
            :headers="headers"
            :body="body"
            :is-sending="isSending"
            @update:method="$emit('update:method', $event)"
            @update:url="$emit('update:url', $event)"
            @update:headers="$emit('update:headers', $event)"
            @update:body="$emit('update:body', $event)"
            @submit="$emit('submit')"
        />
        <ErrorAlert :message="error" />
        <ResponsePanel v-if="response" :response="response" />
    </section>
</template>
