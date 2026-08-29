<script setup>
import ConsoleHeader from "./components/ConsoleHeader.vue";
import EmptyRequestState from "./components/EmptyRequestState.vue";
import RequestSidebar from "./components/RequestSidebar.vue";
import RequestWorkspace from "./components/RequestWorkspace.vue";
import { useApiConsole } from "./composables/useApiConsole";

const {
    method,
    url,
    headers,
    body,
    name,
    savedRequests,
    currentId,
    isLoading,
    isSaving,
    isSending,
    error,
    response,
    isDirty,
    selectRequest,
    createRequest,
    saveRequest,
    sendRequest,
} = useApiConsole();
</script>

<template>
    <main
        class="min-h-screen bg-slate-100 px-3 py-4 text-slate-900 sm:px-6 sm:py-6"
    >
        <div class="mx-auto max-w-7xl">
            <ConsoleHeader
                :is-dirty="isDirty"
                :current-id="currentId"
                :is-saving="isSaving"
                @save="saveRequest"
            />
            <div class="grid gap-4 lg:grid-cols-[16rem_minmax(0,1fr)]">
                <RequestSidebar
                    :requests="savedRequests"
                    :current-id="currentId"
                    :is-loading="isLoading"
                    @select="selectRequest"
                    @create="createRequest"
                />
                <RequestWorkspace
                    v-if="currentId !== null"
                    :name="name"
                    :method="method"
                    :url="url"
                    :headers="headers"
                    :body="body"
                    :is-sending="isSending"
                    :error="error"
                    :response="response"
                    @update:name="name = $event"
                    @update:method="method = $event"
                    @update:url="url = $event"
                    @update:headers="headers = $event"
                    @update:body="body = $event"
                    @submit="sendRequest"
                />
                <EmptyRequestState v-else />
            </div>
        </div>
    </main>
</template>
