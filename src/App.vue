<script setup>
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import RequestEditor from "./components/RequestEditor.vue";
import ResponsePanel from "./components/ResponsePanel.vue";

const method = ref("GET");
const url = ref("");
const headers = ref([{ name: "", value: "" }]);
const body = ref("{\n  \n}");
const name = ref("");
const savedRequests = ref([]);
const currentId = ref(null);
const savedSnapshot = ref("");
const isLoading = ref(true);
const isSaving = ref(false);
const isSending = ref(false);
const error = ref("");
const response = ref(null);

const currentSnapshot = computed(() =>
    JSON.stringify({
        name: name.value,
        method: method.value,
        url: url.value,
        headers: headers.value,
        body: body.value,
    }),
);
const isDirty = computed(
    () =>
        currentId.value !== null &&
        currentSnapshot.value !== savedSnapshot.value,
);

function addHeader() {
    headers.value.push({ name: "", value: "" });
}
function removeHeader(index) {
    headers.value.splice(index, 1);
}

function loadRequest(request) {
    currentId.value = request.id;
    name.value = request.name;
    method.value = request.method;
    url.value = request.url;
    headers.value = request.headers.length
        ? request.headers
        : [{ name: "", value: "" }];
    body.value = request.body || "";
    savedSnapshot.value = currentSnapshot.value;
    response.value = null;
    error.value = "";
}

async function refreshList() {
    savedRequests.value = await invoke("list_saved_requests");
}

async function selectRequest(id) {
    if (id === currentId.value) return;
    if (
        isDirty.value &&
        !window.confirm("当前接口有未保存修改，确定放弃这些修改吗？")
    )
        return;
    try {
        loadRequest(await invoke("get_saved_request", { id }));
    } catch (requestError) {
        error.value = String(requestError);
    }
}

async function createRequest() {
    if (
        isDirty.value &&
        !window.confirm("当前接口有未保存修改，确定放弃这些修改吗？")
    )
        return;
    try {
        const request = await invoke("create_saved_request");
        await refreshList();
        loadRequest(request);
    } catch (requestError) {
        error.value = String(requestError);
    }
}

async function saveRequest() {
    if (currentId.value === null) return;
    error.value = "";
    isSaving.value = true;
    try {
        const request = await invoke("save_saved_request", {
            input: {
                id: currentId.value,
                name: name.value || "未命名请求",
                request: {
                    method: method.value,
                    url: url.value.trim(),
                    headers: headers.value,
                    body:
                        method.value !== "GET"
                            ? body.value.trim() || null
                            : null,
                },
            },
        });
        loadRequest(request);
        await refreshList();
    } catch (saveError) {
        error.value = String(saveError);
    } finally {
        isSaving.value = false;
    }
}

async function sendRequest() {
    error.value = "";
    response.value = null;
    if (!url.value.trim()) {
        error.value = "请输入请求地址。";
        return;
    }
    const activeHeaders = headers.value
        .filter((header) => header.name.trim() || header.value.trim())
        .map((header) => ({ name: header.name.trim(), value: header.value }));
    if (activeHeaders.some((header) => !header.name)) {
        error.value = "每个请求头都需要提供名称。";
        return;
    }
    const requestBody = method.value !== "GET" ? body.value.trim() : "";
    if (requestBody) {
        try {
            JSON.parse(requestBody);
        } catch {
            error.value = "请求体必须是有效的 JSON。";
            return;
        }
    }
    isSending.value = true;
    try {
        response.value = await invoke("request_api", {
            request: {
                method: method.value,
                url: url.value.trim(),
                headers: activeHeaders,
                body: requestBody || null,
            },
        });
    } catch (requestError) {
        error.value = String(requestError);
    } finally {
        isSending.value = false;
    }
}

onMounted(async () => {
    try {
        await refreshList();
        if (savedRequests.value.length)
            loadRequest(
                await invoke("get_saved_request", {
                    id: savedRequests.value[0].id,
                }),
            );
    } catch (loadError) {
        error.value = String(loadError);
    } finally {
        isLoading.value = false;
    }
});
</script>

