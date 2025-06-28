<template>
    <h2>predicted symbols</h2>
    <ul>
        <template v-for="(ch, index) of content.row.trim()">
            <li v-if="ch === '1'">
                {{ names[index] }}
            </li>
        </template>
    </ul>
</template>

<script>
export default {
    props: ['content', 'names']
}
</script>

<style>
</style>
