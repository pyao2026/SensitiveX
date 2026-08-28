<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import RequestEditor from "./components/RequestEditor.vue";
import ResponsePanel from "./components/ResponsePanel.vue";

const method = ref("GET");
const url = ref("");
const headers = ref([{ name: "", value: "" }]);
const body = ref("{\n  \n}");
const isSending = ref(false);
const error = ref("");
const response = ref(null);

function addHeader() {
    headers.value.push({ name: "", value: "" });
}

function removeHeader(index) {
    headers.value.splice(index, 1);
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

    const requestBody = method.value === "POST" ? body.value.trim() : "";
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
</script>

<template>
    <main
        class="min-h-screen bg-slate-100 px-4 py-6 text-slate-900 sm:px-8 sm:py-10"
    >
        <div class="mx-auto max-w-6xl">
            <header
                class="mb-6 flex items-end justify-between border-b border-slate-300 pb-4"
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
                <p class="hidden text-sm text-slate-500 sm:block">
                    本地请求调试工具
                </p>
            </header>

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
        </div>
    </main>
</template>
