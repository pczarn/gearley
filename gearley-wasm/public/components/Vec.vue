<template>
    <h1>medial items</h1>
    <ul>
        <template v-for="(item, index) of content">
            <li>
                (origin: {{ item.origin }}, dot: {{ item.dot }})
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
