<script setup>
const props = defineProps({
    headers: { type: Array, required: true },
});

const emit = defineEmits(["update:headers"]);

function updateHeader(index, field, value) {
    emit(
        "update:headers",
        props.headers.map((header, headerIndex) =>
            headerIndex === index ? { ...header, [field]: value } : header,
        ),
    );
}

function addHeader() {
    emit("update:headers", [...props.headers, { name: "", value: "" }]);
}

function removeHeader(index) {
    emit(
        "update:headers",
        props.headers.filter((_, headerIndex) => headerIndex !== index),
    );
}
</script>

<template>
    <div class="p-4 sm:p-6">
        <div class="mb-3 flex items-center justify-between">
            <h2 class="text-xs font-semibold tracking-wider text-slate-600">
                HEADERS
            </h2>
            <button
                class="text-sm font-medium text-teal-700 hover:text-teal-900"
                type="button"
                @click="addHeader"
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
                    :value="header.name"
                    class="h-10 min-w-0 flex-1 border border-slate-300 px-3 font-mono text-sm outline-none focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
                    type="text"
                    placeholder="Header name"
                    :aria-label="`请求头 ${index + 1} 名称`"
                    @input="updateHeader(index, 'name', $event.target.value)"
                />
                <input
                    :value="header.value"
                    class="h-10 min-w-0 flex-1 border border-slate-300 px-3 font-mono text-sm outline-none focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
                    type="text"
                    placeholder="Value"
                    :aria-label="`请求头 ${index + 1} 值`"
                    @input="updateHeader(index, 'value', $event.target.value)"
                />
                <button
                    v-if="headers.length > 1"
                    class="h-10 w-10 shrink-0 border border-slate-300 text-lg text-slate-500 hover:border-rose-300 hover:bg-rose-50 hover:text-rose-700"
                    type="button"
                    :aria-label="`删除请求头 ${index + 1}`"
                    @click="removeHeader(index)"
                >
                    ×
                </button>
            </div>
        </div>
    </div>
</template>
