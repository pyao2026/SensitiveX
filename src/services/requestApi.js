import { invoke } from "@tauri-apps/api/core";

export function listSavedRequests(projectId) {
    return invoke("list_saved_requests", { projectId });
}

export function getSavedRequest(projectId, id) {
    return invoke("get_saved_request", { projectId, id });
}

export function createSavedRequest(projectId) {
    return invoke("create_saved_request", { projectId });
}

export function saveSavedRequest(input) {
    return invoke("save_saved_request", { input });
}

export function deleteSavedRequest(projectId, id) {
    return invoke("delete_saved_request", { projectId, id });
}

export function sendApiRequest(request) {
    return invoke("request_api", { request });
}
