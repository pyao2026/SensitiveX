import { computed, onMounted, ref } from "vue";
import {
    createSavedRequest,
    deleteSavedRequest,
    getSavedRequest,
    listSavedRequests,
    saveSavedRequest,
    sendApiRequest,
} from "../services/requestApi";
import { validateRequest } from "../utils/validateRequest";

const emptyHeaders = () => [{ name: "", value: "" }];

export function useApiConsole(projectId) {
    const scopedProjectId = Number(projectId);
    const method = ref("GET");
    const url = ref("");
    const headers = ref(emptyHeaders());
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

    function loadRequest(request) {
        currentId.value = request.id;
        name.value = request.name;
        method.value = request.method;
        url.value = request.url;
        headers.value = request.headers?.length
            ? request.headers.map((header) => ({ ...header }))
            : emptyHeaders();
        body.value = request.body || "";
        savedSnapshot.value = currentSnapshot.value;
        response.value = null;
        error.value = "";
    }

    async function refreshList() {
        savedRequests.value = await listSavedRequests(scopedProjectId);
    }

    function confirmDiscard() {
        return (
            !isDirty.value ||
            window.confirm("当前接口有未保存修改，确定放弃这些修改吗？")
        );
    }

    async function selectRequest(id) {
        if (id === currentId.value || !confirmDiscard()) return;
        try {
            loadRequest(await getSavedRequest(scopedProjectId, id));
        } catch (requestError) {
            error.value = String(requestError);
        }
    }

    async function createRequest() {
        if (!confirmDiscard()) return;
        try {
            const request = await createSavedRequest(scopedProjectId);
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
            const request = await saveSavedRequest({
                projectId: scopedProjectId,
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
        const result = validateRequest({
            method: method.value,
            url: url.value,
            headers: headers.value,
            body: body.value,
        });
        if (result.error) {
            error.value = result.error;
            return;
        }

        isSending.value = true;
        try {
            response.value = await sendApiRequest(result.value);
        } catch (requestError) {
            error.value = String(requestError);
        } finally {
            isSending.value = false;
        }
    }

    async function removeRequest() {
        if (
            currentId.value === null ||
            !window.confirm("确定删除当前接口吗？")
        ) {
            return;
        }
        error.value = "";
        try {
            await deleteSavedRequest(scopedProjectId, currentId.value);
            await refreshList();
            if (savedRequests.value.length) {
                loadRequest(
                    await getSavedRequest(
                        scopedProjectId,
                        savedRequests.value[0].id,
                    ),
                );
            } else {
                currentId.value = null;
                savedSnapshot.value = "";
                response.value = null;
            }
        } catch (requestError) {
            error.value = String(requestError);
        }
    }

    onMounted(async () => {
        try {
            await refreshList();
            if (savedRequests.value.length) {
                loadRequest(
                    await getSavedRequest(
                        scopedProjectId,
                        savedRequests.value[0].id,
                    ),
                );
            }
        } catch (loadError) {
            error.value = String(loadError);
        } finally {
            isLoading.value = false;
        }
    });

    return {
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
    };
}
