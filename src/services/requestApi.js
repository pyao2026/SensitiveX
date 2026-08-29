import { invoke } from "@tauri-apps/api/core";

export function listSavedRequests() {
    return invoke("list_saved_requests");
}

export function getSavedRequest(id) {
    return invoke("get_saved_request", { id });
}

export function createSavedRequest() {
    return invoke("create_saved_request");
}

export function saveSavedRequest(input) {
    return invoke("save_saved_request", { input });
}

export function sendApiRequest(request) {
    return invoke("request_api", { request });
}
