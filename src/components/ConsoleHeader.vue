<script setup>
defineProps({
    projectName: { type: String, default: "API Console" },
    isDirty: { type: Boolean, default: false },
    currentId: { type: [Number, String], default: null },
    isSaving: { type: Boolean, default: false },
});

defineEmits(["save", "back", "delete"]);
</script>

<template>
    <header
        class="mb-4 flex items-center justify-between border-b border-slate-300 pb-4"
    >
        <div>
            <p
                class="mb-1 text-xs font-semibold tracking-[0.18em] text-teal-700"
            >
                SENSITIVE X
            </p>
            <h1 class="text-2xl font-semibold tracking-wide">
                {{ projectName }} · API Console
            </h1>
        </div>
        <div class="flex items-center gap-3">
            <button
                class="border border-slate-300 px-3 py-2 text-sm font-semibold text-slate-700 hover:bg-slate-50"
                @click="$emit('back')"
            >
                项目列表
            </button>
            <button
                v-if="currentId !== null"
                class="border border-red-200 px-3 py-2 text-sm font-semibold text-red-700 hover:bg-red-50"
                @click="$emit('delete')"
            >
                删除接口
            </button>
            <span v-if="isDirty" class="text-sm text-amber-700">● 未保存</span>
            <button
                v-if="currentId !== null"
                class="bg-teal-700 px-4 py-2 text-sm font-semibold text-white hover:bg-teal-800 disabled:bg-slate-400"
                :disabled="!isDirty || isSaving"
                @click="$emit('save')"
            >
                {{ isSaving ? "保存中..." : "保存" }}
            </button>
        </div>
    </header>
</template>
