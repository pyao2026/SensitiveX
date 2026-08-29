<script setup>
import { onMounted, reactive, ref } from "vue";
import { useRouter } from "vue-router";
import {
    createProject,
    deleteProject,
    listProjects,
    updateProject,
} from "../services/projectApi";

const router = useRouter();
const projects = ref([]);
const isLoading = ref(true);
const error = ref("");
const isSaving = ref(false);
const editingId = ref(null);
const form = reactive({ name: "", description: "" });

function resetForm() {
    editingId.value = null;
    form.name = "";
    form.description = "";
}

function beginEdit(project) {
    editingId.value = project.id;
    form.name = project.name;
    form.description = project.description || "";
    error.value = "";
}

async function refresh() {
    projects.value = await listProjects();
}

async function submit() {
    error.value = "";
    isSaving.value = true;
    try {
        const input = { name: form.name, description: form.description };
        if (editingId.value === null) {
            await createProject(input);
        } else {
            await updateProject(editingId.value, input);
        }
        resetForm();
        await refresh();
    } catch (saveError) {
        error.value = String(saveError);
    } finally {
        isSaving.value = false;
    }
}

async function remove(project) {
    if (!window.confirm(`确定删除项目“${project.name}”及其全部接口吗？`))
        return;
    error.value = "";
    try {
        await deleteProject(project.id);
        if (editingId.value === project.id) resetForm();
        await refresh();
    } catch (deleteError) {
        error.value = String(deleteError);
    }
}

onMounted(async () => {
    try {
        await refresh();
    } catch (loadError) {
        error.value = String(loadError);
    } finally {
        isLoading.value = false;
    }
});
</script>

<template>
    <section>
        <div
            class="mb-6 flex items-end justify-between border-b border-slate-300 pb-4"
        >
            <div>
                <p
                    class="mb-1 text-xs font-semibold tracking-[0.18em] text-teal-700"
                >
                    SENSITIVE X
                </p>
                <h1 class="text-2xl font-semibold tracking-wide">项目</h1>
            </div>
            <span class="text-sm text-slate-500"
                >{{ projects.length }} 个项目</span
            >
        </div>

        <form
            class="mb-6 border border-slate-300 bg-white p-4 shadow-sm"
            @submit.prevent="submit"
        >
            <h2 class="mb-4 text-sm font-semibold text-slate-700">
                {{ editingId === null ? "新建项目" : "编辑项目" }}
            </h2>
            <div
                class="grid gap-3 md:grid-cols-[minmax(0,1fr)_minmax(0,2fr)_auto_auto]"
            >
                <input
                    v-model="form.name"
                    required
                    class="h-10 border border-slate-300 px-3 text-sm outline-none focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
                    placeholder="项目名称"
                />
                <input
                    v-model="form.description"
                    class="h-10 border border-slate-300 px-3 text-sm outline-none focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
                    placeholder="描述（可选）"
                />
                <button
                    class="h-10 bg-teal-700 px-4 text-sm font-semibold text-white hover:bg-teal-800 disabled:bg-slate-400"
                    :disabled="isSaving"
                >
                    {{
                        isSaving
                            ? "保存中..."
                            : editingId === null
                              ? "创建"
                              : "保存"
                    }}
                </button>
                <button
                    v-if="editingId !== null"
                    type="button"
                    class="h-10 border border-slate-300 px-4 text-sm font-semibold text-slate-700 hover:bg-slate-50"
                    @click="resetForm"
                >
                    取消
                </button>
            </div>
        </form>

        <p
            v-if="error"
            class="mb-4 border border-red-200 bg-red-50 p-3 text-sm text-red-700"
        >
            {{ error }}
        </p>
        <div
            v-if="isLoading"
            class="border border-slate-300 bg-white p-8 text-center text-sm text-slate-500"
        >
            加载中...
        </div>
        <div
            v-else-if="!projects.length"
            class="border border-dashed border-slate-300 bg-white p-10 text-center text-sm text-slate-500"
        >
            还没有项目，请先创建一个项目。
        </div>
        <div v-else class="grid gap-3 md:grid-cols-2">
            <article
                v-for="project in projects"
                :key="project.id"
                class="border border-slate-300 bg-white p-4 shadow-sm"
            >
                <div class="flex items-start justify-between gap-4">
                    <button
                        class="min-w-0 flex-1 text-left"
                        @click="router.push(`/projects/${project.id}`)"
                    >
                        <h2
                            class="truncate text-lg font-semibold text-slate-900"
                        >
                            {{ project.name }}
                        </h2>
                        <p class="mt-1 min-h-6 text-sm text-slate-500">
                            {{ project.description || "暂无描述" }}
                        </p>
                    </button>
                    <div class="flex shrink-0 gap-2">
                        <button
                            class="text-sm font-semibold text-teal-700 hover:text-teal-900"
                            @click="beginEdit(project)"
                        >
                            编辑
                        </button>
                        <button
                            class="text-sm font-semibold text-red-700 hover:text-red-900"
                            @click="remove(project)"
                        >
                            删除
                        </button>
                    </div>
                </div>
                <button
                    class="mt-4 text-sm font-semibold text-teal-700 hover:text-teal-900"
                    @click="router.push(`/projects/${project.id}`)"
                >
                    进入 API 管理 →
                </button>
            </article>
        </div>
    </section>
</template>
