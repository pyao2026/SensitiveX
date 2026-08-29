<script setup>
import HeadersEditor from "./HeadersEditor.vue";
import JsonBodyEditor from "./JsonBodyEditor.vue";
import RequestTargetBar from "./RequestTargetBar.vue";

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
    "update:headers",
    "update:body",
    "submit",
]);
</script>

<template>
    <form
        class="border border-slate-300 bg-white shadow-sm"
        @submit.prevent="emit('submit')"
    >
        <RequestTargetBar
            :method="method"
            :url="url"
            :is-sending="isSending"
            @update:method="emit('update:method', $event)"
            @update:url="emit('update:url', $event)"
            @submit="emit('submit')"
        />

        <section
            class="grid divide-y divide-slate-200 lg:grid-cols-2 lg:divide-x lg:divide-y-0"
        >
            <HeadersEditor
                :headers="headers"
                @update:headers="emit('update:headers', $event)"
            />
            <JsonBodyEditor
                :method="method"
                :body="body"
                @update:body="emit('update:body', $event)"
            />
        </section>
    </form>
</template>