<template>
    <main
        class="min-h-screen bg-slate-100 px-3 py-4 text-slate-900 sm:px-6 sm:py-6"
    >
        <div class="mx-auto max-w-7xl">
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
                        API Console
                    </h1>
                </div>
                <div class="flex items-center gap-3">
                    <span v-if="isDirty" class="text-sm text-amber-700"
                        >● 未保存</span
                    >
                    <button
                        v-if="currentId !== null"
                        class="bg-teal-700 px-4 py-2 text-sm font-semibold text-white hover:bg-teal-800 disabled:bg-slate-400"
                        :disabled="!isDirty || isSaving"
                        @click="saveRequest"
                    >
                        {{ isSaving ? "保存中..." : "保存" }}
                    </button>
                </div>
            </header>
            <div class="grid gap-4 lg:grid-cols-[16rem_minmax(0,1fr)]">
                <aside class="border border-slate-300 bg-white shadow-sm">
                    <div
                        class="flex items-center justify-between border-b border-slate-200 p-4"
                    >
                        <h2
                            class="text-xs font-semibold tracking-wider text-slate-600"
                        >
                            接口
                        </h2>
                        <button
                            class="text-sm font-semibold text-teal-700 hover:text-teal-900"
                            @click="createRequest"
                        >
                            + 新建
                        </button>
                    </div>
                    <div v-if="isLoading" class="p-4 text-sm text-slate-500">
                        加载中...
                    </div>
                    <div
                        v-else-if="!savedRequests.length"
                        class="p-4 text-sm leading-6 text-slate-500"
                    >
                        还没有保存的接口。
                    </div>
                    <div
                        v-else
                        class="max-h-[calc(100vh-10rem)] overflow-y-auto"
                    >
                        <button
                            v-for="request in savedRequests"
                            :key="request.id"
                            class="block w-full border-b border-slate-100 p-3 text-left hover:bg-teal-50"
                            :class="
                                request.id === currentId
                                    ? 'border-l-4 border-l-teal-700 bg-teal-50 pl-2'
                                    : ''
                            "
                            @click="selectRequest(request.id)"
                        >
                            <div class="flex items-center gap-2">
                                <span
                                    class="font-mono text-xs font-bold"
                                    :class="
                                        request.method === 'POST'
                                            ? 'text-orange-700'
                                            : 'text-teal-700'
                                    "
                                    >{{ request.method }}</span
                                ><span class="truncate text-sm font-medium">{{
                                    request.name
                                }}</span>
                            </div>
                            <div
                                class="mt-1 truncate font-mono text-xs text-slate-500"
                            >
                                {{ request.url || "未设置 URL" }}
                            </div>
                        </button>
                    </div>
                </aside>
                <section v-if="currentId !== null" class="min-w-0">
                    <div
                        class="mb-4 border border-slate-300 bg-white p-4 shadow-sm"
                    >
                        <label
                            class="mb-2 block text-xs font-semibold tracking-wider text-slate-600"
                            for="request-name"
                            >接口名称</label
                        ><input
                            id="request-name"
                            v-model="name"
                            class="h-10 w-full border border-slate-300 px-3 text-sm outline-none focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
                            placeholder="未命名请求"
                        />
                    </div>
                    <RequestEditor
                        v-model:method="method"
                        v-model:url="url"
                        v-model:body="body"
                        :headers="headers"
                        :is-sending="isSending"
                        @add-header="addHeader"
                        @remove-header="removeHeader"
                        @submit="sendRequest"
                    />
                    <p
                        v-if="error"
                        class="mt-4 border border-rose-200 bg-rose-50 px-4 py-3 text-sm text-rose-800"
                        role="alert"
                    >
                        {{ error }}
                    </p>
                    <ResponsePanel v-if="response" :response="response" />
                </section>
                <section
                    v-else
                    class="border border-dashed border-slate-300 bg-white p-8 text-center text-slate-500"
                >
                    <p class="text-sm">从左侧选择接口，或新建一个接口开始。</p>
                </section>
            </div>
        </div>
    </main>
</template>
