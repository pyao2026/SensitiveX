<script setup>
import { onMounted, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import ConsoleHeader from "../components/ConsoleHeader.vue";
import EmptyRequestState from "../components/EmptyRequestState.vue";
import RequestSidebar from "../components/RequestSidebar.vue";
import RequestWorkspace from "../components/RequestWorkspace.vue";
import { useApiConsole } from "../composables/useApiConsole";
import { listProjects } from "../services/projectApi";

const route = useRoute();
const router = useRouter();
const projectId = Number(route.params.projectId);
const project = ref(null);
const projectError = ref("");
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
    removeRequest,
} = useApiConsole(projectId);

onMounted(async () => {
    try {
        project.value = (await listProjects()).find(
            (item) => item.id === projectId,
        );
        if (!project.value) router.replace("/");
    } catch (loadError) {
        projectError.value = String(loadError);
    }
});
</script>

<template>
    <section>
        <p
            v-if="projectError"
            class="mb-4 border border-red-200 bg-red-50 p-3 text-sm text-red-700"
        >
            {{ projectError }}
        </p>
        <p
            v-if="error && currentId === null"
            class="mb-4 border border-red-200 bg-red-50 p-3 text-sm text-red-700"
        >
            {{ error }}
        </p>
        <ConsoleHeader
            :project-name="project?.name || '项目'"
            :is-dirty="isDirty"
            :current-id="currentId"
            :is-saving="isSaving"
            @back="router.push('/')"
            @delete="removeRequest"
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
    </section>
</template>
