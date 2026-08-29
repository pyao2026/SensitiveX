<script setup>
defineProps({
    message: { type: String, default: "" },
});
</script>

<template>
    <p
        v-if="message"
        class="mt-4 border border-rose-200 bg-rose-50 px-4 py-3 text-sm text-rose-800"
        role="alert"
    >
        {{ message }}
    </p>
</template>
