<script setup>
defineProps({
    message: String,
    own: Boolean
});
</script>

<template>
    <div class="message-wrapper" :class="{ own }">
        <div class="message">
            <p>{{ message }}</p>
        </div>
    </div>
</template>

<style scoped>
.message-wrapper {
    display: flex;
    width: 100%;
    padding: 0 10px;
}

.message-wrapper.own {
    justify-content: flex-end;
}

.message {
    max-width: 70%;
    padding: 8px 12px;
    margin: 4px 0;

    background-color: var(--theme-primary);
    color: var(--theme-black);

    border-radius: 12px;
    overflow-wrap: anywhere;
}

.message p {
    margin: 0;
}
</style>
