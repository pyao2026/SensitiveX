import { createRouter, createWebHashHistory } from "vue-router";

export const router = createRouter({
    history: createWebHashHistory(),
    routes: [
        {
            path: "/",
            name: "projects",
            component: () => import("./pages/ProjectListPage.vue"),
        },
        {
            path: "/projects/:projectId",
            name: "api-console",
            component: () => import("./pages/ApiConsolePage.vue"),
            props: true,
        },
    ],
});
