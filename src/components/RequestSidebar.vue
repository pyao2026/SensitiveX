<script setup>
defineProps({
    requests: { type: Array, required: true },
    currentId: { type: [Number, String], default: null },
    isLoading: { type: Boolean, default: false },
});

defineEmits(["create", "select"]);
</script>

<template>
    <aside class="border border-slate-300 bg-white shadow-sm">
        <div
            class="flex items-center justify-between border-b border-slate-200 p-4"
        >
            <h2 class="text-xs font-semibold tracking-wider text-slate-600">
                接口
            </h2>
            <button
                class="text-sm font-semibold text-teal-700 hover:text-teal-900"
                @click="$emit('create')"
            >
                + 新建
            </button>
        </div>
        <div v-if="isLoading" class="p-4 text-sm text-slate-500">加载中...</div>
        <div
            v-else-if="!requests.length"
            class="p-4 text-sm leading-6 text-slate-500"
        >
            还没有保存的接口。
        </div>
        <div v-else class="max-h-[calc(100vh-10rem)] overflow-y-auto">
            <button
                v-for="request in requests"
                :key="request.id"
                class="block w-full border-b border-slate-100 p-3 text-left hover:bg-teal-50"
                :class="
                    request.id === currentId
                        ? 'border-l-4 border-l-teal-700 bg-teal-50 pl-2'
                        : ''
                "
                @click="$emit('select', request.id)"
            >
                <div class="flex items-center gap-2">
                    <span
                        class="font-mono text-xs font-bold"
                        :class="
                            request.method === 'POST'
                                ? 'text-orange-700'
                                : 'text-teal-700'
                        "
                    >
                        {{ request.method }}
                    </span>
                    <span class="truncate text-sm font-medium">
                        {{ request.name }}
                    </span>
                </div>
                <div class="mt-1 truncate font-mono text-xs text-slate-500">
                    {{ request.url || "未设置 URL" }}
                </div>
            </button>
        </div>
    </aside>
</template>
