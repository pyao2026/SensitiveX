import { invoke } from "@tauri-apps/api/core";

export function listProjects() {
    return invoke("list_projects");
}

export function createProject(input) {
    return invoke("create_project", { input });
}

export function updateProject(id, input) {
    return invoke("update_project", { id, input });
}

export function deleteProject(id) {
    return invoke("delete_project", { id });
}
