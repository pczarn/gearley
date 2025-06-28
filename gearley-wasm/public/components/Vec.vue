<template>
    <h2>medial items</h2>
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
