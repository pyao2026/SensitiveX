import { createRouter, createWebHashHistory } from "vue-router";
import ProjectListPage from "./pages/ProjectListPage.vue";
import ApiConsolePage from "./pages/ApiConsolePage.vue";

export const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        { path: "/", name: "projects", component: ProjectListPage },
        {
            path: "/projects/:projectId",
            name: "api-console",
            component: ApiConsolePage,
            props: true,
        },
    ],
});
